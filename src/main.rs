use std::{
    collections::HashMap,
    fs::{self, create_dir, File},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};

#[macro_use]
extern crate lazy_static;

use askama::Template;
use chrono::NaiveDateTime;
use clap::Parser;
use dialoguer::Confirm;
use env_logger::{self, Builder};
use log::{error, info, LevelFilter};
use regex::Regex;
use strum_macros::{Display, EnumString};

const CLIPPING_FILE: &str = "documents/My Clippings.txt";

const DATE_FORMAT: &str = "%d %B %Y %H:%M:%S";

const OUTPUT_DIRECTORY: &str = "clippings/";

const _QUOTE_DELIMITER_REGEX: &str = r"\n*\s*==========\s*\n*";
const _QUOTE_REGEX: &str = concat!(
    r"\n*(?P<title>.*)\s+\((?P<author>.*)\)\s*",
    r"\n*\s+-\s+Your (?P<clippingType>Highlight|Bookmark|Note)\s*",
    r"(on page (?P<page>[0-9]+) \||at)?\s*",
    r"(location\s+(?P<locStart>[0-9]+)-(?P<locEnd>[0-9]+)\s+\|)?",
    r".*\,\s+(?P<dateStr>[a-zA-Z0-9 :]+)\s*[\r\n]*",
    r"(?P<quote>.*)?\n*",
);

lazy_static! {
    static ref QUOTE_REGEX: Regex = Regex::new(_QUOTE_REGEX).unwrap();
    static ref QUOTE_DELIMITER_REGEX: Regex = Regex::new(_QUOTE_DELIMITER_REGEX).unwrap();
    static ref FILENAME_SANITISE_REGEX: Regex = Regex::new(r#"[/?<>\:*|\"]"#).unwrap();
}

#[derive(Debug, EnumString, Display, Clone, Copy)]
enum ClippingType {
    Bookmark,
    Highlight,
    Note,
}

#[derive(Debug, Template, Clone)]
#[template(path = "clipping_template.md")]
struct Clipping {
    title: String,
    author: String,
    clipping_type: ClippingType,
    location: Option<(i32, i32)>,
    page: Option<i32>,
    added_date: Option<NaiveDateTime>,
    quote: String,
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long)]
    file: Option<String>,
}

fn find_clipping_file() -> Option<PathBuf> {
    let find_kindle_mount_command = "/usr/bin/env";

    let shell_output = Command::new(find_kindle_mount_command)
        .arg("bash")
        .arg("-c")
        .arg("df | awk '{print $NF}' | tail -n +2 | grep Kindle")
        .output()
        .expect("Failed to find Kindle mount point");

    if !shell_output.status.success() {
        info!("Found no Kindles mounted on the system");
        return None;
    }

    let mount_points: Vec<String> = String::from_utf8_lossy(&shell_output.stdout)
        .trim()
        .split("\n")
        .map(|x| x.trim().to_string())
        .filter(|x| x.len() > 0)
        .collect();

    let mount_point = match mount_points.len() {
        1 => {
            info!("Found a Kindle mounted on {}", mount_points[0]);
            mount_points[0].clone()
        }
        _ => panic!("Found multiple mount points: {}", mount_points.join(", ")),
    };

    let clipping_file = Path::new(mount_point.as_str()).join(CLIPPING_FILE);

    assert!(
        clipping_file.is_file(),
        "Kindle mount found but {} is not a file",
        clipping_file.display()
    );

    Some(clipping_file)
}

fn parse_optional_int(capture_group: Option<regex::Match>) -> Option<i32> {
    match capture_group {
        Some(x) => x.as_str().parse::<i32>().ok(),
        None => None,
    }
}

fn parse_highlight_time(highlight_time_str: &str) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(highlight_time_str, DATE_FORMAT).ok()
}

