use std::{
    collections::{HashMap, HashSet},
    fs, i32, io,
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

fn shortest_distance(
    start: (i32, i32),
    end: (i32, i32),
    blockers: &HashSet<(i32, i32)>,
    rows: i32,
    cols: i32,
    shortcut: bool,
) -> HashMap<(i32, i32), i32> {
    let mut scores: HashMap<(i32, i32), i32> = HashMap::new();
    let mut front: Vec<((i32, i32), i32)> = Vec::new();

    scores.insert(start, 0);
    front.push((start, 0));

    while let Some((pos, score)) = front.pop() {
        if (pos == end) && shortcut {
            scores.insert(pos, score);
            return scores;
        }
        for d in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            let test_pos = move_direction(&pos, &d);

            if !is_in_bounds(rows, cols, test_pos.0, test_pos.1) {
                continue;
            }

            if !blockers.contains(&test_pos) {
                let current_score = scores.get(&test_pos).unwrap_or(&(score + 2)).clone();
                if score + 1 < current_score {
                    scores.insert(test_pos, score + 1);
                    front.push((test_pos, score + 1));
                }
            }
        }
        front.sort_by_key(|x| -x.1);
    }

    return scores;
}

#[instrument]
pub fn day20(filename: String, part_b: bool) -> Result<()> {
    let content = fs::read_to_string(filename).context("Couldn't read input")?;

    let mut walls: HashSet<(i32, i32)> = HashSet::new();
    let mut start: (i32, i32) = (0, 0);
    let mut end: (i32, i32) = (0, 0);

    let mut rows: i32 = 0;
    let mut cols: i32 = 0;

    let mut wall_list: Vec<(i32, i32)> = Vec::new();

    for (row, line) in content.lines().enumerate() {
        rows += 1;
        cols = line.len() as i32;
        for (col, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    walls.insert((row as i32, col as i32));
                    wall_list.push((row as i32, col as i32));
                }
                'S' => {
                    start = (row as i32, col as i32);
                }
                'E' => {
                    end = (row as i32, col as i32);
                }
                _ => {}
            };
        }
    }

    let original_scores = shortest_distance(start, end, &walls, rows, cols, false);
    let original_scores_reverse = shortest_distance(end, start, &walls, rows, cols, false);
    let original_time = original_scores.get(&end).context("No Path Found")?;

    let mut cheats = 0;

    for (r, c) in walls.iter().cloned() {
        let mut empties: Vec<(i32, i32)> = Vec::new();
        for (dr, dc) in LURD {
            if is_in_bounds(rows, cols, r + dr, c + dc) && !walls.contains(&(r + dr, c + dc)) {
                let from_start = original_scores
                    .get(&(r + dr, c + dc))
                    .context("No path from start")?;
                let from_end = original_scores_reverse
                    .get(&(r + dr, c + dc))
                    .context("No path from end")?;
                empties.push((*from_start, *from_end));
            }
        }

        let mut found = false;
        if empties.len() > 1 {
            for (i, (start_i, _)) in empties.iter().enumerate() {
                for (j, (_, end_j)) in empties.iter().enumerate() {
                    if i == j {
                        continue;
                    }

                    let diff = original_time - (start_i + end_j + 1);
                    if diff >= 100 {
                        found = true;
                    }
                }
            }
        }

        if found {
            cheats += 1;
        }
    }

    info!(cheats = cheats, "Done");
    Ok(())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    info!("Tracing Setup");

    day20("./inputs/day20small.txt".to_string(), false).context("Small Example")?;
    day20("./inputs/day20.txt".to_string(), false).context("Big Example")?;

    Ok(())
}
