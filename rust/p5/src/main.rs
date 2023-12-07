mod input;
mod parser;

use rayon::prelude::*;

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

        let mut pairs = Vec::new();
        while let (Some(&start), Some(&len)) = (seeds.next(), seeds.next()) {
            pairs.push((start, len));
        }

        let best = pairs
            .into_iter()
            .map(|(start, len)| {
                (start..start + len)
                    .into_par_iter()
                    .map(|s| input.seed_location(s))
                    .reduce(|| std::u64::MAX, std::cmp::min)
            })
            .reduce(std::cmp::min)
            .unwrap();

        Ok(best.to_string())
    }
}

fn main() -> Result<()> {
    utils::run_part1::<Problem5>()?;
    utils::run_part2::<Problem5>()?;
    Ok(())
}
