use std::{
    collections::{HashMap, HashSet},
    fs, i32, io,
};

use anyhow::{Context, Result};
use itertools::Itertools;
use rayon::prelude::*;
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
    tricky: bool,
) -> Result<Vec<String>> {
    let mut current: (i32, i32) = keypad.get(&'A').context("No A Key Found!")?.clone();

    let mut result: Vec<String> = Vec::new();
    result.push("".to_string());

    for c in target.chars() {
        let mut directions_v: Vec<Direction> = Vec::new();
        let mut directions_h: Vec<Direction> = Vec::new();
        let target = keypad.get(&c).context("Unexpected Character Requested")?;
        let dr: i32 = target.0 - current.0;
        let dc: i32 = target.1 - current.1;

        for _ in 0..dc.abs() {
            if dc < 0 {
                directions_h.push(Direction::West);
            } else if dc > 0 {
                directions_h.push(Direction::East);
            }
        }

        for _ in 0..dr.abs() {
            if dr < 0 {
                directions_v.push(Direction::North);
            } else if dr > 0 {
                directions_v.push(Direction::South);
            }
        }

        let mut new_results: Vec<String> = Vec::new();

        // Surely only LR then UD or UD then LR matter
        let mut vh = directions_v.clone();
        vh.extend(directions_h.clone());

        let mut hv = directions_h.clone();
        hv.extend(directions_v.clone());
        /*
        for permutation in directions
            .iter()
            .cloned()
            .permutations(directions.len())
            .unique()
        */
        let mut choices: Vec<Vec<Direction>> = Vec::new();
        if tricky {
            if dc == 0 || dr == 0 {
                choices.push(hv);
            } else if dr > 0 && dc > 0 {
                choices.push(vh);
            } else if dr > 0 && dc < 0 {
                if dc == -1 {
                    choices.push(hv);
                    choices.push(vh);
                } else {
                    choices.push(vh);
                }
            } else if dr < 0 && dc < 0 {
                if dc == -1 {
                    choices.push(hv);
                    choices.push(vh);
                } else {
                    choices.push(vh);
                }
            } else {
                // dr < 0 && dc > 0
                if dc == 1 {
                    choices.push(vh);
                    choices.push(hv);
                } else {
                    choices.push(hv);
                }
            }
        } else {
            choices.push(vh);
            choices.push(hv);
        }

        for permutation in choices.iter() {
            let mut test = current;
            let mut keep = true;
            for step in permutation.iter().cloned() {
                let new_pos = move_direction(&test, &step);
                if new_pos == empty {
                    keep = false;
                    // We can never actually get here apparently based on how we've set things up
                }
                test = new_pos;
            }
            if keep {
                for current_string in result.iter().cloned() {
                    let mut new_string = current_string.clone();
                    new_string.extend(directions_to_keys(&permutation));
                    new_results.push(new_string);
                }
                if tricky {
                    break;
                }
            }
        }
        result = new_results.iter().unique().cloned().collect();
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

    let steps: i32;

    if part_b {
        steps = 25;
    } else {
        steps = 2;
    }

    for code in codes {
        let step_1: Vec<String> = generate_short_path(&numeric_pad, numeric_empty, &code, false)
            .context("Step 1 Failed")?;

        let mut step_n: Vec<String> = step_1.clone();
        let mut completed = 0;

        for _ in 0..steps {
            debug!(len = step_n.len(), "Continuing");

            let next_step: Vec<String> = step_n
                .par_iter()
                .map(|input| generate_short_path(&key_pad, keypad_empty, &input, true).unwrap())
                .flatten()
                .collect();

            let min_len = next_step
                .iter()
                .map(|x| x.len())
                .min()
                .context("Nothing to len")?;

            // Only keep the shortest sequences
            step_n = next_step
                .iter()
                .filter(|x| x.len() == min_len)
                .cloned()
                .collect();
            debug!(pre_l = next_step.len(), post_l = step_n.len(), "Filtered");

            completed += 1;

            if step_n.len() == 1 {
                debug!(completed, "Leaving Part 1");
                break;
            }
        }

        if step_n.len() != 1 {
            error!("More than 1 Option Left");
        }
        let next_step: String = step_n[0].clone();
        let next_parts: Vec<String> = next_step
            .split_inclusive('A')
            .map(|x| x.to_string())
            .collect();
        let mut next_hist: HashMap<String, u64> = HashMap::new();

        for part in next_parts {
            next_hist.entry(part).and_modify(|x| *x += 1).or_insert(1);
        }

        for _ in completed..steps {
            let mut new_hist: HashMap<String, u64> = HashMap::new();
            for (k, v) in next_hist.clone() {
                let chunk = generate_short_path(&key_pad, keypad_empty, &k, true).unwrap();
                let chunk_parts: Vec<String> = chunk[0]
                    .split_inclusive('A')
                    .map(|x| x.to_string())
                    .collect();
                for part in chunk_parts {
                    new_hist.entry(part).and_modify(|x| *x += v).or_insert(v);
                }
            }
            next_hist = new_hist;
        }

        let mut len = 0;
        for (k, v) in next_hist.clone() {
            len += k.len() as u64 * v;
        }
        let digits: u64 = code[..3].parse().context("Couldn't parse code digits")?;
        let complexity: u64 = digits * len as u64;
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

    day21("./inputs/day21.txt".to_string(), true).context("Big Example")?;
    //day21("./inputs/day21small.txt".to_string(), true).context("Small Example")?;

    Ok(())
}
