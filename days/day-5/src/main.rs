// https://adventofcode.com/2022/day/5
use anyhow::Context;

const INPUT_FILE: &str = "days/day-5/resources/input";
const SUPPLY_CHAR_SEPARATION: u8 = 4;
const COLUMNS: u8 = 9;

struct Procedure {
    amount: u8,
    from: u8,
    to: u8,
}

impl TryFrom<&str> for Procedure {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let procedure = value.split_whitespace().collect::<Vec<_>>();
        let amount = procedure
            .get(1)
            .with_context(|| "Missing 'amount' in procedure.")?
            .parse::<u8>()?;
        let from = procedure
            .get(3)
            .with_context(|| "Missing 'from' in procedure.")?
            .parse::<u8>()?;
        let to = procedure
            .get(5)
            .with_context(|| "Missing 'to' in procedure.")?
            .parse::<u8>()?;

        Ok(Self { amount, from, to })
    }
}

impl std::fmt::Display for Procedure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "move {} from {} to {}", self.amount, self.from, self.to)
    }
}

struct Supply {
    columns: Vec<Vec<char>>,
}

impl Supply {
    fn execute_procedure(&mut self, procedure: Procedure) -> anyhow::Result<()> {
        for _ in 0..procedure.amount {
            self.move_crate(procedure.from, procedure.to)?;
        }
        Ok(())
    }

    fn move_crate(&mut self, from: u8, to: u8) -> anyhow::Result<()> {
        let from = from - 1;
        let to = to - 1;

        let move_crate = self
            .columns
            .get_mut(from as usize)
            .with_context(|| format!("Couldn't get find column at index {}", from))?
            .pop()
            .with_context(|| "Column is empty.")?;

        self.columns
            .get_mut(to as usize)
            .with_context(|| format!("Couldn't get find column at index {}", to))?
            .push(move_crate);
        Ok(())
    }
}

impl TryFrom<&str> for Supply {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut columns = vec![Vec::new(); COLUMNS as usize];
        let mut lines = value.lines().collect::<Vec<_>>();
        lines.pop();

        for row_index in (0..lines.len()).rev() {
            let line = lines[row_index];
            for (column_index, character) in line.chars().enumerate() {
                if character.is_alphabetic() {
                    let index = (column_index - 1) / SUPPLY_CHAR_SEPARATION as usize;
                    columns
                        .get_mut(index)
                        .with_context(|| format!("Wasn't able to find column at index: {}", index))?
                        .push(character);
                }
            }
        }

        Ok(Self { columns })
    }
}

fn main() -> anyhow::Result<()> {
    let mut supply;
    let mut procedures = Vec::new();

    {
        let input = std::fs::read_to_string(INPUT_FILE)?;

        let sections = input.split("\n\n").collect::<Vec<_>>();
        supply = TryInto::<Supply>::try_into(
            *sections
                .get(0)
                .with_context(|| "Failed to find supply section.")?,
        )?;

        let procedure_section = sections
            .get(1)
            .with_context(|| "Failed to find procedure section.")?;

        for line in procedure_section.lines() {
            let procedure = TryInto::<Procedure>::try_into(line)?;
            procedures.push(procedure);
        }
    }

    for procedure in procedures {
        supply.execute_procedure(procedure)?;
    }

    let mut top_characters = String::new();

    for column in supply.columns {
        top_characters.push(column.last().with_context(|| "Column is empty.")?.clone());
    }

    println!("The top crates in the supply are: {}", top_characters);

    Ok(())
}
