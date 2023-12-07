use std::str::FromStr;

use nom::error::Error;
use nom::Finish;

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    pub seeds: Vec<u64>,
    pub sections: Vec<Section>,
}

impl FromStr for Input {
    type Err = nom::error::Error<String>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match crate::parser::parse_input(s.trim()).finish() {
            Ok((_, result)) => Ok(result),
            Err(Error { input, code }) => Err(Error { input: input.to_string(), code }),
        }
    }
}

impl Input {
    pub fn seed_location(&self, seed: u64) -> u64 {
        self.sections
            .iter()
            .fold(seed, |val, section| section.translate(val))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Section {
    pub heading: String,
    pub entries: Vec<MapEntry>,
}

impl Section {
    fn translate(&self, val: u64) -> u64 {
        self.entries
            .iter()
            .find_map(|entry| entry.translate(val))
            .unwrap_or(val)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct MapEntry {
    pub dest: u64,
    pub source: u64,
    pub length: u64,
}

impl MapEntry {
    fn translate(&self, val: u64) -> Option<u64> {
        if val >= self.source && val < self.source + self.length {
            println!("{:?}", self);
            Some(self.dest + (val - self.source))
        } else {
            None
        }
    }
}

impl From<(u64, u64, u64)> for MapEntry {
    fn from((dest, source, length): (u64, u64, u64)) -> Self {
        MapEntry { dest, source, length }
    }
}
