use std::fs;

#[macro_use]
extern crate lazy_static;

use chrono::NaiveDateTime;
use regex::Regex;

// \s+Your Highlight (on page (?P<page>[0-9]+) \||at) location\s+([0-9]+-[0-9]+)\s+\|.*\,\s+(?P<dateStr>[a-zA-Z0-9 :]+)\n+(?P<data>.*)\n+

const CLIPPING_FILE: &str = "../My Clippings.txt";

const _QUOTE_DELIMITER_REGEX: &str = r"\n*\s*==========\s*\n*";
const _QUOTE_REGEX: &str = concat!(
    r"\n+*(?P<title>.*)\((?P<author>.*)\)\s*",
    r"\n*\s+-\s+Your (?P<clippingType>Highlight|Bookmark|Note) ",
    r"(on page (?P<page>[0-9]+) \||at) location\s+([0-9]+-[0-9]+)\s+\|",
    r".*\,\s+(?P<dateStr>[a-zA-Z0-9 :]+)\s*\n+",
    r"(?P<data>.*)\n+",
);

lazy_static! {
    static ref QUOTE_REGEX: Regex = Regex::new(_QUOTE_REGEX).unwrap();
    static ref QUOTE_DELIMITER_REGEX: Regex = Regex::new(_QUOTE_DELIMITER_REGEX).unwrap();
}

struct Quote {
    title: String,
    author: String,
    location: (i32, i32),
    page: Option<i32>,
    added_date: NaiveDateTime,
}

fn parse_highlight_time(highlight_time_str: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(highlight_time_str, "%d %B %Y %H:%M:%S").unwrap()
}

fn parse_quote_block(quote_block: &str) -> Option<Quote> {
    let book_captures = match QUOTE_REGEX.captures(quote_block) {
        Some(x) => x,
        None => return None,
    };

    let highlight_time = parse_highlight_time(&book_captures["dateStr"]);

    println!(
        "{} {} {}",
        book_captures["title"].to_string(),
        book_captures["author"].to_string(),
        highlight_time,
    );

    // let highlight_time = parse_highlight_time(highlight_time_str);

    return None;
}

fn main() {
    let contents = fs::read_to_string(CLIPPING_FILE).expect("Unable to read input file");

    let quote_blocks = QUOTE_DELIMITER_REGEX.split(&contents);

    let x: Vec<Option<Quote>> = quote_blocks.map(|x| parse_quote_block(x)).collect();

    // let q = quote_blocks.next().unwrap();
}
