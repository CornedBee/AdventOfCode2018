const INPUT: &str = include_str!("../../inputs/day1");

fn main() -> Result<(), std::num::ParseIntError> {
    println!("{}", aoc2018::day1(INPUT)?);
    Ok(())
}
