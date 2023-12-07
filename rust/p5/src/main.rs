mod input;
mod parser;

use anyhow::Context;
use utils::{Part1, Part2, Result};

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

impl Part2 for Problem5 {
    fn run2(input: Self::Input) -> anyhow::Result<String> {
        // TODO: Non brute-force solution.
        let mut seeds = input.seeds.iter();
        let mut best = std::u64::MAX;
        while let (Some(&start), Some(&len)) = (seeds.next(), seeds.next()) {
            for seed in start..start + len {
                let loc = input.seed_location(seed);
                if loc < best {
                    best = loc;
                }
            }
        }

        Ok(best.to_string())
    }
}

fn main() -> Result<()> {
    utils::run_part1::<Problem5>()?;
    utils::run_part2::<Problem5>()?;
    Ok(())
}
