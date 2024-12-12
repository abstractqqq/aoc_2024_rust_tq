use crate::error::Error;
use std::fs::File;
use std::io::Read;
use std::collections::{HashMap, HashSet};

fn parse_rules(rule_str: &str) -> HashMap<usize, HashSet<usize>> {

    let mut map = HashMap::new();
    for rule in rule_str.split("\n") {
        // Skip if | doesn't exist
        let mut numbers = rule.split('|');
        let first = numbers.next();
        let second = numbers.next();
        if let (Some(x), Some(y)) = (first, second) {
            if let (Ok(a), Ok(b)) = (x.parse::<usize>(), y.parse::<usize>()) {
                map
                    .entry(a)
                    .and_modify(|s: &mut HashSet<usize>| {s.insert(b);})
                    .or_insert(HashSet::from_iter([b]));
            }
        }
    }
    map
}

fn parse_update_lists(update_str: &str) -> Vec<Vec<usize>> {
    let mut output_lists = Vec::new();
    for update in update_str.split("\n") {
        if !update.trim().is_empty() {
            let update_list = update.trim().split(",").filter_map(|s| s.trim().parse::<usize>().ok()).collect::<Vec<_>>();
            output_lists.push(update_list);
        }
    }
    output_lists
}


pub fn d5_part1_solution(input_path: &str) -> Result<usize, Error> {
    let mut f = File::open(input_path).map_err(|e| Error::IOError(e))?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).map_err(|e| Error::IOError(e))?;

    let divide = "\n\n";
    let divide_index = buffer
        .find(divide)
        .ok_or_else(|| Error::Other("Input file is not correct.".to_string()))?;

    let (left, right) = buffer.split_at(divide_index);

    let rules = parse_rules(left);

    let mut answer = 0usize;

    for sequence in parse_update_lists(right) {
        let mut valid = true;
        for (i, num) in sequence.iter().enumerate() {
            // For every number to the right of num, they should not have an entry in the rules
            // that contains num
            // If they do, that means they should be to the left of num.
            for v in &sequence[i+1..] {
                if let Some(numbers) = rules.get(v) {
                    if numbers.contains(num) {
                        valid = false;
                        break;
                    }
                }
            }
            if !valid {
                break
            }
        }
        // The update sequence is valid. Find middle number
        if valid {
            let mid = sequence.len() / 2;
            answer += sequence[mid];
        }
    }
    Ok(answer)
}
    