// https://adventofcode.com/2022/day/4
use anyhow::{ensure, Context, Ok};

const INPUT_PATH: &str = "days/day-4/resources/input";

#[derive(Debug)]
struct SectionRange {
    lower: u8,
    upper: u8,
}

impl SectionRange {
    fn new(lower: u8, upper: u8) -> anyhow::Result<Self> {
        ensure!(
            lower <= upper,
            "Lower can't be bigger than upper in section range: {} > {}",
            lower,
            upper
        );

        Ok(Self { lower, upper })
    }

    fn within_range(&self, value: u8) -> bool {
        value >= self.lower && value <= self.upper
    }

    fn overlaps(a: &Self, b: &Self) -> bool {
        a.within_range(b.lower)
            || a.within_range(b.upper)
            || b.within_range(a.lower)
            || b.within_range(a.upper)
    }

    fn includes(&self, other: &Self) -> bool {
        other.lower >= self.lower && other.upper <= self.upper
    }
}

impl TryFrom<&str> for SectionRange {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (lower, upper) = value
            .split_once('-')
            .with_context(|| format!("Failed to parse range: {}", value))?;

        let lower = lower.parse::<u8>()?;
        let upper = upper.parse::<u8>()?;

        Self::new(lower, upper)
    }
}

impl std::fmt::Display for SectionRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.lower, self.upper)
    }
}

fn part_one(input: &str) -> anyhow::Result<()> {
    println!("=== Part One ===");

    let mut matches = 0u16;

    for sections in input.lines().map(|line| line.split_once(',')) {
        let (first, second) = sections.with_context(|| "Failed to split section ranges.")?;

        let first = TryInto::<SectionRange>::try_into(first)?;
        let second = TryInto::<SectionRange>::try_into(second)?;

        if first.includes(&second) || second.includes(&first) {
            matches += 1;
        }
    }

    println!("Included pairs: {}", matches);

    Ok(())
}

fn part_two(input: &str) -> anyhow::Result<()> {
    println!("=== Part Two ===");

    let mut matches = 0u16;

    for sections in input.lines().map(|line| line.split_once(',')) {
        let (first, second) = sections.with_context(|| "Failed to split section ranges.")?;

        let first = TryInto::<SectionRange>::try_into(first)?;
        let second = TryInto::<SectionRange>::try_into(second)?;

        if SectionRange::overlaps(&first, &second) {
            matches += 1;
        }
    }

    println!("Overlapping pairs: {}", matches);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;

    part_one(&input)?;
    println!();
    part_two(&input)?;

    Ok(())
}
