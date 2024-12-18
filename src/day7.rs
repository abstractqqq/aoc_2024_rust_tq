use crate::error::Error;
use std::fs::File;
use std::io::Read;

/// Recursively solve the row for part 1, with 2 operations.
fn solve_one_row(num:isize, numbers: &[isize]) -> bool {

    let mut solvable = false;
    if let Some(&a) = numbers.last() {
        let divisible = (num % a) == 0;
        let next_idx = numbers.len() - 1;
        if next_idx == 0 {
            solvable = num == a;
        } else if a <= num {
            // Addition
            solvable = solve_one_row(num - a, &numbers[..next_idx]);

            // Divisible
            if !solvable && divisible {
                solvable = solve_one_row(num / a,&numbers[..next_idx]);
            }
        }
    }
    solvable
}

/// Recursively solve the row for part 1, with 3 operations (+, * and ||)
fn solve_one_row_with_concat(num:isize, numbers: &[isize]) -> bool {

    let mut solvable = false;
    if let Some(&a) = numbers.last() {
        let divisible = (num % a) == 0;
        let next_idx = numbers.len() - 1;
        if next_idx == 0 {
            solvable = num == a;
        } else if a <= num {
            // Addition
            solvable = solve_one_row_with_concat(num - a, &numbers[..next_idx]);

            // Divisible
            if !solvable && divisible {
                solvable = solve_one_row_with_concat(num / a,&numbers[..next_idx]);
            }
            // Concat
            if !solvable {
                let a_str = a.to_string();
                let num_str = num.to_string();
                if num_str.ends_with(a_str.as_str()) {
                    // Always >= 0 because we can concat
                    let split_index = num_str.len() - a_str.len();
                    let (left, _) = num_str.split_at(split_index);
                    if let Ok(n) = left.parse::<isize>() {
                        solvable = solve_one_row_with_concat(n,&numbers[..next_idx]);
                    }
                }
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

pub fn d7_part2_solution(input_path: &str) -> Result<isize, Error> {

    let parsed_lines = parse_inputs(input_path)?;
    let mut sum = 0isize;
    for (num, numbers) in parsed_lines {
        let solvable= solve_one_row_with_concat(num, &numbers);
        // println!("{}: {:?}, Solvable: {}", num, numbers, solvable);
        if solvable {
            sum += num;
        }
    }
    Ok(sum)
}