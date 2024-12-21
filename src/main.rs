use core::panic;
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
    max_len: Option<i32>,
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

        if let Some(x) = max_len {
            if score > x {
                continue;
            }
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
    let mut not_walls: HashSet<(i32, i32)> = HashSet::new();
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

    let original_scores = shortest_distance(start, end, &walls, rows, cols, false, None);
    let original_scores_reverse = shortest_distance(end, start, &walls, rows, cols, false, None);
    let original_time = original_scores.get(&end).context("No Path Found")?;

    let mut cheat_routes: HashSet<((i32, i32), (i32, i32))> = HashSet::new();

    let max_d;

    if part_b {
        max_d = 20;
    } else {
        max_d = 2;
    }

    // Trace the shortest route
    // For each step, search a manhattan circle of max_d for any spots that save time
    // Sum them
    for (point, point_score) in original_scores.iter() {
        for dr in -max_d..=max_d {
            for dc in -max_d..=max_d {
                if ((dr as i32).abs() + (dc as i32).abs() > max_d)
                    || !is_in_bounds(rows, cols, point.0 + dr, point.1 + dc)
                    || walls.contains(&(point.0 + dr, point.1 + dc))
                {
                    continue;
                }

                let steps = dr.abs() + dc.abs();
                let test_point = (point.0 + dr, point.1 + dc);
                let test_score = original_scores_reverse
                    .get(&test_point)
                    .context("No Test Point Score")?;
                let new_distance = point_score + steps + test_score;
                if original_time - new_distance >= 100 {
                    debug!(?point, ?test_point, point_score, steps, "Got One");
                    cheat_routes.insert((*point, test_point));
                }
            }
        }
    }

    /*
        for wall in walls.iter().cloned() {
            let wall_distances = shortest_distance(wall, end, &not_walls, rows, cols, false, None);

            let mut empties_start: Vec<((i32, i32), (i32, i32), i32)> = Vec::new();
            let mut empties_end: Vec<((i32, i32), (i32, i32), i32)> = Vec::new();

            for ((r, c), steps) in wall_distances.iter() {
                if *steps > max_d {
                    continue;
                }

                if is_in_bounds(rows, cols, r + dr, c + dc) && !walls.contains(&(r + dr, c + dc)) {
                    let from_start = original_scores
                        .get(&(r + dr, c + dc))
                        .context("No path from start")?;
                    let from_end = original_scores_reverse
                        .get(&(r + dr, c + dc))
                        .context("No path from end")?;
                    if *steps == 0 {
                        empties_start.push(((*from_start, *from_end), (r + dr, c + dc), *steps));
                    }
                    empties_end.push(((*from_start, *from_end), (r + dr, c + dc), *steps));
                }
            }

            if empties_end.len() > 1 {
                for (i, ((start_i, _), start_pos, _)) in empties_start.iter().enumerate() {
                    for (j, ((_, end_j), end_pos, steps_j)) in empties_end.iter().enumerate() {
                        if i == j {
                            continue;
                        }

                        let diff = original_time - (start_i + end_j + steps_j + 2); // Plus 2 for 'step in' then step out
                        if diff == 74 {
                            if !cheat_routes.contains(&(*start_pos, *end_pos, *steps_j)) {
                                debug!(
                                    diff = diff,
                                    steps = steps_j,
                                    start = start_i,
                                    end = end_j,
                                    og = original_time,
                                    ?start_pos,
                                    ?end_pos,
                                    "diff"
                                );
                            }
                            cheat_routes.insert((*start_pos, *end_pos, *steps_j));
                        }
                    }
                }
            }

        }
    */

    info!(cheats = cheat_routes.len(), "Done");
    Ok(())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    info!("Tracing Setup");

    day20("./inputs/day20small.txt".to_string(), false).context("Small Example")?;
    day20("./inputs/day20.txt".to_string(), false).context("Big Example")?;

    day20("./inputs/day20small.txt".to_string(), true).context("Small Example")?;
    day20("./inputs/day20.txt".to_string(), true).context("Big Example")?;

    Ok(())
}
