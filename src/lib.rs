use std::str::FromStr;

pub fn day1(input: &str) -> Result<i32, std::num::ParseIntError> {
    input.lines()
        .map(<i32 as FromStr>::from_str)
        .try_fold(0i32, |acc, x| Ok(acc + x?))

}
