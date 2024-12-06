/// All Possible Error Types One May Encounter in AOC 2024.

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    ParseIntError(std::num::ParseIntError),
}
