use std::collections::HashMap;

use utils::{Part1, Part2, Result};

struct Problem8;

impl Problem8 {
    fn steps_to_complete(
        mut pos: Label,
        cond: impl Fn(Label) -> bool,
        directions: &[Direction],
        m: &HashMap<Label, (Label, Label)>,
    ) -> Result<usize> {
        for (i, &d) in directions.iter().cycle().enumerate() {
            pos = match (m.get(&pos), d) {
                (Some((left, _)), Direction::Left) => *left,
                (Some((_, right)), Direction::Right) => *right,
                (None, _) => anyhow::bail!("No map node for {pos:?}"),
            };
            if cond(pos) {
                return Ok(i + 1);
            }
        }
        unreachable!()
    }
}

impl Part1 for Problem8 {
    const N: u8 = 8;
    type Input = Input;

    fn parse(data: &str) -> anyhow::Result<Self::Input> {
        utils::parse::finalize(parser::parse_input(data.trim()))
    }

    fn run1(input: Self::Input) -> anyhow::Result<String> {
        let mut m = HashMap::new();
        for node in input.nodes.into_iter() {
            m.insert(node.label, node.choices);
        }

        let steps = Self::steps_to_complete(
            Label(['A', 'A', 'A']),
            |l| l == Label(['Z', 'Z', 'Z']),
            &input.directions,
            &m,
        )?;
        Ok(steps.to_string())
    }
}

impl Part2 for Problem8 {
    fn run2(input: Self::Input) -> anyhow::Result<String> {
        let mut m = HashMap::new();
        for node in input.nodes.iter() {
            m.insert(node.label, node.choices);
        }

        let pos: Vec<Label> = input
            .nodes
            .iter()
            .filter_map(|n| match n.label {
                Label([_, _, 'A']) => Some(n.label),
                _ => None,
            })
            .collect();

        let steps = pos
            .iter()
            .map(|&p| {
                Self::steps_to_complete(p, |Label([_, _, c])| c == 'Z', &input.directions, &m)
            })
            .collect::<Result<Vec<_>>>()?;

        let lcm = steps
            .iter()
            .copied()
            .reduce(|a, b| num::integer::lcm(a, b))
            .unwrap();

        Ok(lcm.to_string())
    }
}

struct Input {
    directions: Vec<Direction>,
    nodes: Vec<MapNode>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
struct Label([char; 3]);

struct MapNode {
    label: Label,
    choices: (Label, Label),
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

mod parser {
    use super::{Direction, Input, Label, MapNode};
    use nom::bytes::complete::{self, is_a, tag};
    use nom::multi::separated_list1;
    use nom::sequence::{self, separated_pair};
    use nom::{IResult, Parser};

    fn directions(input: &str) -> IResult<&str, Vec<Direction>> {
        is_a("LR")
            .map(|s: &str| {
                s.chars()
                    .map(|c| match c {
                        'L' => Direction::Left,
                        'R' => Direction::Right,
                        _ => panic!("bad direction {s}"),
                    })
                    .collect::<Vec<Direction>>()
            })
            .parse(input)
    }

    fn label(input: &str) -> IResult<&str, Label> {
        complete::take(3usize)
            .map(|s: &str| {
                let mut it = s.chars();
                let (Some(a), Some(b), Some(c)) = (it.next(), it.next(), it.next()) else {
                    panic!("bad s: {s}")
                };
                Label([a, b, c])
            })
            .parse(input)
    }

    fn node(input: &str) -> IResult<&str, MapNode> {
        let opts = sequence::separated_pair(label, tag(", "), label);
        let rhs = sequence::delimited(tag("("), opts, tag(")"));
        let parser = sequence::separated_pair(label, tag(" = "), rhs);
        parser
            .map(|(label, choices)| MapNode { label, choices })
            .parse(input)
    }

    pub fn parse_input(input: &str) -> IResult<&str, Input> {
        let nodes = separated_list1(tag("\n"), node);
        let parser = separated_pair(directions, tag("\n\n"), nodes);
        nom::combinator::all_consuming(parser)
            .map(|(directions, nodes)| Input { directions, nodes })
            .parse(input)
    }
}

fn main() -> Result<()> {
    utils::run_part1::<Problem8>()?;
    utils::run_part2::<Problem8>()?;
    Ok(())
}
