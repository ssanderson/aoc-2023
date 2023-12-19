pub mod parse;

use anyhow::Context;
use std::path::PathBuf;

/// Take a statically-known number of items from an iterator.
pub fn take_fixed<const N: usize, T>(mut it: impl Iterator<Item = T>) -> [Option<T>; N] {
    let mut out: [Option<T>; N] = std::array::from_fn(|_| None);
    for elem in out.iter_mut() {
        *elem = it.next();
    }
    out
}

pub fn map_fixed<const N: usize, T>(

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_take_fixed() {
        let [one, two] = take_fixed([1, 2, 3].into_iter());
        assert_eq!([one, two], [Some(1), Some(2)]);

        let items = take_fixed::<4, _>([1, 2, 3].into_iter());
        assert_eq!(items, [Some(1), Some(2), Some(3), None]);
    }
}

pub fn input_path(n: u8) -> PathBuf {
    let mut p = PathBuf::from("/home/ssanderson/projects/aoc-2023/problems/");
    p.push(n.to_string());
    p.push("input.txt");
    p
}

pub type Result<T> = anyhow::Result<T>;

pub trait Part1 {
    const N: u8;
    type Input;
    fn parse(data: &str) -> anyhow::Result<Self::Input>;
    fn run1(input: Self::Input) -> anyhow::Result<String>;
}

pub trait Part2: Part1 {
    fn run2(input: Self::Input) -> anyhow::Result<String>;
}

pub fn run_part1<T: Part1>() -> anyhow::Result<()> {
    let p = input_path(T::N);
    let data =
        std::fs::read_to_string(&p).with_context(|| format!("Failed to read {}", p.display()))?;

    println!("============= Part 1 ============= ");
    let parsed = T::parse(&data)?;
    let result = T::run1(parsed)?;
    println!("Result: {}", result);
    println!("=========== End Part 1 =========== ");

    Ok(())
}

pub fn run_part2<T: Part2>() -> anyhow::Result<()> {
    let p = input_path(T::N);
    let data =
        std::fs::read_to_string(&p).with_context(|| format!("Failed to read {}", p.display()))?;

    println!("============= Part 2 ============= ");
    let parsed = T::parse(&data)?;
    let result = T::run2(parsed)?;
    println!("Result: {}", result);
    println!("=========== End Part 2 =========== ");

    Ok(())
}
