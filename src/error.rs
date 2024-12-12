/// All Possible Error Types One May Encounter in AOC 2024.
use regex;

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    ParseIntError(std::num::ParseIntError),
    RegexError(regex::Error),
    Other(String)
}
