#![feature(array_windows)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod error;

use day1::{d1_part1_solution, d1_part2_solution};
use day2::{d2_part1_solution, d2_part2_solution};
use day3::{d3_part1_solution, d3_part2_solution};
use day4::{d4_part1_solution, d4_part2_solution};
use day5::{d5_part1_solution, d5_part2_solution};
use day6::{d6_part1_solution, d6_part2_solution};
use day7::{d7_part1_solution, d7_part2_solution};
use error::Error;

fn main() -> Result<(), Error> {
    // let d1_part1 = d1_part1_solution("inputs/d1_input.txt")?;
    // println!("D1 part 1 solution: {}", d1_part1.0);
    // let d1_part2 = d1_part2_solution(d1_part1.1, d1_part1.2);
    // println!("D1 part 2 solution: {}", d1_part2);

    // let d2_part1 = d2_part1_solution("inputs/d2_input.txt")?;
    // println!("D2 part 1 solution: {}", d2_part1);
    // let d2_part2 = d2_part2_solution("inputs/d2_input.txt")?;
    // println!("D2 part 2 solution: {}", d2_part2);

    // let d3_part1 = d3_part1_solution("inputs/d3_input.txt")?;
    // println!("D3 part 1 solution: {}", d3_part1);
    // let d3_part2 = d3_part2_solution("inputs/d3_input.txt")?;
    // println!("D3 part 2 solution: {}", d3_part2);

    // let d4_part1 = d4_part1_solution("inputs/d4_input.txt")?;
    // println!("D4 part 1 solution: {}", d4_part1);
    // let d4_part2 = d4_part2_solution("inputs/d4_input.txt")?;
    // println!("D4 part 2 solution: {}", d4_part2);

    // let d5_part1 = d5_part1_solution("inputs/d5_input.txt")?;
    // println!("D5 part 1 solution: {}", d5_part1);
    // let d5_part2 = d5_part2_solution("inputs/d5_input.txt")?;
    // println!("D5 part 2 solution: {}", d5_part2);

    // let d6_part1 = d6_part1_solution("inputs/d6_input.txt")?;
    // println!("D6 part 1 solution: {}", d6_part1);
    // let d6_part2 = d6_part2_solution("inputs/d6_input.txt")?;
    // println!("D6 part 2 solution: {}", d6_part2);

    let d7_part1 = d7_part1_solution("inputs/d7_input.txt")?;
    println!("D7 part 1 solution: {}", d7_part1);
    // let d7_part2 = d7_part2_solution("inputs/d7_input.txt")?;
    // println!("D7 part 2 solution: {}", d7_part2);

    Ok(())
}
