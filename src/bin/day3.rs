use std::num::ParseIntError;

const INPUT: &str = include_str!("../../inputs/day3");

fn main() -> Result<(), ParseIntError> {
    let results = aoc2018::day3(INPUT)?;
    println!("{}", results.0);
    println!("{}", results.1.unwrap_or(0));
    Ok(())
}
