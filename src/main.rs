use std::{
    collections::{HashMap, HashSet},
    fs, io,
};

use anyhow::{Context, Result};
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

fn can_be_made(target: String, parts: &Vec<String>, cache: &mut HashMap<String, u64>) -> u64 {
    if cache.contains_key(&target) {
        debug!(cache_size = cache.len(), target = target, "Way Cache Hit");
        return *cache.get(&target).unwrap();
    }

    // Check for the longest possible substring
    //for split in 1..target.len() {
    let mut to_try: Vec<String> = cache
        .iter()
        .filter(|&(_, v)| *v > 0)
        .map(|(k, _)| k.clone())
        .collect();
    to_try.extend(parts.clone());
    to_try.sort_by_key(|x| -(x.len() as i32));

    let mut ways: u64 = 0;
    for part in parts.clone() {
        if target.len() >= part.len() {
            let (l, r) = target.split_at(part.len());
            if l == part {
                if r.len() == 0 {
                    debug!(l = l, r = r, cache_size = cache.len(), "Way Found");
                    ways += 1;
                } else {
                    let result = can_be_made(r.to_string(), parts, cache);
                    if result > 0 {
                        debug!(
                            l = l,
                            r = r,
                            cache_size = cache.len(),
                            result = result,
                            "Way Found"
                        );
                        ways += result;
                    }
                }
            }
        }
    }

    cache.insert(target.clone(), ways);
    return ways;
}

#[instrument]
pub fn day19(filename: String, part_b: bool) -> Result<()> {
    let content = fs::read_to_string(filename).context("Couldn't read input")?;
    let mut section_one = true;

    let mut parts: Vec<String> = Vec::new();
    let mut targets: Vec<String> = Vec::new();
    let mut cache: HashMap<String, u64> = HashMap::new();

    for line in content.lines() {
        if line.len() == 0 {
            section_one = false;
            continue;
        }

        if section_one {
            parts = line.split(", ").map(|x| x.to_string()).collect();
        } else {
            targets.push(line.to_string());
        }
    }

    parts.sort_by_key(|x| -(x.len() as i32));

    let mut total = 0;
    let mut total_b = 0;
    for target in targets {
        info!(target = target, "Trying Next");
        let result = can_be_made(target.clone(), &parts, &mut cache);
        if result > 0 {
            info!(target = ?target.clone(), cache_size=cache.len(), result=result, "Found Match");
            total += 1;
            total_b += result;
        } else {
            info!(target = ?target.clone(), cache_size=cache.len(), result=result, "No Match");
        }
    }

    info!(total = total, total_b = total_b, "Done");

    Ok(())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    info!("Tracing Setup");

    day19("./inputs/day19small.txt".to_string(), false).context("Small Example")?;
    day19("./inputs/day19.txt".to_string(), false).context("Big Example")?;

    Ok(())
}
