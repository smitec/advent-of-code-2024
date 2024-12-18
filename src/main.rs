use std::{
    collections::{HashMap, HashSet},
    fs, io,
};

use tracing::{Level, debug, error, event, info, instrument, warn};
use tracing_subscriber::EnvFilter;

mod done;
mod util;

use crate::done::*;
use crate::util::*;

#[instrument]
pub fn dayxx(filename: String, part_b: bool) {
    let content = fs::read_to_string(filename).expect("Couldn't read input");
}

fn display_18_map(map: &HashSet<(i32, i32)>, size: i32) {
    for r in 0..=size {
        for c in 0..=size {
            if map.contains(&(r, c)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

#[instrument]
pub fn day18(filename: String, part_b: bool, size: i32, steps: usize, first_jump: i32) {
    let content = fs::read_to_string(filename).expect("Couldn't read input");

    let mut position: (i32, i32) = (0, 0);
    let mut goal: (i32, i32) = (size, size);

    let mut blockers: Vec<(i32, i32)> = Vec::new();

    for (r, line) in content.lines().enumerate() {
        let (col, row) = line.split_once(',').unwrap();
        blockers.push((row.parse::<i32>().unwrap(), col.parse::<i32>().unwrap()));
    }

    let mut steps = steps;
    let mut jump = first_jump;
    loop {
        // Path find on a changing map. Could work with an exhaustive search.
        let mut front: Vec<((i32, i32), i32)> = Vec::new();
        let mut scores: HashMap<(i32, i32), i32> = HashMap::new();
        front.push((position, 0)); // Store position + time
        scores.insert(position, 0);

        // Simulate X steps
        let mut map: HashSet<(i32, i32)> = HashSet::new();
        for t in 0..steps {
            map.insert(blockers[t]);
        }

        // display_18_map(&map, size);

        while let Some((pos, distance)) = front.pop() {
            let mut early = false;
            for dir in [
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ] {
                let test = move_direction(&pos, &dir);
                if is_in_bounds(size + 1, size + 1, test.0, test.1) && !map.contains(&test) {
                    let current = scores.get(&test).unwrap_or(&(distance + 2)).clone();
                    if distance + 1 < current {
                        front.push((test, distance + 1));
                        scores.insert(test, distance + 1);
                        if test == goal {
                            early = true;
                            break;
                        }
                    }
                }
            }
            if early {
                break;
            }
            front.sort_by_key(|x| x.1);
        }

        if let None = scores.get(&goal) {
            if jump > 1 {
                debug!(steps = jump, "little jump");
                steps -= 2 * jump as usize;
                jump = 1;
            } else {
                info!(distance = steps, blocker = ?blockers[steps-1], "Finished");
                break;
            }
        } else {
            debug!(steps = steps, "Continuing");
            steps += jump as usize;
        }
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    info!("Tracing Setup");

    day18("./inputs/day18small.txt".to_string(), false, 6, 12, 1);
    day18("./inputs/day18.txt".to_string(), false, 70, 1024, 100);
}
