// https://adventofcode.com/2022/day/2
use std::{fs::read_to_string, slice::Chunks};

use anyhow::{bail, Context};

const INPUT_PATH: &str = "days/day-2/resources/input";

#[derive(Debug, Clone, Copy, PartialEq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    fn score(self) -> u8 {
        self as u8
    }

    fn from_move(value: char) -> anyhow::Result<Self> {
        Ok(match value {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => bail!("Failed to the parse move code: {}", value),
        })
    }

    fn from_intent(elf: Move, value: char) -> anyhow::Result<Self> {
        Ok(match value {
            // Loose
            'X' => elf.loose_against(),
            // Draw
            'Y' => elf,
            // Win
            'Z' => elf.win_against(),
            _ => bail!("Failed to the parse move code: {}", value),
        })
    }

    fn win_against(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn loose_against(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    fn wins(main: &Move, against: &Move) -> bool {
        against.win_against() == *main
    }
}

#[derive(Debug, Clone, Copy)]
enum GameResult {
    Lost = 0,
    Draw = 3,
    Win = 6,
}

impl GameResult {
    fn score(self) -> u8 {
        self as u8
    }

    fn from_moves(elf: Move, you: Move) -> Self {
        if elf == you {
            Self::Draw
        } else if Move::wins(&you, &elf) {
            Self::Win
        } else {
            Self::Lost
        }
    }
}

fn part_one<'a>(games: Chunks<'a, &str>) -> anyhow::Result<()> {
    println!("=== Part One ===");

    let mut score = 0u32;

    for game in games {
        assert_eq!(game.len(), 2, "Game is incomplete.");

        let elf_move = Move::from_move(
            game[0]
                .chars()
                .nth(0)
                .with_context(|| format!("Failed to get character: '{}'", game[1]))?,
        )?;
        let your_move = Move::from_move(
            game[1]
                .chars()
                .nth(0)
                .with_context(|| format!("Failed to get character: '{}'", game[1]))?,
        )?;

        let game_result = GameResult::from_moves(elf_move, your_move);
        score += (game_result.score() + your_move.score()) as u32;
    }

    println!("Total score is: {}", score);

    Ok(())
}

fn part_two<'a>(games: Chunks<'a, &str>) -> anyhow::Result<()> {
    println!("=== Part Two ===");

    let mut score = 0u32;

    for game in games {
        assert_eq!(game.len(), 2, "Game is incomplete.");

        let elf_move = Move::from_move(
            game[0]
                .chars()
                .nth(0)
                .with_context(|| format!("Failed to get character: '{}'", game[1]))?,
        )?;
        let your_move = Move::from_intent(
            elf_move,
            game[1]
                .chars()
                .nth(0)
                .with_context(|| format!("Failed to get character: '{}'", game[1]))?,
        )?;

        let game_result = GameResult::from_moves(elf_move, your_move);
        score += (game_result.score() + your_move.score()) as u32;
    }

    println!("Total score is: {}", score);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let input = read_to_string(INPUT_PATH)
        .with_context(|| format!("Failed to read input file at: {}", INPUT_PATH))?;

    let moves = input.trim().split(&[' ', '\n']).collect::<Vec<_>>();
    let games = moves.chunks(2);

    part_one(games.clone())?;
    println!();
    part_two(games)?;

    Ok(())
}
