use anyhow::anyhow;
use regex::Regex;
use utils::{Part1, Part2, Result};

struct Problem1 {}

impl Part1 for Problem1 {
    const N: u8 = 1;
    type Input = Vec<String>;

    fn parse(data: &str) -> Result<Self::Input> {
        Ok(data.lines().map(ToOwned::to_owned).collect())
    }

    fn run1(input: Self::Input) -> Result<String> {
        let nums = input
            .iter()
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
            .collect::<Result<Vec<_>>>()?;

        Ok(nums.iter().sum::<u32>().to_string())
    }
}

impl Part2 for Problem1 {
    fn run2(input: Self::Input) -> Result<String> {
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

        let nums: Vec<u32> = input
            .iter()
            .map(|line| {
                let first = get_match(line, &match_first);
                let last = get_match(line, &match_last);
                match (first, last) {
                    (Some(x), Some(y)) => Ok(10 * x + y),
                    _ => Err(anyhow!("Invalid line: {line}")),
                }
            })
            .collect::<Result<_>>()?;

        Ok(nums.iter().sum::<u32>().to_string())
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
        let parsed = Problem1::parse(s)?;

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
        let parsed = Problem1::parse(s)?;
        assert_eq!(Problem1::run2(parsed)?, "281");

        Ok(())
    }
}
