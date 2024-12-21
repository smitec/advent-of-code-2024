use std::collections::{HashMap, HashSet};

use tracing::instrument;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Direction {
    North,
    East,
    West,
    South,
}

#[derive(Debug)]
pub enum Rotation {
    Left,
    Right,
}

#[instrument]
pub fn opposite_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::South,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
        Direction::South => Direction::North,
    }
}

#[instrument]
pub fn move_direction(start: &(i32, i32), direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::North => (start.0 - 1, start.1),
        Direction::East => (start.0, start.1 + 1),
        Direction::West => (start.0, start.1 - 1),
        Direction::South => (start.0 + 1, start.1),
    }
}

#[instrument]
pub fn turn(direction: &Direction, rotation: Rotation) -> Direction {
    match direction {
        Direction::North => match rotation {
            Rotation::Left => {
                return Direction::West;
            }
            Rotation::Right => {
                return Direction::East;
            }
        },
        Direction::East => match rotation {
            Rotation::Left => {
                return Direction::North;
            }
            Rotation::Right => {
                return Direction::South;
            }
        },
        Direction::West => match rotation {
            Rotation::Left => {
                return Direction::South;
            }
            Rotation::Right => {
                return Direction::North;
            }
        },
        Direction::South => match rotation {
            Rotation::Left => {
                return Direction::East;
            }
            Rotation::Right => {
                return Direction::West;
            }
        },
    }
}

#[instrument]
pub fn is_in_bounds(rows: i32, cols: i32, row: i32, col: i32) -> bool {
    if row < 0 || col < 0 {
        return false;
    }

    if row >= rows || col >= cols {
        return false;
    }

    true
}

pub fn shortest_distance(
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
