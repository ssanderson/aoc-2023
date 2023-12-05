use std::str::FromStr;

use utils::{Part1, Part2, Result};

struct Problem3 {}

impl Part1 for Problem3 {
    const N: u8 = 3;
    type Input = EngineDiagram;

    fn parse(data: &str) -> Result<Self::Input> {
        let lines: Result<Vec<_>> = data.lines().map(|s| s.parse::<Line>()).collect();
        Ok(EngineDiagram { lines: lines? })
    }

    fn run1(input: EngineDiagram) -> Result<String> {
        let parts = input.get_parts();
        let total: u32 = parts.iter().map(|num| num.n).sum();
        Ok(total.to_string())
    }
}

impl Part2 for Problem3 {
    fn run2(data: Self::Input) -> Result<String> {
        let gears = data.get_gears();
        let result: u32 = gears.iter().map(Gear::ratio).sum();
        Ok(result.to_string())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct EngineDiagram {
    lines: Vec<Line>,
}

impl EngineDiagram {
    pub fn iter_adjacent_lines(
        &self,
    ) -> impl Iterator<Item = (Option<&Line>, &Line, Option<&Line>)> {
        self.lines.iter().enumerate().map(|(i, line)| {
            let prev = self.lines.get(i.wrapping_sub(1));
            let next = self.lines.get(i + 1);
            (prev, line, next)
        })
    }

    pub fn get_parts(&self) -> Vec<&Num> {
        self.iter_adjacent_lines()
            .flat_map(|(prev, cur, next)| cur.parts(prev, next))
            .collect()
    }

    pub fn get_gears(&self) -> Vec<Gear<'_>> {
        self.iter_adjacent_lines()
            .flat_map(|(prev, cur, next)| cur.gears(prev, next))
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Line {
    symbols: Vec<Sym>,
    numbers: Vec<Num>,
}

impl Line {
    pub fn parts<'a>(
        &'a self,
        prev: Option<&'a Line>,
        next: Option<&'a Line>,
    ) -> impl Iterator<Item = &'a Num> {
        let neighbors = [prev, Some(self), next];
        self.numbers.iter().filter(move |num| {
            neighbors
                .iter()
                .filter_map(|&x| x)
                .flat_map(|line| line.symbols.iter())
                .any(|sym| sym.adjacent_to(num))
        })
    }

    pub fn gears<'a>(
        &'a self,
        prev: Option<&'a Line>,
        next: Option<&'a Line>,
    ) -> impl Iterator<Item = Gear<'a>> + 'a {
        let neighbors = [prev, Some(self), next];
        self.symbols.iter().filter_map(move |sym| {
            let adjacent_nums = neighbors
                .iter()
                .filter_map(|&x| x)
                .flat_map(|line| line.numbers.iter().filter(|num| sym.adjacent_to(num)));

            let [Some(num1), Some(num2), None] = utils::take_fixed(adjacent_nums) else {
                return None;
            };

            Some(Gear { sym, num1, num2 })
        })
    }
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

        // finalize last number
        if let Some(num) = state {
            numbers.push(num);
        }

        Ok(Line { symbols, numbers })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Sym {
    pos: usize,
    c: char,
}

impl Sym {
    pub fn adjacent_to(&self, num: &Num) -> bool {
        (num.start <= self.pos + 1) && (self.pos <= num.end + 1)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Gear<'a> {
    sym: &'a Sym,
    num1: &'a Num,
    num2: &'a Num,
}

impl Gear<'_> {
    pub fn ratio(&self) -> u32 {
        self.num1.n * self.num2.n
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Num {
    start: usize,
    end: usize,
    n: u32,
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

        let parsed = Problem3::parse(input.trim())?;
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
    fn test_run1() -> Result<()> {
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

        let parsed = Problem3::parse(input.trim())?;
        let result = Problem3::run1(parsed)?;
        assert_eq!(result, "4361");

        Ok(())
    }

    #[test]
    fn test_run2() -> Result<()> {
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

        let parsed = Problem3::parse(input.trim())?;
        let result = Problem3::run2(parsed)?;
        assert_eq!(result, "467835");

        Ok(())
    }
}

fn main() -> Result<()> {
    utils::run_part1::<Problem3>()?;
    utils::run_part2::<Problem3>()?;
    Ok(())
}
