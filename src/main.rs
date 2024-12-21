use core::panic;
use std::{
    collections::{HashMap, HashSet},
    fs, i32, io,
};

use anyhow::{Context, Result};
use itertools::Itertools;
use tracing::{Level, debug, error, event, info, instrument, warn};
use tracing_subscriber::EnvFilter;

mod done;
mod util;

use crate::done::*;
use crate::util::*;

#[instrument]
pub fn dayxx(filename: String, part_b: bool) -> Result<()> {
    let content = fs::read_to_string(filename).context("Couldn't read input")?;

    Ok(())
}

fn directions_to_keys(directions: &Vec<Direction>) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    for dir in directions {
        result.push(match dir {
            Direction::North => '^',
            Direction::East => '>',
            Direction::West => '<',
            Direction::South => 'v',
        });
    }
    result.push('A');
    return result;
}

fn generate_short_path(
    keypad: &HashMap<char, (i32, i32)>,
    empty: (i32, i32),
    target: &String,
) -> Result<Vec<String>> {
    let mut current: (i32, i32) = keypad.get(&'A').context("No A Key Found!")?.clone();

    let mut result: Vec<String> = Vec::new();
    result.push("".to_string());

    for c in target.chars() {
        let mut directions: Vec<Direction> = Vec::new();
        let target = keypad.get(&c).context("Unexpected Character Requested")?;
        let dr: i32 = target.0 - current.0;
        let dc: i32 = target.1 - current.1;

        for _ in 0..dc.abs() {
            if dc < 0 {
                directions.push(Direction::West);
            } else if dc > 0 {
                directions.push(Direction::East);
            }
        }

        for _ in 0..dr.abs() {
            if dr < 0 {
                directions.push(Direction::North);
            } else if dr > 0 {
                directions.push(Direction::South);
            }
        }

        let mut new_results: Vec<String> = Vec::new();

        for permutation in directions
            .iter()
            .cloned()
            .permutations(directions.len())
            .unique()
        {
            let mut test = current;
            let mut keep = true;
            for step in permutation.iter().cloned() {
                let new_pos = move_direction(&test, &step);
                if new_pos == empty {
                    keep = false;
                }
                test = new_pos;
            }
            if keep {
                for current_string in result.iter().cloned() {
                    // Check if it passes zero
                    let mut new_string = current_string.clone();
                    new_string.extend(directions_to_keys(&permutation));
                    new_results.push(new_string);
                }
            }
        }
        result = new_results;
        current = target.clone();
    }

    Ok(result)
}

#[instrument]
pub fn day21(filename: String, part_b: bool) -> Result<()> {
    let content = fs::read_to_string(filename).context("Couldn't read input")?;

    let codes: Vec<String> = content.lines().map(|x| x.to_string()).collect();

    let numeric_pad: HashMap<char, (i32, i32)> = HashMap::from([
        ('0', (3, 1)),
        ('A', (3, 2)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
    ]);
    let numeric_empty: (i32, i32) = (3, 0);

    let key_pad: HashMap<char, (i32, i32)> = HashMap::from([
        ('<', (1, 0)),
        ('v', (1, 1)),
        ('>', (1, 2)),
        ('^', (0, 1)),
        ('A', (0, 2)),
    ]);

    let keypad_empty: (i32, i32) = (0, 0);

    let mut total = 0;

    for code in codes {
        let step_1: Vec<String> =
            generate_short_path(&numeric_pad, numeric_empty, &code).context("Step 1 Failed")?;
        debug!(len_1 = step_1.len(), "Step 1");
        let mut step_2: Vec<String> = Vec::new();
        for input in step_1 {
            let step_2_bits =
                generate_short_path(&key_pad, keypad_empty, &input).context("Step 2 Failed")?;
            step_2.extend(step_2_bits);
        }
        debug!(len_2=?step_2.len(), "Step 2");
        let mut step_3: Vec<String> = Vec::new();
        for input in step_2 {
            let step_3_bits =
                generate_short_path(&key_pad, keypad_empty, &input).context("Step 2 Failed")?;
            step_3.extend(step_3_bits);
        }
        debug!(len_3=?step_3.len(), "Step 3");
        let min_len = step_3
            .iter()
            .map(|x| x.len())
            .min()
            .context("Nothing to len")?;
        let digits: i32 = code[..3].parse().context("Couldn't parse code digits")?;
        let complexity: i32 = digits * min_len as i32;
        debug!(complexity, "Code Done");

        total += complexity;
    }

    info!(total, "Done");

    Ok(())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    info!("Tracing Setup");

    day21("./inputs/day21small.txt".to_string(), false).context("Small Example")?;
    day21("./inputs/day21.txt".to_string(), false).context("Big Example")?;

    day21("./inputs/day21small.txt".to_string(), true).context("Small Example")?;
    day21("./inputs/day21.txt".to_string(), true).context("Big Example")?;

    Ok(())
}
