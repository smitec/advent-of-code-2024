use core::panic;
use std::{
    collections::{HashMap, HashSet},
    fs,
    iter::zip,
};

use anyhow::{Context, Result};
use itertools::{Combinations, Itertools, Permutations};
use tracing::{Level, debug, error, event, info, instrument, warn};
use tracing_subscriber::{EnvFilter, field::debug};

mod done;
mod util;

#[instrument]
pub fn dayxx(part_b: bool) -> Result<()> {
    //let content = fs::read_to_string(filename).context("Couldn't read input")?;

    Ok(())
}

#[derive(Clone)]
enum KeyLocks {
    Key(Vec<i32>),
    Lock(Vec<i32>),
}

#[instrument]
pub fn day25(filename: &str, part_b: bool) -> Result<()> {
    let content = fs::read_to_string(filename).context("Couldn't read input")?;

    let kls: Vec<KeyLocks> = content
        .split("\n\n")
        .map(|grid| {
            let mut values: Vec<i32> = vec![0, 0, 0, 0, 0];
            let mut is_key = true;
            for (r, line) in grid.lines().enumerate() {
                for (c, v) in line.chars().enumerate() {
                    if r == 0 && v == '#' {
                        is_key = false;
                    }

                    if v == '#' {
                        if is_key {
                            values[c] = values[c].max(6 - r as i32);
                        } else {
                            values[c] = values[c].max(r as i32);
                        }
                    }
                }
            }

            if is_key {
                return KeyLocks::Key(values);
            } else {
                return KeyLocks::Lock(values);
            }
        })
        .collect();

    let keys: Vec<KeyLocks> = kls
        .iter()
        .filter(|x| matches!(x, KeyLocks::Key(_)))
        .cloned()
        .collect();
    let locks: Vec<KeyLocks> = kls
        .iter()
        .filter(|x| matches!(x, KeyLocks::Lock(_)))
        .cloned()
        .collect();

    debug!(
        key_len = keys.len(),
        lock_len = locks.len(),
        "Setup Complete"
    );

    let mut total = 0;
    for key in keys.iter().cloned() {
        for lock in locks.iter().cloned() {
            if let KeyLocks::Key(ref key_list) = key {
                if let KeyLocks::Lock(ref lock_list) = lock {
                    let mut okay = true;
                    for (key_v, lock_v) in zip(key_list, lock_list) {
                        if key_v + lock_v > 5 {
                            debug!(key_v, lock_v, "Failed");
                            okay = false;
                            break;
                        }
                    }
                    if okay {
                        total += 1;
                    }
                }
            }
        }
    }

    info!(total, "Done");

    Ok(())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    info!("Tracing Setup");

    day25("./inputs/day25small.txt", false).context("Big Example")?;
    day25("./inputs/day25.txt", false).context("Big Example")?;

    Ok(())
}
