use std::collections::HashMap;

struct Problem19;

impl utils::Part1 for Problem19 {
    type Input = Input;
    const N: u8 = 19;

    fn parse(data: &str) -> anyhow::Result<Self::Input> {
        utils::parse::finalize(parser::parse_input(data.trim()))
    }

    fn run1(input: Self::Input) -> anyhow::Result<String> {
        let workflows: HashMap<String, Vec<Rule>> = input
            .workflows
            .into_iter()
            .map(|w| (w.name, w.rules))
            .collect();

        let accepted = input.parts.iter().filter(|part| {
            let mut pos = Label::Workflow("in".to_owned());

            while let Label::Workflow(target) = pos.clone() {
                pos = match workflows.get(&target) {
                    Some(rules) => {
                        let Some(pos) = rules.iter().find_map(|rule| {
                            if rule.matches(&part) {
                                Some(rule.dest.clone())
                            } else {
                                None
                            }
                        }) else {
                            panic!("no rules matched {target}");
                        };
                        pos
                    }
                    None => panic!("failed lookup for {target}"),
                };
            }

            match pos {
                Label::Accept => true,
                Label::Reject => false,
                _ => panic!("Invalid terminal state {pos:?}"),
            }
        });

        let res: u32 = accepted.map(|p| p.x + p.m + p.a + p.s).sum();

        Ok(res.to_string())
    }
}

fn main() -> anyhow::Result<()> {
    utils::run_part1::<Problem19>()?;
    Ok(())
}

struct Input {
    workflows: Vec<Workflow>,
    parts: Vec<Part>,
}

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
struct Rule {
    cond: Cond,
    dest: Label,
}

impl Rule {
    fn matches(&self, part: &Part) -> bool {
        match self.cond {
            Cond::Always => true,
            Cond::Compare(attr, Cmp::Lt, n) => part[attr] < n,
            Cond::Compare(attr, Cmp::Gt, n) => part[attr] > n,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Cond {
    Always,
    Compare(Attr, Cmp, u32),
}

#[derive(Debug, Clone, Copy)]
enum Cmp {
    Gt,
    Lt,
}

#[derive(Debug, Clone, Copy)]
enum Attr {
    X,
    M,
    A,
    S,
}

impl std::ops::Index<Attr> for Part {
    type Output = u32;

    fn index(&self, index: Attr) -> &Self::Output {
        match index {
            Attr::X => &self.x,
            Attr::M => &self.m,
            Attr::A => &self.a,
            Attr::S => &self.s,
        }
    }
}

#[derive(Debug, Clone)]
enum Label {
    Accept,
    Reject,
    Workflow(String),
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, one_of},
        combinator::all_consuming,
        multi::separated_list1,
        sequence::{delimited, separated_pair, tuple},
        IResult, Parser,
    };

    use crate::{Attr, Cmp, Cond, Input, Label, Part, Rule, Workflow};

    fn label(input: &str) -> IResult<&str, Label> {
        alpha1
            .map(|s| match s {
                "A" => Label::Accept,
                "R" => Label::Reject,
                w => Label::Workflow(w.to_string()),
            })
            .parse(input)
    }

    fn part(input: &str) -> IResult<&str, Part> {
        let attrs = tuple((
            delimited(tag("{x="), utils::parse::num::<u32>, tag(",")),
            delimited(tag("m="), utils::parse::num::<u32>, tag(",")),
            delimited(tag("a="), utils::parse::num::<u32>, tag(",")),
            delimited(tag("s="), utils::parse::num::<u32>, tag("}")),
        ));
        attrs.map(|(x, m, a, s)| Part { x, m, a, s }).parse(input)
    }

    pub fn parse_input(input: &str) -> IResult<&str, Input> {
        let cond_rule = tuple((
            one_of("xmas"),
            one_of("<>"),
            utils::parse::num::<u32>,
            tag(":"),
            label,
        ))
        .map(|(attr, cond, n, _, dest)| {
            let attr = match attr {
                'x' => Attr::X,
                'm' => Attr::M,
                'a' => Attr::A,
                's' => Attr::S,
                _ => panic!("invalid attr {attr}"),
            };
            let cond = match cond {
                '<' => Cond::Compare(attr, Cmp::Lt, n),
                '>' => Cond::Compare(attr, Cmp::Gt, n),
                _ => panic!("invalid cond {cond}"),
            };
            Rule { cond, dest }
        });

        let uncond_rule = label.map(|dest| Rule { cond: Cond::Always, dest });

        let rule = alt((cond_rule, uncond_rule));
        let rules = delimited(tag("{"), separated_list1(tag(","), rule), tag("}"));

        let workflow =
            tuple((alpha1, rules)).map(|(name, rules)| Workflow { name: name.to_string(), rules });

        let workflows = separated_list1(tag("\n"), workflow);

        let parts = separated_list1(tag("\n"), part);

        let parser = separated_pair(workflows, tag("\n\n"), parts);

        all_consuming(parser)
            .map(|(workflows, parts)| Input { workflows, parts })
            .parse(input)
    }
}
