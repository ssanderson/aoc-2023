use anyhow::{bail, Context};
use std::{collections::HashSet, str::FromStr};

use utils::{Part1, Result};

struct Problem4 {}

impl Part1 for Problem4 {
    const N: u8 = 4;
    type Input1 = Cards;

    fn parse1(data: &str) -> Result<Self::Input1> {
        let cards: Vec<Card> = data.lines().map(Card::from_str).collect::<Result<_>>()?;
        Ok(Cards { cards })
    }

    fn run1(cards: Cards) -> Result<String> {
        let result: u32 = cards.cards.iter().map(Card::point_value).sum();
        Ok(result.to_string())
    }
}

struct Cards {
    cards: Vec<Card>,
}

struct Card {
    winners: Vec<u8>,
    nums: Vec<u8>,
}

impl Card {
    pub fn point_value(&self) -> u32 {
        let winners: HashSet<_> = HashSet::from_iter(self.winners.iter().cloned());
        let match_count: u32 = self
            .nums
            .iter()
            .map(|n| if winners.contains(n) { 1 } else { 0 })
            .sum();

        if match_count == 0 {
            0
        } else {
            2u32.pow(match_count - 1)
        }
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let Some((_, nums)) = s.split_once(": ") else {
            bail!("invalid card {s:?}");
        };
        let Some((winners, nums)) = nums.split_once(" | ") else {
            bail!("invalid card {s:?}");
        };

        let winners = winners
            .trim()
            .split_ascii_whitespace()
            .map(u8::from_str)
            .collect::<std::result::Result<Vec<u8>, _>>()
            .with_context(|| s.to_owned())?;

        let nums = nums
            .trim()
            .split_ascii_whitespace()
            .map(u8::from_str)
            .collect::<std::result::Result<Vec<u8>, _>>()
            .with_context(|| s.to_owned())?;

        Ok(Card { winners, nums })
    }
}

fn main() -> Result<()> {
    utils::run_part1::<Problem4>()?;
    Ok(())
}
