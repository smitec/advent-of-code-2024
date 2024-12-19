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

fn can_be_made(target: String, parts: &Vec<String>, cache: &mut HashMap<String, bool>) -> bool {
    if target.len() == 0 {
        return true;
    }

    if cache.contains_key(&target) {
        return *cache.get(&target).unwrap();
    }

    // Check for the longest possible substring
    //for split in 1..target.len() {
    let mut to_try: Vec<String> = cache
        .iter()
        .filter(|&(_, v)| *v)
        .map(|(k, _)| k.clone())
        .collect();
    to_try.sort_by_key(|x| -(x.len() as i32));

    let mut ways = 0;
    for part in to_try {
        if target.len() >= part.len() {
            let (l, r) = target.split_at(part.len());
            if l == part {
                debug!(l = l, r = r, cache_size = cache.len(), "Trying");
                if can_be_made(r.to_string(), parts, cache) {
                    cache.insert(target, true);
                    return true;
                }
            }
        }
    }

    cache.insert(target.clone(), false);
    debug!(
        target = target,
        cache_size = cache.len(),
        "adding false to cache"
    );
    return false;
}

#[instrument]
pub fn day19(filename: String, part_b: bool) -> Result<()> {
    let content = fs::read_to_string(filename).context("Couldn't read input")?;
    let mut section_one = true;

    let mut parts: Vec<String> = Vec::new();
    let mut targets: Vec<String> = Vec::new();
    let mut cache: HashMap<String, bool> = HashMap::new();

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

    // Pre-Cache
    for part in parts.clone() {
        cache.insert(part.clone(), true);
        for part_2 in parts.clone() {
            let both: String = format!("{}{}", part, part_2);
            cache.insert(both, true);
        }
    }

    let mut total = 0;
    for target in targets {
        if can_be_made(target.clone(), &parts, &mut cache) {
            info!(target = ?target.clone(), cache_size=cache.len(), "Found Match");
            total += 1;
        } else {
            info!(target = ?target.clone(), cache_size=cache.len(), "No Match");
        }
    }

    info!(total = total, "Done");

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
