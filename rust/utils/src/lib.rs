use anyhow::Context;
use std::path::PathBuf;

pub fn input_path(n: u8) -> PathBuf {
    let mut p = PathBuf::from("/home/ssanderson/projects/aoc-2023/problems/");
    p.push(n.to_string());
    p.push("input.txt");
    p
}

pub type Result<T> = anyhow::Result<T>;

pub trait Part1 {
    const N: u8;
    type Input1;
    fn parse1(data: &str) -> anyhow::Result<Self::Input1>;
    fn run1(input: Self::Input1) -> anyhow::Result<String>;
}

pub trait Part2: Part1 {
    type Input2;
    fn parse2(data: &str) -> anyhow::Result<Self::Input2>;
    fn run2(input: Self::Input2) -> anyhow::Result<String>;
}

pub fn run_part1<T: Part1>() -> anyhow::Result<()> {
    let p = input_path(T::N);
    let data =
        std::fs::read_to_string(&p).with_context(|| format!("Failed to read {}", p.display()))?;

    println!("============= Part 1 ============= ");
    let parsed = T::parse1(&data)?;
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
    let parsed = T::parse2(&data)?;
    let result = T::run2(parsed)?;
    println!("Result: {}", result);
    println!("=========== End Part 2 =========== ");

    Ok(())
}
