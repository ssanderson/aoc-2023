use nom::character::complete::{digit1, multispace1};
use nom::combinator::map_res;
use nom::error::Error;
use nom::multi::separated_list1;
use nom::{Finish, IResult};
use std::str::FromStr;

/// Parse a number into an integer type.
pub fn num<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, str::parse)(input)
}

/// Parse a whitespace-delimited list of numbers.
pub fn whitespace_delimited_nums<T: FromStr>(input: &str) -> IResult<&str, Vec<T>> {
    separated_list1(multispace1, num::<T>)(input)
}

pub fn finalize<T>(result: IResult<&str, T>) -> anyhow::Result<T> {
    Ok(match result.finish() {
        Ok((_, result)) => Ok(result),
        Err(Error { input, code }) => Err(Error { input: input.to_string(), code }),
    }?)
}
