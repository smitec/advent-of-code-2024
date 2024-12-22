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

fn process(number: u64) -> u64 {
    let a = number << 6;
    let b = (a ^ number) % 16777216; // b is the secret now

    let c = b >> 5;
    let d = c ^ b; // d is the secret now
    let e = d % 16777216; // e is the secret now

    let f = e << 11;
    let g = f ^ e; // g is the secret now
    let h = g % 16777216;

    return h;
}

#[instrument]
pub fn day22(filename: String, part_b: bool, steps: i32) -> Result<()> {
    let content = fs::read_to_string(filename).context("Couldn't read input")?;

    let numbers: Vec<u64> = content.lines().map(|x| x.parse().unwrap()).collect();
    let mut sequences: HashMap<(i8, i8, i8, i8), i32> = HashMap::new();

    let mut sum: u64 = 0;
    for number in numbers {
        let mut value = number;
        let mut changes: Vec<i8> = Vec::new();
        let mut values: Vec<i8> = Vec::new();
        let mut last: i8 = 0;
        for i in 0..steps {
            value = process(value);
            if i > 0 {
                changes.push((value % 10) as i8 - last);
            }
            last = (value % 10) as i8;
            values.push(last);
        }
        sum += value;
        debug!(value = value, "Step");

        // Store all the 4 step combos and results (once with the earliest ones)
        let mut seen: HashSet<(i8, i8, i8, i8)> = HashSet::new();
        for offset in 3..changes.len() {
            let key_parts = &changes[offset - 3..=offset];
            let value = values[offset + 1];
            let key: (i8, i8, i8, i8) = (key_parts[0], key_parts[1], key_parts[2], key_parts[3]);
            if !seen.contains(&key) {
                sequences
                    .entry(key)
                    .and_modify(|x| {
                        *x += value as i32;
                    })
                    .or_insert(value as i32);
                seen.insert(key);
            }
        }
    }

    let max_b = sequences.iter().map(|(k, v)| *v).max();

    info!(sum, max_b, "Done");

    Ok(())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    info!("Tracing Setup");

    day22("./inputs/day22tiny.txt".to_string(), false, 10).context("Small Example")?;
    day22("./inputs/day22small.txt".to_string(), false, 2000).context("Small Example")?;
    day22("./inputs/day22.txt".to_string(), false, 2000).context("Big Example")?;

    // day22("./inputs/day22small.txt".to_string(), true).context("Small Example")?;
    // day22("./inputs/day22.txt".to_string(), true).context("Big Example")?;

    Ok(())
}
