use crate::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Checks the safety without dampener
fn check_safety(data: &[i32]) -> bool {
    if data.len() <= 1 {
        true
    } else {
        let mut all_pos = true;
        let mut all_neg = true;
        let mut within_bound = true;
        let mut answer = true;
        for sl in data.array_windows::<2>() {
            let diff = sl[1] - sl[0];
            all_pos &= diff > 0;
            all_neg &= diff < 0;
            within_bound &= diff.abs() >= 1 && (diff.abs() <= 3);
            answer = (all_pos | all_neg) & within_bound;
            if !answer {
                return false;
            }
        }
        answer
    }
}

/// Checks the safety wit dampener
fn check_safety_with_dampener(data: &[i32]) -> bool {
    // Kind of a brute-force approach, but I try my best to reduce copying data
    // The reason I don't use a linked-list is that...
    // Vec is faster for most of the tasks. Copying small vecs is very fast.
    if data.len() <= 1 {
        true
    } else {
        // Note: with the dampener, if data[1..] is safe in the og sense, then data is safe
        // in the dampener sense. All data that is safe in the og sense will also be captured
        // here in this first branch
        if check_safety(&data[1..]) {
            return true;
        } else {
            // Now check what happens if we remove the j-th number in the og data.
            for j in 1..data.len() {
                let mut new_data = vec![0i32; data.len() - 1];
                for i in 0..j {
                    new_data[i] = data[i];
                }
                for i in j..new_data.len() {
                    new_data[i] = data[i + 1];
                }
                if check_safety(&new_data) {
                    return true;
                }
            }
            // Finished for loop without returning, false
            false
        }
    }
}

/// Parses the line, and returns a ParseError if get an error during parsing.
fn parse_line(line: String) -> Result<Vec<i32>, Error> {
    let mut numbers = Vec::new();
    for v in line.split(' ') {
        match v.parse::<i32>() {
            Ok(i) => numbers.push(i),
            Err(e) => return Err(Error::ParseIntError(e)),
        }
    }
    Ok(numbers)
}

pub fn d2_part1_solution(input_path: &str) -> Result<usize, Error> {
    let f = File::open(input_path).map_err(|e| Error::IOError(e))?;
    let reader = BufReader::new(f);

    let mut count: usize = 0;
    for line in reader.lines().filter_map(|line| line.ok()) {
        // Split line into Vec of numbers. Only use lines we parse successfully
        if let Ok(v) = parse_line(line) {
            count += check_safety(&v) as usize;
        }
    }

    Ok(count)
}

pub fn d2_part2_solution(input_path: &str) -> Result<usize, Error> {
    let f = File::open(input_path).map_err(|e| Error::IOError(e))?;
    let reader = BufReader::new(f);

    let mut count: usize = 0;
    for line in reader.lines().filter_map(|line| line.ok()) {
        // Split line into Vec of numbers. Only use lines we parse successfully
        if let Ok(v) = parse_line(line) {
            count += check_safety_with_dampener(&v) as usize;
        }
    }

    Ok(count)
}
