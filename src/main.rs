use std::fs;

use chrono::NaiveDateTime;
use regex::Regex;

const CLIPPING_FILE: &str = "../My Clippings.txt";
const QUOTE_DELIMITER_REGEX: &str = r"\n*\s*==========\s*\n*";

// \s+Your Highlight (on page (?P<page>[0-9]+) \||at) location\s+([0-9]+-[0-9]+)\s+\|.*\,\s+(?P<dateStr>[a-zA-Z0-9 :]+)\n+(?P<data>.*)\n+

const QUOTE_REGEX: &str = concat!(
    r"\n+*(?P<title>.*)\((?P<author>.*)\)\s*",
    r"\n*\s+-\s+Your (?P<clippingType>Highlight|Bookmark|Note) ",
    r"(on page (?P<page>[0-9]+) \||at) location\s+([0-9]+-[0-9]+)\s+\|",
    r".*\,\s+(?P<dateStr>[a-zA-Z0-9 :]+)\s*\n+",
    r"(?P<data>.*)\n+",
);

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
    // let mut quote_block_lines = quote_block.lines();

    let book_regex = Regex::new(QUOTE_REGEX).unwrap();
    let book_captures = match book_regex.captures(quote_block) {
        Some(x) => x,
        None => return None,
    };

    println!(
        "{} {}",
        book_captures["title"].to_string(),
        book_captures["author"].to_string()
    );

    // let location_date_line = quote_block_lines.next()?;

    // let quote = &quote_block_lines.fold(String::new(), |a, b| a + b + "\n");

    // let highlight_time = parse_highlight_time(highlight_time_str);

    return None;
}

fn main() {
    println!("{}", QUOTE_REGEX);
    let contents = fs::read_to_string(CLIPPING_FILE).expect("Unable to read input file");

    let quote_re = Regex::new(QUOTE_DELIMITER_REGEX).unwrap();
    let quote_blocks = quote_re.split(&contents);

    let x: Vec<Option<Quote>> = quote_blocks.map(|x| parse_quote_block(x)).collect();

    // let q = quote_blocks.next().unwrap();
}
