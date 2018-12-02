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
