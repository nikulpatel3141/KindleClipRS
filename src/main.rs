use std::fs;

#[macro_use]
extern crate lazy_static;

use chrono::NaiveDateTime;
use regex::Regex;

// \s+Your Highlight (on page (?P<page>[0-9]+) \||at) location\s+([0-9]+-[0-9]+)\s+\|.*\,\s+(?P<dateStr>[a-zA-Z0-9 :]+)\n+(?P<data>.*)\n+

const CLIPPING_FILE: &str = "../My Clippings.txt";

const DATE_FORMAT: &str = "%d %B %Y %H:%M:%S";

const _QUOTE_DELIMITER_REGEX: &str = r"\n*\s*==========\s*\n*";
const _QUOTE_REGEX: &str = concat!(
    r"\n*(?P<title>.*)\s+\((?P<author>.*)\)\s*",
    r"\n*\s+-\s+Your (?P<clippingType>Highlight|Bookmark|Note) ",
    r"(on page (?P<page>[0-9]+) \||at)? ",
    r"location\s+(?P<locStart>[0-9]+)-(?P<locEnd>[0-9]+)\s+\|",
    r".*\,\s+(?P<dateStr>[a-zA-Z0-9 :]+)\s*[\r\n]*",
    r"(?P<quote>.*)\n*",
);

lazy_static! {
    static ref QUOTE_REGEX: Regex = Regex::new(_QUOTE_REGEX).unwrap();
    static ref QUOTE_DELIMITER_REGEX: Regex = Regex::new(_QUOTE_DELIMITER_REGEX).unwrap();
}

#[derive(Debug)]
struct Quote<'a> {
    title: &'a str,
    author: &'a str,
    clipping_type: &'a str,
    location: Option<(i32, i32)>,
    page: Option<i32>,
    added_date: Option<NaiveDateTime>,
    quote: &'a str,
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

fn parse_quote_block(quote_block: &str) -> Option<Quote> {
    let book_captures = match QUOTE_REGEX.captures(quote_block) {
        Some(x) => x,
        None => return None,
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

    let parsed_quote = Quote {
        title: &book_captures["title"],
        author: &book_captures["author"],
        quote: &book_captures["quote"],
        clipping_type: &book_captures["clippingType"],
        location,
        page,
        added_date,
    };

    println!("{:?}", parsed_quote);

    return None;
}

fn main() {
    let contents = fs::read_to_string(CLIPPING_FILE).expect("Unable to read input file");

    let quote_blocks = QUOTE_DELIMITER_REGEX.split(&contents);

    let x: Vec<Option<Quote>> = quote_blocks.map(|x| parse_quote_block(x)).collect();
}
