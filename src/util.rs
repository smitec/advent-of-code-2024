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
