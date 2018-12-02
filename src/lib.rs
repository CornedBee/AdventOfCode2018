use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

pub fn day1(input: &str) -> Result<(i32, i32), std::num::ParseIntError> {
    let numbers = input.lines().map(<i32 as FromStr>::from_str).collect::<Result<Vec<_>, _>>()?;

    let part1 = numbers.iter().sum();

    let mut seen = HashSet::new();
    let mut frequency = 0;
    for change in numbers.iter().cycle() {
        frequency += change;
        if !seen.insert(frequency) {
            return Ok((part1, frequency));
        }
    }

    unreachable!("cycle() is infinite");
}

pub fn day2(input: &str) -> (i32, i32) {
    let checksum_parts = input.lines().map(checksum_increments)
        .fold((0, 0), |(x1, x2), (y1, y2)| (x1 + y1, x2 + y2));

    (checksum_parts.0 * checksum_parts.1, 0)
}

fn checksum_increments(word: &str) -> (i32, i32) {
    let counts = letter_counts(word);
    let mut result = (0, 0);
    for (_, v) in counts {
        match v {
            2 => result.0 = 1,
            3 => result.1 = 1,
            _ => ()
        }
    }
    result
}

fn letter_counts(word: &str) -> HashMap<char, i32> {
    let mut result = HashMap::new();
    for c in word.chars() {
        let count = result.entry(c).or_insert(0);
        *count += 1;
    }
    result
}
