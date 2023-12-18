use utils::{Part1, Part2, Result};

struct Problem6;

impl Part1 for Problem6 {
    const N: u8 = 6;
    type Input = Input;

    fn parse(data: &str) -> Result<Self::Input> {
        utils::parse::finalize(parser::parse_input(data.trim()))
    }

    fn run1(input: Self::Input) -> Result<String> {
        let counts = input.races.iter().map(Race::record_count);
        let prod = counts.fold(1, |a, b| a * b);
        Ok(prod.to_string())
    }
}

impl Part2 for Problem6 {
    fn run2(input: Self::Input) -> anyhow::Result<String> {
        let race = input
            .races
            .iter()
            .copied()
            .reduce(|a, b| a.concat_fields(b))
            .expect("should be multiple races");

        Ok(race.record_count().to_string())
    }
}

#[derive(Debug)]
struct Input {
    races: Vec<Race>,
}

#[derive(Debug, Clone, Copy)]
struct Race {
    duration: u64,
    record: u64,
}

impl Race {
    fn concat_fields(self, other: Race) -> Race {
        let duration = format!(
            "{}{}",
            self.duration.to_string(),
            other.duration.to_string()
        )
        .parse::<u64>()
        .unwrap();

        let record = format!("{}{}", self.record.to_string(), other.record.to_string())
            .parse::<u64>()
            .unwrap();

        Race { duration, record }
    }

    fn record_count(&self) -> u64 {
        (1..self.duration)
            .map(|d| self.distance_for_charge(d))
            .filter(|&t| t > self.record)
            .count() as u64
    }

    fn distance_for_charge(&self, charge_time: u64) -> u64 {
        let race_time = self.duration.saturating_sub(charge_time);
        race_time * charge_time
    }
}

mod parser {
    use super::{Input, Race};
    use nom::bytes::complete::tag;
    use nom::character::complete::multispace1;
    use nom::combinator::{all_consuming, verify};
    use nom::sequence::{preceded, separated_pair, tuple};
    use nom::{IResult, Parser};
    use utils::parse::whitespace_delimited_nums;

    pub(crate) fn parse_input(input: &str) -> IResult<&str, Input> {
        let nums = whitespace_delimited_nums::<u64>;
        let times = preceded(tuple((tag("Time:"), multispace1)), nums);
        let dists = preceded(tuple((tag("Distance:"), multispace1)), nums);
        let parser = all_consuming(separated_pair(times, tag("\n"), dists));
        let parser = verify(parser, |(times, dists)| times.len() == dists.len());

        parser
            .map(|(times, dists)| {
                let races = times
                    .iter()
                    .copied()
                    .zip(dists)
                    .map(|(duration, record)| Race { duration, record })
                    .collect::<Vec<_>>();
                Input { races }
            })
            .parse(input)
    }
}

fn main() -> Result<()> {
    utils::run_part1::<Problem6>()?;
    utils::run_part2::<Problem6>()?;
    Ok(())
}
