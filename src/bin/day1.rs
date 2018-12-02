const INPUT: &str = include_str!("../../inputs/day1");

fn main() -> Result<(), std::num::ParseIntError> {
    let results = aoc2018::day1(INPUT)?;
    println!("{}", results.0);
    println!("{}", results.1);
    Ok(())
}
