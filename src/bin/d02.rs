#[macro_use]
extern crate lazy_static;

use core::fmt;
use std::error::Error;
use std::io;
use std::io::BufRead;
use std::num::ParseIntError;
use std::str::FromStr;

use regex::Regex;

#[derive(Debug)]
struct D2Entry {
    low_range: usize,
    high_range: usize,
    c: char,
    pass: String,
}

impl D2Entry {
    fn valid_part1(&self) -> bool {
        let char_count = self.pass.matches(self.c).count();
        char_count <= self.high_range && char_count >= self.low_range
    }
    fn valid_part2(&self) -> bool {
        (self.pass.chars().nth(self.low_range - 1).unwrap() == self.c)
            ^ (self.pass.chars().nth(self.high_range - 1).unwrap() == self.c)
    }
}

#[derive(Debug)]
enum ParseD2Error {
    RegexError,
    IntParseError(ParseIntError),
}

impl fmt::Display for ParseD2Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseD2Error::RegexError => write!(f, "Error in regex matching"),
            ParseD2Error::IntParseError(e) => write!(f, "{}", e),
        }
    }
}

impl Error for ParseD2Error {}

impl From<ParseIntError> for ParseD2Error {
    fn from(e: ParseIntError) -> Self {
        ParseD2Error::IntParseError(e)
    }
}

impl FromStr for D2Entry {
    type Err = ParseD2Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PARSE_RE: Regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
        }
        let cap = match PARSE_RE.captures(s) {
            Some(x) => x,
            None => return Err(ParseD2Error::RegexError),
        };

        let low_range = cap.get(1).unwrap().as_str().parse()?;
        let high_range = cap.get(2).unwrap().as_str().parse()?;
        let c = cap.get(3).unwrap().as_str().chars().next().unwrap();
        let pass = cap.get(4).unwrap().as_str().to_owned();
        Ok(D2Entry {
            low_range,
            high_range,
            c,
            pass,
        })
    }
}

fn main() -> Result<(), ParseD2Error> {
    let mut valid1_count = 0;
    let mut valid2_count = 0;

    for entry in io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().trim().parse::<D2Entry>())
    {
        let entry = entry?;
        if entry.valid_part1() {
            valid1_count += 1;
        }
        if entry.valid_part2() {
            valid2_count += 1;
        }
    }
    println!("{}", valid1_count);
    println!("{}", valid2_count);
    Ok(())
}