fn parse_quote_block(quote_block: &str) -> Result<Clipping, ()> {
    let book_captures = QUOTE_REGEX.captures(quote_block).ok_or(())?;

    let clipping_type = match book_captures.name("clippingType") {
        Some(x) => match ClippingType::from_str(x.as_str()) {
            Ok(y) => y,
            Err(_) => return Err(()),
        },
        None => return Err(()),
    };

    let added_date = parse_highlight_time(&book_captures["dateStr"]);

    let page = parse_optional_int(book_captures.name("page"));

    let loc_start = parse_optional_int(book_captures.name("locStart"));
    let loc_end = parse_optional_int(book_captures.name("locEnd"));

    let location = if loc_start.is_some() && loc_end.is_some() {
        Some((loc_start.unwrap(), loc_end.unwrap()))
    } else {
        None
    };

    let quote = match book_captures.name("quote") {
        Some(x) => x.as_str().into(),
        None => String::new(),
    };

    let parsed_quote = Clipping {
        title: book_captures["title"].to_string(),
        author: book_captures["author"].to_string(),
        quote,
        clipping_type,
        location,
        page,
        added_date,
    };

    return Ok(parsed_quote);
}

fn parse_clippings(clipping_text: String) -> Vec<Clipping> {
    let quote_blocks: Vec<&str> = QUOTE_DELIMITER_REGEX.split(&clipping_text).collect();

    let num_clippings = quote_blocks.len() - 1;

    let parsed_quotes: Vec<Clipping> = quote_blocks
        .iter()
        .map(|x| parse_quote_block(*x))
        .filter_map(|x| x.ok())
        .collect();

    info!(
        "Parsed {} out of {} clippings",
        parsed_quotes.len(),
        num_clippings
    );

    parsed_quotes
}

fn sanitise_filename(filename: String) -> String {
    FILENAME_SANITISE_REGEX
        .replace_all(filename.as_str(), "")
        .replace("&", "and")
        .into()
}

fn write_parsed_clippings(parsed_clippings: Vec<Clipping>) -> Result<(), std::io::Error> {
    let mut grouped_clippings: HashMap<(String, String), Vec<&Clipping>> = HashMap::new();

    for x in parsed_clippings.iter() {
        grouped_clippings
            .entry((x.author.to_owned(), x.title.to_owned()))
            .or_insert(vec![])
            .push(x)
    }

    let output_directory = Path::new(OUTPUT_DIRECTORY);

    info!(
        "Attempting to write clippings to {}",
        output_directory.display()
    );

    if !output_directory.try_exists().unwrap() {
        info!(
            "Output directory {} doesn't exist, attempting to create it",
            output_directory.display()
        );
        create_dir(output_directory).unwrap();
        info!("Created output directory {}", output_directory.display());
    }

    for ((author, title), clippings) in grouped_clippings.iter() {
        let filename = sanitise_filename(format!(r"{} - {}.md", title, author).into());
        let file_path = output_directory.join(filename);

        info!(
            "Writing {:3} clippings for {}, {}",
            clippings.len(),
            title,
            author
        );

        let file_body = clippings
            .iter()
            .map(|x| x.render().unwrap())
            .collect::<Vec<String>>()
            .join("\n");

        let mut file = File::create(file_path)?;
        file.write(&file_body.into_bytes())?;
    }
    return Ok(());
}

fn confirm_found_clipping_file(clipping_file: PathBuf) -> Option<PathBuf> {
    let mut clipping_file_prompt = Confirm::new();

    clipping_file_prompt
        .with_prompt(format!(
            "Found clippings file {}, do you want to continue?",
            clipping_file.display()
        ))
        .default(true)
        .wait_for_newline(true);

    if clipping_file_prompt.interact().unwrap() {
        Some(clipping_file)
    } else {
        None
    }
}

fn main() -> Result<(), ()> {
    Builder::new().filter_level(LevelFilter::Info).init(); // TODO: improve logging format

    let cli = Cli::parse();

    let clipping_file = match cli.file {
        Some(x) => Path::new(x.as_str()).into(),
        None => {
            info!("Attempting to find a clipping file from any mounted Kindles");
            match find_clipping_file() {
                Some(x) => match confirm_found_clipping_file(x) {
                    Some(y) => y,
                    None => return Ok(()),
                },
                None => {
                    error!(
                        "No clippings files detected, mount a Kindle device \
                        or explicitly pass a clippings file to parse"
                    );
                    return Err(());
                }
            }
        }
    };

    info!(
        "Attempting to parse clippings file {}",
        clipping_file.display()
    );

    let contents = fs::read_to_string(clipping_file.clone())
        .unwrap_or_else(|_| panic!("Unable to read input file {}", clipping_file.display()));

    if contents.trim().len() == 0 {}

    let parsed_clippings = parse_clippings(contents);

    write_parsed_clippings(parsed_clippings).expect("Unable to write clippings");

    Ok(())
}
