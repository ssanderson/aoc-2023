use std::str::FromStr;

use utils::{Part1, Result};

struct Problem3 {}

impl Part1 for Problem3 {
    const N: u8 = 3;
    type Input1 = EngineDiagram;

    fn parse1(data: &str) -> Result<Self::Input1> {
        let lines: Result<Vec<_>> = data.lines().map(|s| s.parse::<Line>()).collect();
        Ok(EngineDiagram { lines: lines? })
    }

    fn run1(input: EngineDiagram) -> Result<String> {
        let parts = input.get_parts();
        let total: u32 = parts.iter().map(|num| num.n).sum();
        Ok(total.to_string())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct EngineDiagram {
    lines: Vec<Line>,
}

impl EngineDiagram {
    pub fn get_parts(&self) -> Vec<&Num> {
        self.lines
            .iter()
            .enumerate()
            .flat_map(|(i, line)| {
                let adjacent_lines = [
                    self.lines.get(i.wrapping_sub(1)),
                    Some(line),
                    self.lines.get(i + 1),
                ];
                line.numbers.iter().filter(move |num| {
                    let is_good = adjacent_lines
                        .iter()
                        .filter_map(|&x| x)
                        .flat_map(|line| line.symbols.iter())
                        .any(|sym| (num.start <= sym.pos + 1) && (sym.pos <= num.end + 1));
                    if !is_good {
                        println!("Filtered out line={i} n={num:?}");
                    }
                    is_good
                })
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Line {
    symbols: Vec<Sym>,
    numbers: Vec<Num>,
}

impl FromStr for Line {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut symbols = vec![];
        let mut numbers = vec![];

        let mut state: Option<Num> = None;

        for (i, c) in s.chars().enumerate() {
            state = match c {
                '0'..='9' => {
                    let digit = (c as u8 - '0' as u8) as u32;
                    Some(match state {
                        Some(num) => Num {
                            n: 10 * num.n + digit,
                            start: num.start,
                            end: i,
                        },
                        None => Num { n: digit, start: i, end: i },
                    })
                }
                _ => {
                    if let Some(num) = state {
                        numbers.push(num);
                    }
                    if c != '.' {
                        symbols.push(Sym { pos: i, c });
                    }
                    None
                }
            };
        }

        if let Some(num) = state {
            numbers.push(num);
        }

        Ok(Line { symbols, numbers })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Sym {
    pos: usize,
    c: char,
}

#[derive(Debug, PartialEq, Eq)]
struct Num {
    start: usize,
    end: usize,
    n: u32,
}

fn main() -> Result<()> {
    utils::run_part1::<Problem3>()?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() -> Result<()> {
        let input = r#"
467..114..
...*......
..35..633.
......#...
617*......"#;

        let parsed = Problem3::parse1(input.trim())?;
        let expected = EngineDiagram {
            lines: vec![
                Line {
                    symbols: vec![],
                    numbers: vec![
                        Num { start: 0, end: 2, n: 467 },
                        Num { start: 5, end: 7, n: 114 },
                    ],
                },
                Line {
                    symbols: vec![Sym { pos: 3, c: '*' }],
                    numbers: vec![],
                },
                Line {
                    symbols: vec![],
                    numbers: vec![
                        Num { start: 2, end: 3, n: 35 },
                        Num { start: 6, end: 8, n: 633 },
                    ],
                },
                Line {
                    symbols: vec![Sym { pos: 6, c: '#' }],
                    numbers: vec![],
                },
                Line {
                    symbols: vec![Sym { pos: 3, c: '*' }],
                    numbers: vec![Num { start: 0, end: 2, n: 617 }],
                },
            ],
        };

        for (i, (l1, l2)) in parsed.lines.iter().zip(expected.lines.iter()).enumerate() {
            assert_eq!(l1, l2, "line {i}");
        }
        assert_eq!(parsed, expected);

        let parts = parsed.get_parts();
        let expected = vec![
            &Num { start: 0, end: 2, n: 467 },
            &Num { start: 2, end: 3, n: 35 },
            &Num { start: 6, end: 8, n: 633 },
            &Num { start: 0, end: 2, n: 617 },
        ];
        assert_eq!(parts, expected);

        Ok(())
    }

    #[test]
    fn test_run() -> Result<()> {
        let input = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#
            .trim();

        let parsed = Problem3::parse1(input.trim())?;
        let result = Problem3::run1(parsed)?;
        assert_eq!(result, "4361");

        Ok(())
    }
}
