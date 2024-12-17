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

fn display_map(walls: &HashSet<(i32, i32)>, spots: &HashSet<(i32, i32)>, rows: i32, cols: i32) {
    for row in 0..rows {
        for col in 0..cols {
            if walls.contains(&(row, col)) {
                print!("#");
            } else if spots.contains(&(row, col)) {
                print!("O");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

#[instrument]
pub fn day16(filename: String, part_b: bool) {
    let content = fs::read_to_string(filename).expect("Couldn't read input");

    let mut walls: HashSet<(i32, i32)> = HashSet::new();
    let mut start: (i32, i32) = (0, 0);
    let mut end: (i32, i32) = (0, 0);

    let mut rows: i32 = 0;
    let mut cols: i32 = 0;

    for (row, line) in content.lines().enumerate() {
        rows += 1;
        cols = line.len() as i32;
        for (col, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    walls.insert((row as i32, col as i32));
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

    let mut scores: HashMap<(i32, i32), i32> = HashMap::new();
    let mut spots: HashSet<(i32, i32)> = HashSet::new();
    let mut front: Vec<((i32, i32), Direction, i32, HashSet<(i32, i32)>)> = Vec::new();

    scores.insert(start, 0);
    let mut start_spots: HashSet<(i32, i32)> = HashSet::new();
    start_spots.insert(start);
    front.push((start, Direction::East, 0, start_spots));

    // TODO: There are multiple ways to step and turn with different scores at different points
    // Storing only the absolute minimum is not going to cut it. Maybe need to be more exhaustive.

    while let Some(next) = front.pop() {
        // Straight
        let next_spot = move_direction(&next.0, &next.1);
        if !walls.contains(&next_spot) {
            let current_score: i32 = *scores.get(&next_spot).unwrap_or(&(next.2 + 2));
            if next.2 + 1 <= current_score {
                scores.insert(next_spot, next.2 + 1);

                let mut new_spots = next.3.clone();
                new_spots.insert(next_spot);

                if next_spot == end {
                    if next.2 + 1 == current_score {
                        spots.extend(&new_spots);
                    } else {
                        spots = new_spots.clone();
                    }
                }

                front.insert(0, (next_spot, next.1.clone(), next.2 + 1, new_spots));
            }
        }

        // Left & Right
        for turning in [Rotation::Left, Rotation::Right] {
            let test_direction = turn(&next.1, turning);
            let next_spot = move_direction(&next.0, &test_direction);
            if !walls.contains(&next_spot) {
                let current_score: i32 = *scores.get(&next_spot).unwrap_or(&(next.2 + 1002));
                if next.2 + 1001 <= current_score {
                    scores.insert(next_spot, next.2 + 1001);

                    let mut new_spots = next.3.clone();
                    new_spots.insert(next_spot);

                    if next_spot == end {
                        if next.2 + 1001 == current_score {
                            spots.extend(&new_spots);
                        } else {
                            spots = new_spots.clone();
                        }
                    }

                    // front.insert(0, (next_spot, test_direction, next.2 + 1001, new_spots));
                    front.push((next_spot, test_direction, next.2 + 1001, new_spots));
                }
            }
        }

        //front.sort_by_key(|x| x.2);
    }

    display_map(&walls, &spots, rows, cols);

    info!(score=?scores.get(&end), spots=?spots.len(), "Finished Pathfinding");
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    info!("Tracing Setup");

    day16("./inputs/day16small.txt".to_string(), true);
    day16("./inputs/day16.txt".to_string(), true);
}
