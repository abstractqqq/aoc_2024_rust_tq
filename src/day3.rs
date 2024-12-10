use crate::error::Error;
use regex::Regex;
use std::fs::File;
use std::io::Read;

// For this question, we sacrifice efficiency a little bit.
// Let's make things nice and easy

fn mul_and_sum(text:&str) -> Result<f64, Error> {
    // Not efficient because we are rebuilding the same regex 

    let re = Regex::new(r"mul\((\d+),(\d+)\)")
        .map_err(|e| Error::RegexError(e))?;
    
    let mut acc:f64 = 0.0; // use f64 as output to prevent overflow
    for (_, [x, y]) in re.captures_iter(text).map(|c| c.extract()) {
        // Only add if xx and yy both can be parsed properly
        if let (Ok(xx), Ok(yy)) = (x.parse::<i64>(), y.parse::<i64>()) {
            acc += (xx * yy) as f64;
        }
    }
    Ok(acc)
}

pub fn d3_part1_solution(input_path: &str) -> Result<f64, Error> {
    let mut f = File::open(input_path).map_err(|e| Error::IOError(e))?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).map_err(|e| Error::IOError(e))?;
    let text = buffer.to_lowercase(); // unnecessary
    mul_and_sum(text.as_str())
}

pub fn d3_part2_solution(input_path: &str) -> Result<f64, Error> {
    let mut f = File::open(input_path).map_err(|e| Error::IOError(e))?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).map_err(|e| Error::IOError(e))?;
    let text = buffer.to_lowercase(); // unnecessary

    let mut acc:f64 = 0.0;
    for part in text.split(r"do()") {
        // find first occurrence of "don't". Ignore everything after "don't"
        let value = match part.find(r"don't()") {
            Some(i) => {
                let (left, _) = part.split_at(i); // all valid UTF8 so no panic here.
                mul_and_sum(left)
            }
            // "don't" doesn't exist in part. Regular 
            , None => mul_and_sum(part)
        };

        acc += value.unwrap_or(0.0);

    }
    Ok(acc)
}
