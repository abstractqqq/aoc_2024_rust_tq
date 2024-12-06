use crate::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Inputs the text file given by AOC and finds the solution to day 1 part 1.
/// Outputs the (total_distance, sorted first column, sorted second column)
pub fn d1_part1_solution(input_path: &str) -> Result<(usize, Vec<usize>, Vec<usize>), Error> {
    let f = File::open(input_path).map_err(|e| Error::IOError(e))?;
    let reader = BufReader::new(f);

    let mut c1 = Vec::new();
    let mut c2 = Vec::new();
    for line in reader.lines().filter_map(|line| line.ok()) {
        // Skip the line if the line parsing fails
        let mut s = line.split("   "); // 3 spaces
                                       // If parsing for either number fails, skip this line
        let first = s.next();
        let second = s.next();
        let s1 = first.map(|s| s.parse::<usize>());
        let s2 = second.map(|s| s.parse::<usize>());
        // Only push if both are good
        if let (Some(Ok(id1)), Some(Ok(id2))) = (s1, s2) {
            c1.push(id1);
            c2.push(id2);
        }
    }
    // sort c1 and c2
    c1.sort_unstable();
    c2.sort_unstable();

    let total_distance = c1
        .iter()
        .copied()
        .zip(c2.iter().copied())
        .fold(0usize, |acc, (x, y)| acc + (x.abs_diff(y)));

    Ok((total_distance, c1, c2))
}

/// Computes the similarity score as described in day 1 part 2 AOC 2024.
pub fn d1_part2_solution(c1: Vec<usize>, c2: Vec<usize>) -> usize {
    // c1 and c2 are sorted
    let mut last_idx: usize = 0;
    // Calculate similarity score
    c1.into_iter().fold(0usize, |acc, id| {
        let sl = &c2[last_idx..];
        // j = first place in sl where we have an element > id;
        let j = sl.partition_point(|i| *i <= id);
        // A side effect here, because the next time, we can search starting from last_idx + j.
        last_idx += j;
        // This implies all elements in left are <= id
        let left = &sl[..j];
        // Find the number of matches to id in left. Notice we can short-circuit if the
        // last element in left is < id. If left has 0 size, we can also short-circuit.
        if left.last().map(|l| *l < id).unwrap_or(true) {
            acc // return acc, because we are adding 0
        } else {
            // The number of matches in left * id
            acc + left.iter().rev().take_while(|&k| *k == id).count() * id
        }
    })
}
