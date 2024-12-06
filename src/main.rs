#![feature(array_windows)]

mod day1;
mod day2;
mod error;

use day1::{d1_part1_solution, d1_part2_solution};
use day2::{d2_part1_solution, d2_part2_solution};
use error::Error;

fn main() -> Result<(), Error> {
    let d1_part1 = d1_part1_solution("inputs/d1_input.txt")?;
    println!("D1 part 1 solution: {}", d1_part1.0);
    let d1_part2 = d1_part2_solution(d1_part1.1, d1_part1.2);
    println!("D1 part 2 solution: {}", d1_part2);

    let d2_part1 = d2_part1_solution("inputs/d2_input.txt")?;
    println!("D2 part 1 solution: {}", d2_part1);
    let d2_part2 = d2_part2_solution("inputs/d2_input.txt")?;
    println!("D2 part 2 solution: {}", d2_part2);

    Ok(())
}
