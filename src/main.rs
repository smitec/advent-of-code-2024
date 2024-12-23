use std::{
    collections::{HashMap, HashSet},
    fs,
};

use anyhow::{Context, Result};
use tracing::{Level, debug, error, event, info, instrument, warn};
use tracing_subscriber::EnvFilter;

mod done;
mod util;

#[instrument]
pub fn dayxx(filename: String, part_b: bool) -> Result<()> {
    let content = fs::read_to_string(filename).context("Couldn't read input")?;

    Ok(())
}

#[instrument]
pub fn day23(filename: String, part_b: bool) -> Result<()> {
    let content = fs::read_to_string(filename).context("Couldn't read input")?;

    let mut links: HashMap<String, Vec<String>> = HashMap::new();

    content.lines().for_each(|line| {
        let (l, r) = line.split_once('-').unwrap();
        links
            .entry(l.to_string())
            .and_modify(|e| {
                e.push(r.to_string());
            })
            .or_insert(vec![r.to_string()]);
        links
            .entry(r.to_string())
            .and_modify(|e| {
                e.push(l.to_string());
            })
            .or_insert(vec![l.to_string()]);
    });

    debug!(len = links.len(), "Links Read");

    let mut max_len = 0;
    let mut sets: HashSet<(String, String, String)> = HashSet::new();
    for (key, value) in links.iter() {
        // The most it could be is all the ones in value. For each of them, check how many contain
        // each other.
        let mut values = value.clone();
        let mut done = false;
        while !done {
            done = true;
            for (i, v) in values.clone().iter().enumerate() {
                // Does it contain key?
                let i_list = links.get(v).unwrap_or(&vec![]).clone();
                if !i_list.contains(key) {
                    values.remove(i);
                    done = false;
                    break;
                }
                // Does it contain all the other values?
                for v_other in values.iter() {
                    if v_other == v {
                        continue;
                    }
                    if !i_list.contains(v_other) {
                        values.remove(i);
                        done = false;
                        break;
                    }
                }
                if !done {
                    break;
                }
            }
        }
        values.push(key.clone());
        if values.len() > max_len {
            values.sort();
            info!(v = values.join(","), "New Max");
        }
        max_len = max_len.max(values.len());

        if key.chars().nth(0).unwrap_or('x') != 't' {
            continue;
        }

        debug!(key, ?value, "Checking link");

        // Search the list of connections and list all recipricated connections
        let reverse_connected: Vec<String> = value
            .iter()
            .filter(|&v| {
                let entry: Vec<String> = links.get(v).unwrap_or(&vec![]).clone();
                debug!(len = entry.len(), ?entry, "Got Other Links");
                return entry.contains(key);
            })
            .cloned()
            .collect();

        debug!(len = reverse_connected.len(), "Found Reverse Connections");

        // From the filtered list, find any connected to one another
        for (i, item_i) in reverse_connected.iter().cloned().enumerate() {
            let i_list = links.get(&item_i).unwrap_or(&vec![]).clone();
            for (j, item_j) in reverse_connected.iter().cloned().enumerate() {
                if i == j {
                    continue;
                }
                let j_list = links.get(&item_j).unwrap_or(&vec![]).clone();
                if i_list.contains(&item_j) && j_list.contains(&item_i) {
                    let mut triple = vec![item_i.clone(), item_j.clone(), key.to_string()];
                    triple.sort();

                    sets.insert((triple[0].clone(), triple[1].clone(), triple[2].clone()));
                }
            }
        }
    }

    info!(len = sets.len(), max_len, "Done");

    Ok(())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    info!("Tracing Setup");

    day23("./inputs/day23small.txt".to_string(), false).context("Small Example")?;
    day23("./inputs/day23.txt".to_string(), false).context("Big Example")?;

    day23("./inputs/day23smallb.txt".to_string(), true).context("Small Example")?;
    day23("./inputs/day23.txt".to_string(), true).context("Big Example")?;

    Ok(())
}
