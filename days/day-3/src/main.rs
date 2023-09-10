use anyhow::bail;

// https://adventofcode.com/2022/day/3
const INPUT_PATH: &str = "days/day-3/resources/input";

fn char_to_bitmask(character: char) -> anyhow::Result<u64> {
    Ok(match character {
        'a'..='z' => 1 << (character as u64 - 96),
        'A'..='Z' => 1 << (character as u64 - 38),
        _ => bail!("Character is not supported for bitmask: {}", character),
    })
}

fn create_character_bitmask(text: &str) -> anyhow::Result<u64> {
    let mut bitmask = 0u64;

    for character in text.chars() {
        bitmask |= char_to_bitmask(character)?;
    }

    Ok(bitmask)
}

fn part_one(input: &str) -> anyhow::Result<()> {
    println!("=== Part One ===");

    let mut score = 0u32;

    for line in input.lines() {
        let (compartment_one, compartment_two) = line.split_at(line.len() / 2);
        let bitmask_common =
            create_character_bitmask(compartment_one)? & create_character_bitmask(compartment_two)?;

        score += bitmask_common.ilog2();
    }

    println!("The total score is: {}", score);

    Ok(())
}

fn part_two(input: &str) -> anyhow::Result<()> {
    println!("=== Part Two ===");

    let lines = input.lines().collect::<Vec<_>>();

    let group_size = 3;

    assert_eq!(
        lines.len() % group_size,
        0,
        "Incorrect amount of groups. Should be a multiple of 3."
    );

    let mut score = 0u32;

    for group in lines.chunks(group_size) {
        let bitmask = create_character_bitmask(group[0])?
            & create_character_bitmask(group[1])?
            & create_character_bitmask(group[2])?;

        score += bitmask.ilog2();
    }

    println!("The total score is: {}", score);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;

    part_one(&input)?;
    println!();
    part_two(&input)?;

    Ok(())
}
