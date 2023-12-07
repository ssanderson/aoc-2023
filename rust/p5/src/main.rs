mod input;
mod parser;

use anyhow::Context;
use utils::{Part1, Result};

use crate::input::Input;

struct Problem5 {}

impl Part1 for Problem5 {
    const N: u8 = 5;
    type Input = Input;

    fn parse(data: &str) -> Result<Self::Input> {
        Ok(data.parse()?)
    }

    fn run1(input: Self::Input) -> Result<String> {
        let result = input
            .seeds
            .iter()
            .copied()
            .map(|s| input.seed_location(s))
            .reduce(std::cmp::min)
            .context("no seeds")?;
        Ok(result.to_string())
    }
}

fn main() -> Result<()> {
    utils::run_part1::<Problem5>()?;
    Ok(())
}
