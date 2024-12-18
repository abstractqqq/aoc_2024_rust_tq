use crate::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

/// Recursively solve the row
fn solve_one_row(num:isize, numbers: &[isize]) -> bool {

    let mut solvable = false;
    if let Some(&a) = numbers.last() {
        let divisible = (num % a) == 0;
        let second_to_last = numbers.len() - 1;
        // Addition
        if a <= num {
            if second_to_last == 0 {
                solvable = num == a;
            } else {
                // println!("+{}", a);
                solvable = solve_one_row(num - a, &numbers[..second_to_last]);
            }
        }
        // Divisible
        if !solvable && divisible {
            if second_to_last == 0 {
                solvable = num == a; // num == a. We got a solution
            } else {
                // println!("*{}", a);
                solvable = solve_one_row(num / a,&numbers[..second_to_last]);
            }
        }
    }

    solvable

}

pub fn parse_inputs(input_path: &str) -> Result<Vec<(isize, Vec<isize>)>, Error> {

    let mut f = File::open(input_path).map_err(|e| Error::IOError(e))?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).map_err(|e| Error::IOError(e))?;

    let mut outputs = Vec::new();

    for line in buffer.lines() {
        let mut split = line.split(':');
        if let (Some(left), Some(right)) = (split.next(), split.next()) {
            let a = left.trim().parse::<isize>();
            let b = right
                .trim()
                .split(' ')
                .filter_map(|v| v.parse::<isize>().ok())
                .collect::<Vec<_>>();

            // Line is correct
            if a.is_ok() && b.len() > 0 {
                outputs.push((a.unwrap(), b));
            } else {
                return Err(Error::Other("Input file is not correctly formatted.".into()))
            }
        }
    }

    Ok(outputs)

}

pub fn d7_part1_solution(input_path: &str) -> Result<isize, Error> {

    let parsed_lines = parse_inputs(input_path)?;
    let mut sum = 0isize;
    for (num, numbers) in parsed_lines {
        let solvable= solve_one_row(num, &numbers);
        if solvable {
            sum += num;
        }
    }
    Ok(sum)
}

pub fn d7_part2_solution(input_path: &str) -> Result<usize, Error> {
    todo!()
}