use std::fs;

use chrono::NaiveDateTime;

const CLIPPING_FILE: &str = "../My Clippings.txt";

struct Quote {
    title: String,
    author: String,
    location: (i32, i32),
    page: Option<i32>,
    added_date: NaiveDateTime,
}

fn main() {
    let contents = fs::read_to_string(CLIPPING_FILE).expect("Unable to read input file");

    println!("{}", contents);
}
