use crate::input::{Input, MapEntry, Section};
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::digit1;
use nom::combinator::{all_consuming, map_res};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, terminated, tuple};
use nom::{IResult, Parser};

fn num(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

pub fn parse_input(input: &str) -> IResult<&str, Input> {
    let seeds = preceded(tag("seeds: "), separated_list1(tag(" "), num));

    let map_heading = terminated(take_until(" map:"), tag(" map:")).map(String::from);
    let map_entry =
        tuple((terminated(num, tag(" ")), terminated(num, tag(" ")), num)).map(MapEntry::from);

    let section = separated_pair(
        map_heading,
        tag("\n"),
        separated_list1(tag("\n"), map_entry),
    )
    .map(|(heading, entries)| Section { heading, entries });

    let sections = separated_list1(tag("\n\n"), section);
    let parser = all_consuming(separated_pair(seeds, tag("\n\n"), sections));
    parser
        .map(|(seeds, sections)| Input { seeds, sections })
        .parse(input)
}

#[cfg(test)]
mod test {
    use crate::{input::MapEntry, parser::parse_input};

    use super::Section;

    #[test]
    fn test_parse_input() -> anyhow::Result<()> {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

water-to-light map:
88 18 7
18 25 70"#;

        let (rem, parsed) = parse_input(input)?;
        assert_eq!(rem, "");
        assert_eq!(parsed.seeds, vec![79, 14, 55, 13]);
        assert_eq!(
            parsed.sections[0],
            Section {
                heading: String::from("seed-to-soil"),
                entries: vec![MapEntry::from((50, 98, 2)), MapEntry::from((52, 50, 48)),],
            }
        );

        assert_eq!(
            parsed.sections[1],
            Section {
                heading: String::from("soil-to-fertilizer"),
                entries: vec![
                    MapEntry::from((0, 15, 37)),
                    MapEntry::from((37, 52, 2)),
                    MapEntry::from((39, 0, 15)),
                ],
            }
        );

        assert_eq!(
            parsed.sections[2],
            Section {
                heading: String::from("water-to-light"),
                entries: vec![MapEntry::from((88, 18, 7)), MapEntry::from((18, 25, 70)),],
            }
        );

        Ok(())
    }
}
