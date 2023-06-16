use core::panic;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};

#[macro_use]
extern crate lazy_static;

use askama::Template;
use chrono::NaiveDateTime;
use env_logger::{self, Builder};
use log::{info, LevelFilter};
use regex::Regex;
use strum_macros::{Display, EnumString};

// \s+Your Highlight (on page (?P<page>[0-9]+) \||at) location\s+([0-9]+-[0-9]+)\s+\|.*\,\s+(?P<dateStr>[a-zA-Z0-9 :]+)\n+(?P<data>.*)\n+

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
#[template(path = "quote_template.md")]
struct Clipping {
    title: String,
    author: String,
    clipping_type: ClippingType,
    location: Option<(i32, i32)>,
    page: Option<i32>,
    added_date: Option<NaiveDateTime>,
    quote: String,
}

fn find_clipping_file() -> PathBuf {
    let find_kindle_mount_command = "/usr/bin/env";

    let shell_output = Command::new(find_kindle_mount_command)
        .arg("bash")
        .arg("-c")
        .arg("cat /proc/mounts | awk '{print $2}' | grep Kindle")
        .output()
        .expect("Failed to find Kindle mount point");

    assert!(shell_output.status.success());

    let mount_points: Vec<String> = String::from_utf8_lossy(&shell_output.stdout)
        .split("\n")
        .map(|x| x.trim().to_string())
        .filter(|x| x.len() > 0)
        .collect();

    println!("{}, {}", mount_points.clone().join(" "), mount_points.len());

    let mount_point = match mount_points.len() {
        0 => panic!("Found no Kindles mounted on the system"),
        1 => {
            info!("Found a Kindle mounted on {}", mount_points[0]);
            mount_points[0].clone()
        }
        _ => panic!("Found multiple mount points: {}", mount_points.join(", ")),
    };

    let clipping_file = Path::new(mount_point.as_str()).join(CLIPPING_FILE);

    assert!(clipping_file.is_file());
    info!("Found clippings file {}", clipping_file.display());

    clipping_file
}

fn parse_command_line_args() -> () {
    // Need option to pass custom clippings file, otherwise use find_clipping_file
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

    let num_clippings = quote_blocks.len();

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

fn main() {
    Builder::new().filter_level(LevelFilter::Info).init(); // FIXME: improve logging format

    let clipping_file = find_clipping_file();

    let contents = fs::read_to_string(clipping_file).expect("Unable to read input file");

    let parsed_clippings = parse_clippings(contents);

    write_parsed_clippings(parsed_clippings).expect("Unable to write clippings");
}
