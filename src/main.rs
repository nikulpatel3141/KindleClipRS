use std::{
    collections::HashMap,
    fs::{self, File},
    str::FromStr,
};

#[macro_use]
extern crate lazy_static;

use askama::Template;
use chrono::NaiveDateTime;
use regex::Regex;
use strum_macros::{Display, EnumString};

// \s+Your Highlight (on page (?P<page>[0-9]+) \||at) location\s+([0-9]+-[0-9]+)\s+\|.*\,\s+(?P<dateStr>[a-zA-Z0-9 :]+)\n+(?P<data>.*)\n+

const CLIPPING_FILE: &str = "../My Clippings.txt";

const DATE_FORMAT: &str = "%d %B %Y %H:%M:%S";

const _QUOTE_DELIMITER_REGEX: &str = r"\n*\s*==========\s*\n*";
const _QUOTE_REGEX: &str = concat!(
    r"\n*(?P<title>.*)\s+\((?P<author>.*)\)\s*",
    r"\n*\s+-\s+Your (?P<clippingType>Highlight|Bookmark|Note) ",
    r"(on page (?P<page>[0-9]+) \||at)? ",
    r"location\s+(?P<locStart>[0-9]+)-(?P<locEnd>[0-9]+)\s+\|",
    r".*\,\s+(?P<dateStr>[a-zA-Z0-9 :]+)\s*[\r\n]+",
    r"(?P<quote>.*)\n*",
);

lazy_static! {
    static ref QUOTE_REGEX: Regex = Regex::new(_QUOTE_REGEX).unwrap();
    static ref QUOTE_DELIMITER_REGEX: Regex = Regex::new(_QUOTE_DELIMITER_REGEX).unwrap();
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

    let parsed_quote = Clipping {
        title: book_captures["title"].to_string(),
        author: book_captures["author"].to_string(),
        quote: book_captures["quote"].to_string(),
        clipping_type,
        location,
        page,
        added_date,
    };

    return Ok(parsed_quote);
}

fn parse_clippings(clipping_text: String) -> Vec<Clipping> {
    let quote_blocks = QUOTE_DELIMITER_REGEX.split(&clipping_text);

    quote_blocks
        .map(parse_quote_block)
        .filter_map(|x| x.ok())
        .collect()
}

fn write_parsed_clippings(parsed_clippings: Vec<Clipping>) ->  {
    let mut grouped_clippings: HashMap<(String, String), Vec<&Clipping>> = HashMap::new();

    for x in parsed_clippings.iter() {
        grouped_clippings
            .entry((x.author.to_owned(), x.title.to_owned()))
            .or_insert(vec![])
            .push(x)
    }

    for ((author, title), clippings) in grouped_clippings.iter() {
        let filename = author;
        let file_body = clippings
            .iter()
            .map(|x| x.render().unwrap())
            .collect::<Vec<String>>()
            .join("\n");

        let mut file = File::open(filename)?;
        file.
    }
    return Ok(());
}

fn main() {
    let contents = fs::read_to_string(CLIPPING_FILE).expect("Unable to read input file");

    parse_clippings(contents);

    // let quote_blocks = QUOTE_DELIMITER_REGEX.split(&contents);
    // let x: Vec<Result<Clipping>> = quote_blocks.map(|x| parse_quote_block(x)).collect();
}
