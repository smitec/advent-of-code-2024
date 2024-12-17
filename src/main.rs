use std::{
    collections::{HashMap, HashSet},
    fs, io,
};

use tracing::{Level, debug, error, event, info, instrument, warn};
use tracing_subscriber::EnvFilter;

mod done;
mod util;

use crate::done::*;

#[instrument]
pub fn dayxx(filename: String) {
    let content = fs::read_to_string(filename).expect("Couldn't read input");
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Wall,
    Box,
    Empty,
    BoxLeft,
    BoxRight,
}

fn display_map(map: &HashMap<(i32, i32), Tile>, rows: i32, cols: i32, player: (i32, i32)) {
    for row in 0..rows {
        for col in 0..cols {
            if player.0 == row && player.1 == col {
                print!("@");
            } else {
                if let Some(x) = map.get(&(row, col)) {
                    match x {
                        Tile::Wall => print!("#"),
                        Tile::Box => print!("O"),
                        Tile::Empty => print!("."),
                        Tile::BoxLeft => print!("["),
                        Tile::BoxRight => print!("]"),
                    };
                } else {
                    print!("?");
                }
            }
        }
        print!("\n");
    }
}

#[instrument]
pub fn day15(filename: String, part_b: bool) {
    let content = fs::read_to_string(filename).expect("Couldn't read input");

    let mut map_parse = true;

    let mut map: HashMap<(i32, i32), Tile> = HashMap::new();
    let mut player: (i32, i32) = (-1, -1);

    let mut directions: Vec<util::Direction> = Vec::new();

    let mut rows = 0;
    let mut cols = 0;

    for (row, line) in content.lines().enumerate() {
        if line.len() == 0 {
            map_parse = false;
        }

        if map_parse {
            rows += 1;
            cols = line.len() as i32;
            for (col, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        if part_b {
                            map.insert((row as i32, 2 * col as i32), Tile::Wall);
                            map.insert((row as i32, 2 * col as i32 + 1), Tile::Wall);
                        } else {
                            map.insert((row as i32, col as i32), Tile::Wall);
                        }
                    }
                    'O' => {
                        if part_b {
                            map.insert((row as i32, 2 * col as i32), Tile::BoxLeft);
                            map.insert((row as i32, 2 * col as i32 + 1), Tile::BoxRight);
                        } else {
                            map.insert((row as i32, col as i32), Tile::Box);
                        }
                    }
                    '.' => {
                        if part_b {
                            map.insert((row as i32, 2 * col as i32), Tile::Empty);
                            map.insert((row as i32, 2 * col as i32 + 1), Tile::Empty);
                        } else {
                            map.insert((row as i32, col as i32), Tile::Empty);
                        }
                    }
                    '@' => {
                        if part_b {
                            map.insert((row as i32, 2 * col as i32), Tile::Empty);
                            map.insert((row as i32, 2 * col as i32 + 1), Tile::Empty);
                            player = (row as i32, 2 * col as i32);
                        } else {
                            map.insert((row as i32, col as i32), Tile::Empty);
                            player = (row as i32, col as i32);
                        }
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
        //display_map(&map, rows, cols * 2, player);

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
                Tile::BoxLeft | Tile::BoxRight | Tile::Box => {
                    debug!("Found a Box");
                    // Keep moving in the same direction until you reach either an empty space or a
                    // wall. Noting how many boxes you pass on the way.
                    let mut search_positions: Vec<(i32, i32)> = Vec::new();

                    let mut can_move = true;

                    // for part b vertical movement, need to store a collection of left and right
                    // pieces. For each, check they are clear above, when they are, put them into
                    // another list of 'moveable' pieces. If the potentially moveable list is
                    // exhasted and all end up in moveable, move them.
                    let search_position = util::move_direction(&test_position, &dir);
                    search_positions.push(search_position);

                    if part_b {
                        // If part b and moving up or down, add the other half of the box
                        if matches!(dir, util::Direction::North | util::Direction::South) {
                            if matches!(x, Tile::BoxLeft) {
                                search_positions.push((search_position.0, search_position.1 + 1));
                            } else if matches!(x, Tile::BoxRight) {
                                search_positions.push((search_position.0, search_position.1 - 1));
                            } else {
                                error!(
                                    search_position = ?search_position,
                                    "Got Something other than a box half!"
                                );
                            }
                        }
                    }

                    let mut to_fill: Vec<(i32, i32)> = Vec::new();
                    while search_positions.len() > 0 {
                        let mut new_positions: Vec<(i32, i32)> = Vec::new();
                        for search_position in search_positions.iter().cloned() {
                            let search_value = map.get(&search_position);
                            if let Some(x) = search_value {
                                match x {
                                    Tile::Wall => {
                                        debug!("Found a Wall, Can't Move");
                                        can_move = false;
                                        break;
                                    }
                                    Tile::Box => {}
                                    Tile::BoxLeft => {
                                        if matches!(
                                            dir,
                                            util::Direction::North | util::Direction::South
                                        ) {
                                            // Add the area above the box to the list to check
                                            // Don't keep checking this one
                                            let new_search_position =
                                                util::move_direction(&search_position, &dir);
                                            new_positions.push(new_search_position);
                                            new_positions.push((
                                                new_search_position.0,
                                                new_search_position.1 + 1,
                                            ));
                                            to_fill.push(search_position);
                                            continue;
                                        }
                                    }
                                    Tile::BoxRight => {
                                        if matches!(
                                            dir,
                                            util::Direction::North | util::Direction::South
                                        ) {
                                            // Add the area above the box to the list to check
                                            // Don't keep checking this one
                                            /*let new_search_position = util::move_direction(
                                                &util::move_direction(&search_position, &dir),
                                                &dir,
                                            );*/
                                            let new_search_position =
                                                util::move_direction(&search_position, &dir);
                                            new_positions.push(new_search_position);
                                            new_positions.push((
                                                new_search_position.0,
                                                new_search_position.1 - 1,
                                            ));
                                            to_fill.push(search_position);
                                            continue;
                                        }
                                    }
                                    Tile::Empty => {
                                        // Add this spot to the list to fill, don't keep checking
                                        to_fill.push(search_position);
                                        if matches!(
                                            dir,
                                            util::Direction::North | util::Direction::South
                                        ) {
                                            continue;
                                        } else {
                                            break; // TODO: Unsure
                                        }
                                    }
                                };
                                let new_search_position =
                                    util::move_direction(&search_position, &dir);
                                new_positions.push(new_search_position);
                            }
                        }
                        search_positions = new_positions;
                    }

                    if can_move {
                        // Search position will be an empty space, put a box there, move the player
                        // once.
                        if part_b {
                            // Shuffle all the boxes
                            if matches!(dir, util::Direction::East | util::Direction::West) {
                                if to_fill.len() != 1 {
                                    error!(len=?to_fill.len(),"More than one to_fill found");
                                }
                                let search_position = to_fill[0];
                                let first_spot = util::move_direction(&test_position, &dir);
                                let lower = search_position.1.min(first_spot.1);
                                let upper = search_position.1.max(first_spot.1);
                                let mut left = true;
                                for new_col in lower..=upper {
                                    if left {
                                        map.insert((search_position.0, new_col), Tile::BoxLeft);
                                    } else {
                                        map.insert((search_position.0, new_col), Tile::BoxRight);
                                    }
                                    left = !left;
                                }
                            } else {
                                // Move all the boxes in the "move list"
                                // First, get a list of squares to clear
                                debug!(len = to_fill.len(), "Moving Box Parts");
                                let mut to_clear: Vec<(i32, i32)> = Vec::new();
                                let mut to_fill_values: HashMap<(i32, i32), Tile> = HashMap::new();
                                for spot in to_fill.iter().cloned() {
                                    let clear_spot = util::move_direction(
                                        &spot,
                                        &util::opposite_direction(&dir),
                                    );
                                    to_fill_values.insert(spot, *map.get(&clear_spot).unwrap());
                                    to_clear.push(clear_spot);
                                }

                                for clear in to_clear {
                                    map.insert(clear, Tile::Empty);
                                }

                                for spot in to_fill {
                                    map.insert(spot, *to_fill_values.get(&spot).unwrap());
                                }
                            }
                        } else {
                            map.insert(search_position, Tile::Box);
                        }

                        map.insert(test_position, Tile::Empty);

                        player = test_position;

                        event!(Level::DEBUG, from = ?test_position, to = ?search_position, "Moved Box(s)");
                    }

                    continue;
                }
            },
        }
    }

    display_map(&map, rows, cols * 2, player);

    let mut total = 0;
    for ((r, c), value) in map.iter() {
        if matches!(value, Tile::Box) {
            total += 100 * r + c;
        } else if matches!(value, Tile::BoxLeft) {
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
    //day15("./inputs/day15tiny.txt".to_string(), true);
    day15("./inputs/day15tiny_b.txt".to_string(), true);
    day15("./inputs/day15small.txt".to_string(), true);
    day15("./inputs/day15.txt".to_string(), true);
}
