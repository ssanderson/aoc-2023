use anyhow::anyhow;
use regex::Regex;
use utils::{Part1, Part2, Result};

struct Problem1 {}

impl Part1 for Problem1 {
    const N: u8 = 1;
    type Input1 = Vec<u32>;

    fn parse1(data: &str) -> Result<Self::Input1> {
        data.lines()
            .map(|line| {
                let nums: Vec<_> = line
                    .chars()
                    .filter_map(|x| match x {
                        '0'..='9' => Some((x as u8 - '0' as u8) as u32),
                        _ => None,
                    })
                    .collect();

                match (nums.first(), nums.last()) {
                    (Some(&x), Some(&y)) => Ok(10 * x + y),
                    _ => Err(anyhow!("Invalid line: {line}")),
                }
            })
            .collect()
    }

    fn run1(input: Self::Input1) -> Result<String> {
        Ok(input.iter().sum::<u32>().to_string())
    }
}

impl Part2 for Problem1 {
    type Input2 = Self::Input1;

    fn parse2(data: &str) -> Result<Self::Input1> {
        // regexes for getting the first and last occurrence of required
        // pattern.
        let match_first = Regex::new(r"([0-9]|one|two|three|four|five|six|seven|eight|nine).*$")?;
        let match_last = Regex::new(r"^.*([0-9]|one|two|three|four|five|six|seven|eight|nine)")?;

        let match2num = |s: &str| match s {
            "0" => 0,
            "1" | "one" => 1,
            "2" | "two" => 2,
            "3" | "three" => 3,
            "4" | "four" => 4,
            "5" | "five" => 5,
            "6" | "six" => 6,
            "7" | "seven" => 7,
            "8" | "eight" => 8,
            "9" | "nine" => 9,
            s => panic!("Failed to parse line: {s}"),
        };

        let get_match = |line, re: &Regex| {
            re.captures(line).map(|c| {
                // unwrap() is safe here b/c regex captures have at
                // least one capture grou.
                let capture = c.get(1).unwrap().as_str();
                match2num(capture)
            })
        };

        data.lines()
            .map(|line| {
                let first = get_match(line, &match_first);
                let last = get_match(line, &match_last);
                match (first, last) {
                    (Some(x), Some(y)) => Ok(10 * x + y),
                    _ => Err(anyhow!("Invalid line: {line}")),
                }
            })
            .collect()
    }

    fn run2(input: Self::Input2) -> Result<String> {
        Self::run1(input)
    }
}

fn main() -> Result<()> {
    utils::run_part1::<Problem1>()?;
    utils::run_part2::<Problem1>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1_example() -> Result<()> {
        let s = r#"""1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        let parsed = Problem1::parse1(s)?;

        assert_eq!(vec![12, 38, 15, 77], parsed);
        assert_eq!(Problem1::run1(parsed)?, "142");

        Ok(())
    }

    #[test]
    fn test_p2_example() -> Result<()> {
        let s = r#"""two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        let parsed = Problem1::parse2(s)?;

        assert_eq!(vec![29, 83, 13, 24, 42, 14, 76], parsed);
        assert_eq!(Problem1::run2(parsed)?, "281");

        Ok(())
    }
}
