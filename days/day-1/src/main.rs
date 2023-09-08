// https://adventofcode.com/2022/day/1
const FILE_PATH: &str = "days/day-1/resources/input";

fn read_input_file() -> String {
    std::fs::read_to_string(FILE_PATH).expect("Failed to read input file.")
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Elf {
    calories: u32,
}

fn parse_input(raw_input: &str) -> Vec<Elf> {
    let mut elves = Vec::new();

    for section in raw_input.split("\n\n") {
        let mut calories = Vec::new();

        for line in section.lines() {
            // Trimming might not be necessary, but I'm doing it just to be safe
            calories.push(
                line.trim()
                    .parse::<u32>()
                    .expect("Failed to parse input line."),
            );
        }

        elves.push(Elf {
            calories: calories.iter().sum(),
        })
    }

    elves
}

fn part_one(elves: Vec<Elf>) {
    println!("=== Part One ===");

    let mut max_calories = elves.get(0).expect("Expected at least one elf.").calories;
    let mut max_elf_index = 0;

    for (i, elf) in elves.iter().skip(1).enumerate() {
        if elf.calories > max_calories {
            max_calories = elf.calories;
            max_elf_index = i;
        }
    }

    println!(
        "Elf #{} has the most calories totaling at {}.",
        max_elf_index + 1,
        max_calories,
    );
}

fn part_two(mut elves: Vec<Elf>) {
    println!("=== Part Two ===");

    // Highest to lowest
    elves.sort_by(|a, b| b.cmp(a));

    let total = elves.iter().take(3).map(|elf| elf.calories).sum::<u32>();

    println!("The top three elves have {} calories of snacks.", total);
}

fn main() {
    let input = parse_input(&read_input_file());
    part_one(input.clone());
    println!();
    part_two(input);
}
