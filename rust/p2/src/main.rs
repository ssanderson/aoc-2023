use std::str::FromStr;

use regex::Regex;
use utils::{Part1, Part2, Result};

struct Problem2 {}

impl Part1 for Problem2 {
    const N: u8 = 2;
    type Input1 = Input;

    fn parse1(data: &str) -> Result<Self::Input1> {
        let parsed: Result<_> = data.lines().map(|line| line.parse::<Game>()).collect();
        parsed.map(|games| Input { games })
    }

    fn run1(input: Self::Input1) -> Result<String> {
        let possible_games = input
            .games
            .iter()
            .filter(|game| game.consistent_with(CubeSet { red: 12, green: 13, blue: 14 }));

        let output: u32 = possible_games.map(|g| g.id).sum();
        Ok(output.to_string())
    }
}

impl Part2 for Problem2 {
    type Input2 = Input;

    fn parse2(s: &str) -> Result<Self::Input2> {
        Self::parse1(s)
    }

    fn run2(input: Self::Input2) -> Result<String> {
        let sum: u32 = input.games.iter().map(|g| g.power()).sum();
        Ok(sum.to_string())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSet {
    pub fn consistent_with(&self, other: CubeSet) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u32,
    samples: Vec<CubeSet>,
}

impl Game {
    pub fn power(&self) -> u32 {
        let min = self
            .samples
            .iter()
            .copied()
            .reduce(|acc, next| CubeSet {
                red: std::cmp::max(acc.red, next.red),
                green: std::cmp::max(acc.green, next.green),
                blue: std::cmp::max(acc.blue, next.blue),
            })
            .expect("empty game");
        return min.blue * min.red * min.green;
    }

    pub fn consistent_with(&self, other: CubeSet) -> bool {
        self.samples.iter().all(move |s| s.consistent_with(other))
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        // Lines look like
        //
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        let [Some(game), Some(game_desc), None] = utils::take_fixed(s.split(":")) else {
            anyhow::bail!("Invalid game description: {s}");
        };

        let game_re = Regex::new("Game ([0-9]+)")?;
        let game_id: u32 = match game_re.captures(game) {
            Some(cap) => cap.get(1).unwrap().as_str().parse::<u32>()?,
            None => {
                anyhow::bail!("Invalid game: {game}");
            }
        };

        let dice_re = Regex::new("([0-9]+) (red|green|blue)")?;
        let samples: Result<Vec<CubeSet>> = game_desc
            .split("; ")
            .map(|draw| {
                let (mut red, mut blue, mut green) = (None, None, None);

                for die in draw.split(",") {
                    let Some(c) = dice_re.captures(die) else {
                        anyhow::bail!("Invalid dice sample: {die:?}")
                    };
                    let count = c.get(1).unwrap().as_str().parse::<u32>()?;
                    let target = match c.get(2).unwrap().as_str() {
                        "red" => &mut red,
                        "green" => &mut green,
                        "blue" => &mut blue,
                        &_ => panic!("wut"),
                    };

                    // Each color should appear at most once.
                    if target.is_some() {
                        anyhow::bail!("Invalid dice sample: {die:?}");
                    }
                    *target = Some(count);
                }

                Ok(CubeSet {
                    red: red.unwrap_or(0),
                    green: green.unwrap_or(0),
                    blue: blue.unwrap_or(0),
                })
            })
            .collect();

        Ok(Game { id: game_id, samples: samples? })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Input {
    games: Vec<Game>,
}

fn main() -> Result<()> {
    utils::run_part1::<Problem2>()?;
    utils::run_part2::<Problem2>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1_example() -> Result<()> {
        let s = r#"""Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"#;
        let parsed = Problem2::parse1(s)?;
        let expected = Input {
            games: vec![
                Game {
                    id: 1,
                    samples: vec![
                        CubeSet { blue: 3, red: 4, green: 0 },
                        CubeSet { red: 1, green: 2, blue: 6 },
                        CubeSet { red: 0, green: 2, blue: 0 },
                    ],
                },
                Game {
                    id: 2,
                    samples: vec![
                        CubeSet { blue: 1, green: 2, red: 0 },
                        CubeSet { green: 3, blue: 4, red: 1 },
                        CubeSet { green: 1, blue: 1, red: 0 },
                    ],
                },
                Game {
                    id: 3,
                    samples: vec![
                        CubeSet { green: 8, blue: 6, red: 20 },
                        CubeSet { blue: 5, red: 4, green: 13 },
                        CubeSet { green: 5, blue: 0, red: 1 },
                    ],
                },
            ],
        };

        assert_eq!(parsed.games[0], expected.games[0]);
        assert_eq!(parsed.games[1], expected.games[1]);
        assert_eq!(parsed.games[2], expected.games[2]);
        assert_eq!(parsed, expected);

        Ok(())
    }
}
