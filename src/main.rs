use std::{
    collections::{HashMap, HashSet},
    fs, io,
};

use tracing::{Level, debug, event, info, instrument, warn};
use tracing_subscriber::EnvFilter;

mod done;
mod util;

use crate::done::*;

#[instrument]
pub fn dayxx(filename: String) {
    let content = fs::read_to_string(filename).expect("Couldn't read input");
}

#[derive(Debug)]
enum Tile {
    Wall,
    Box,
    Empty,
}

#[instrument]
pub fn day15(filename: String) {
    let content = fs::read_to_string(filename).expect("Couldn't read input");

    let mut map_parse = true;

    let mut map: HashMap<(i32, i32), Tile> = HashMap::new();
    let mut player: (i32, i32) = (-1, -1);

    let mut directions: Vec<util::Direction> = Vec::new();

    for (row, line) in content.lines().enumerate() {
        if line.len() == 0 {
            map_parse = false;
        }

        if map_parse {
            for (col, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        map.insert((row as i32, col as i32), Tile::Wall);
                    }
                    'O' => {
                        map.insert((row as i32, col as i32), Tile::Box);
                    }
                    '.' => {
                        map.insert((row as i32, col as i32), Tile::Empty);
                    }
                    '@' => {
                        map.insert((row as i32, col as i32), Tile::Empty);
                        player = (row as i32, col as i32);
                    }
                    _ => {}
                };
            }
        } else {
            for c in line.chars() {
                match c {
                    '^' => {
                        directions.push(util::Direction::North);
                    }
                    'v' => {
                        directions.push(util::Direction::South);
                    }
                    '<' => {
                        directions.push(util::Direction::West);
                    }
                    '>' => {
                        directions.push(util::Direction::East);
                    }
                    _ => {
                        warn!(char = ?c, "Found Bad Character");
                    }
                };
            }
        }
    }

    // Execute the instructions
    for dir in directions {
        let test_position = util::move_direction(&player, &dir);
        debug!(player=?player, test_position=?test_position, dir=?dir, "Attempting to Move");

        // If the direction is empty, just move.
        match map.get(&test_position) {
            None => {
                continue;
            }
            Some(x) => match *x {
                Tile::Wall => {
                    debug!("Hit a Wall");
                    continue;
                }
                Tile::Empty => {
                    debug!("Moved to Empty");
                    player = test_position;
                    continue;
                }
                Tile::Box => {
                    debug!("Found a Box");
                    // Keep moving in the same direction until you reach either an empty space or a
                    // wall. Noting how many boxes you pass on the way.
                    let mut search_position = util::move_direction(&test_position, &dir);
                    let mut search_value = map.get(&search_position);
                    let mut can_move = true;

                    while let Some(x) = search_value {
                        match x {
                            Tile::Wall => {
                                can_move = false;
                                break;
                            }
                            Tile::Box => {}
                            Tile::Empty => {
                                break;
                            }
                        };
                        search_position = util::move_direction(&search_position, &dir);
                        search_value = map.get(&search_position);
                    }
                    if can_move {
                        // Search position will be an empty space, put a box there, move the player
                        // once.
                        map.insert(search_position, Tile::Box);

                        map.insert(test_position, Tile::Empty);

                        player = test_position;

                        event!(Level::DEBUG, from = ?test_position, to = ?search_position, "Moved Box");
                    }
                    continue;
                }
            },
        }
    }

    let mut total = 0;
    for ((r, c), value) in map.iter() {
        if matches!(value, Tile::Box) {
            total += 100 * r + c;
        }
    }

    info!("Final Score {:?}", total);
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    info!("Tracing Setup");

    println!("day 15");
    day15("./inputs/day15tiny.txt".to_string());
    day15("./inputs/day15small.txt".to_string());
    day15("./inputs/day15.txt".to_string());
}
