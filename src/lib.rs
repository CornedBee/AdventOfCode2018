use std::collections::HashMap;
use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;
use lazy_static::lazy_static;
use ndarray::{Array2, s};
use regex::Regex;

pub fn day1(input: &str) -> Result<(i32, i32), ParseIntError> {
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

struct Rectangle {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

struct Claim {
    _id: i32,
    area: Rectangle,
}

pub fn day3(input: &str) -> Result<(i32, i32), ParseIntError> {
    let claims = input.lines().map(parse_claim).collect::<Result<Vec<_>, _>>()?;

    let mut field = Array2::zeros((1000, 1000));

    for claim in &claims {
        mark_claim(&mut field, claim);
    }

    let part1 = field.iter().filter(|i| **i > 1).count() as i32;

    Ok((part1, 0))
}

fn parse_claim(s: &str) -> Result<Claim, ParseIntError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    }
    let parsed = RE.captures(s).unwrap();
    let left = i32::from_str(parsed.get(2).unwrap().as_str())?;
    let top = i32::from_str(parsed.get(3).unwrap().as_str())?;
    Ok(Claim {
        _id: i32::from_str(parsed.get(1).unwrap().as_str())?,
        area: Rectangle {
            left,
            top,
            right: left + i32::from_str(parsed.get(4).unwrap().as_str())?,
            bottom: top + i32::from_str(parsed.get(5).unwrap().as_str())?
        }
    })
}

fn mark_claim(field: &mut Array2<i32>, claim: &Claim) {
    let mut slice = field.slice_mut(s![claim.area.left..claim.area.right, claim.area.top..claim.area.bottom]);
    slice.map_inplace(|i| *i += 1);
}
