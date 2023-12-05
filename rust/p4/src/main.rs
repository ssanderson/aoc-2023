use anyhow::{bail, Context};
use std::str::FromStr;

use utils::{Part1, Part2, Result};

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

impl Part2 for Problem4 {
    type Input2 = Cards;

    fn parse2(data: &str) -> anyhow::Result<Self::Input2> {
        Self::parse1(data)
    }

    fn run2(cards: Cards) -> Result<String> {
        let mut copies: Vec<u32> = vec![1; cards.cards.len()];
        for (i, card) in cards.cards.iter().enumerate() {
            for j in 0..card.match_count() {
                copies[i + j as usize + 1] += copies[i];
            }
        }

        let result: u32 = copies.iter().sum();
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
    pub fn match_count(&self) -> u32 {
        self.nums
            .iter()
            .filter(|n| self.winners.contains(n))
            .count() as u32
    }

    pub fn point_value(&self) -> u32 {
        let match_count = self.match_count();
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
            .collect::<std::result::Result<Vec<u8>, _>>()?;

        let nums = nums
            .trim()
            .split_ascii_whitespace()
            .map(u8::from_str)
            .collect::<std::result::Result<Vec<u8>, _>>()?;

        Ok(Card { winners, nums })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p2() -> Result<()> {
        let input = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#
            .trim();
        let parsed = Problem4::parse2(input)?;
        let result = Problem4::run2(parsed)?;
        assert_eq!(result, "30");

        Ok(())
    }
}

fn main() -> Result<()> {
    utils::run_part1::<Problem4>()?;
    utils::run_part2::<Problem4>()?;
    Ok(())
}
