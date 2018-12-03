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

pub fn day2(input: &str) -> (i32, String) {
    let lines = input.lines().collect::<Vec<_>>();
    let checksum_parts = lines.iter().cloned().map(checksum_increments)
        .fold((0, 0), |(x1, x2), (y1, y2)| (x1 + y1, x2 + y2));

    let mut part2 = "".to_owned();
    for (i, s1) in lines.iter().enumerate() {
        for s2 in &lines[(i+1)..] {
            if are_similar(s1, s2) {
                part2 = common_subset(s1, s2);
            }
        }
    }

    (checksum_parts.0 * checksum_parts.1, part2)
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

fn are_similar(s1: &str, s2: &str) -> bool {
    let mut diffs = s1.chars().zip(s2.chars()).filter(|(c1, c2)| c1 != c2);
    diffs.next().is_some() && diffs.next().is_none()
}

#[test]
fn are_similar_for_similar_strings() {
    assert!(are_similar("abcde", "abxde"));
}

fn common_subset(s1: &str, s2: &str) -> String {
    s1.chars().zip(s2.chars()).filter(|(c1, c2)| c1 == c2).map(|(c1, _)| c1).collect()
}
