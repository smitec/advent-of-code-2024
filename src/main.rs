use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs,
    iter::zip,
    usize,
};

use regex::Regex;
use tracing::{debug, info, instrument};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Direction {
    North,
    East,
    West,
    South,
}

#[derive(Debug)]
enum Rotation {
    Left,
    Right,
}

#[instrument]
fn move_direction(start: &(i32, i32), direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::North => (start.0 - 1, start.1),
        Direction::East => (start.0, start.1 + 1),
        Direction::West => (start.0, start.1 - 1),
        Direction::South => (start.0 + 1, start.1),
    }
}

#[instrument]
fn turn(direction: &Direction, rotation: Rotation) -> Direction {
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

#[derive(Eq, Hash, PartialEq)]
struct Bearing {
    pos: (i32, i32),
    d: Direction,
}

#[instrument]
fn is_in_bounds(rows: i32, cols: i32, row: i32, col: i32) -> bool {
    if row < 0 || col < 0 {
        return false;
    }

    if row >= rows || col >= cols {
        return false;
    }

    true
}

#[instrument]
fn day6() {
    let content = fs::read_to_string("./inputs/day6.txt").expect("Couldn't read input");

    let mut map: HashSet<(i32, i32)> = HashSet::new();
    let mut rows: i32 = 0;
    let mut cols: i32 = 0;

    let mut position: Bearing = Bearing {
        pos: (-1, -1),
        d: Direction::North,
    };

    for line in content.lines() {
        cols = line.len() as i32;
        for (i, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    map.insert((rows, i as i32));
                }
                '^' => {
                    position = Bearing {
                        pos: (rows, i as i32),
                        d: Direction::North,
                    };
                }
                _ => {}
            };
        }
        rows += 1;
    }

    let mut visited_pos: HashSet<(i32, i32)> = HashSet::new();

    loop {
        // Try Move
        let mut new_pos = move_direction(&position.pos, &position.d);

        let new_direction: Direction;
        if map.contains(&new_pos) {
            // Turn Right but don't move
            new_direction = turn(&position.d, Rotation::Right);
            new_pos = position.pos.clone();
        } else {
            new_direction = position.d.clone();
        }

        if !is_in_bounds(rows, cols, position.pos.0, position.pos.1) {
            break;
        }

        visited_pos.insert(position.pos.clone());

        position = Bearing {
            pos: new_pos,
            d: new_direction,
        };
    }

    info!("Total Visited Locations {:?}", visited_pos.len());
}

fn day5() {
    let content = fs::read_to_string("./inputs/day5.txt").expect("Couldn't read input");

    // Constraints in the form X|Y
    // <blank line>
    // Comma Separated Values
    //
    // X must be left of Y in the lists.
    // i.e. Fail if Y is right of X
    // Store a list of all values not allowed to be right of X

    let mut constraints: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut lists: Vec<Vec<i32>> = Vec::new();
    let mut parsing_contstraints = true;

    for line in content.lines() {
        if line.len() == 0 {
            parsing_contstraints = false;
            continue;
        }

        if parsing_contstraints {
            let (a, b) = line.split_once('|').unwrap();
            let a_p: i32 = a.parse().unwrap();
            let b_p: i32 = b.parse().unwrap();

            constraints
                .entry(a_p)
                .and_modify(|v| v.push(b_p))
                .or_insert(vec![b_p]);
        } else {
            lists.push(line.split(',').map(|x| x.parse().unwrap()).collect());
        }
    }

    let mut total = 0;
    let mut b_total = 0;

    for list in lists {
        let mut clean = true;
        for (i, item) in list.iter().enumerate() {
            let d = constraints.get(&item);
            if let Some(right) = d {
                for check in right {
                    if list[..i].contains(check) {
                        clean = false;
                        break;
                    }
                }
            }
            if !clean {
                break;
            }
        }
        if clean {
            let half = (list.len() as f32 / 2.0).floor();
            total += list[half as usize];
        } else {
            let mut sorted = list.clone();
            sorted.sort_by(|a, b| {
                // Check for Greater
                let d = constraints.get(a);
                if let Some(right) = d {
                    if right.contains(b) {
                        return Ordering::Greater;
                    }
                }

                // Check for Lesser
                let d = constraints.get(b);
                if let Some(right) = d {
                    if right.contains(a) {
                        return Ordering::Less;
                    }
                }

                Ordering::Equal
            });
            let half = (sorted.len() as f32 / 2.0).floor();
            b_total += sorted[half as usize];
        }
    }

    info!("Final Total Part A {:?}", total);
    info!("Final Total Part B {:?}", b_total);
}

struct XmasBit {
    c: char,
    row: i32,
    col: i32,
    dr: i32,
    dc: i32,
}

fn day4() {
    let content = fs::read_to_string("./inputs/day4.txt").expect("Couldn't read input");

    let mut lines: Vec<Vec<char>> = Vec::new();
    let mut to_check: Vec<XmasBit> = Vec::new();
    let mut to_check_b: Vec<XmasBit> = Vec::new();
    let mut rows = 0;
    let mut cols = 0;

    let rcs = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, 0),
        (1, -1),
    ];

    for line in content.lines() {
        cols = 0;
        lines.push(
            line.chars()
                .map(|x| {
                    if x == 'X' {
                        rcs.iter().for_each(|e| {
                            let (dr, dc) = e;
                            to_check.push(XmasBit {
                                c: 'X',
                                row: rows,
                                col: cols,
                                dr: *dr,
                                dc: *dc,
                            });
                        });
                    } else if x == 'A' {
                        to_check_b.push(XmasBit {
                            c: 'A',
                            row: rows,
                            col: cols,
                            dr: 0,
                            dc: 0,
                        });
                    }
                    cols += 1;
                    x
                })
                .collect(),
        );
        rows += 1;
    }

    let mut found = 0;

    while let Some(elem) = to_check.pop() {
        // Check Bounds
        let new_r = elem.row + elem.dr;
        let new_c = elem.col + elem.dc;

        if !is_in_bounds(rows, cols, new_r, new_c) {
            continue;
        }

        // Check Letter
        let letter = lines[new_r as usize][new_c as usize];
        match letter {
            'M' => {
                if elem.c == 'X' {
                    to_check.push(XmasBit {
                        c: 'M',
                        row: new_r,
                        col: new_c,
                        dr: elem.dr,
                        dc: elem.dc,
                    });
                }
            }
            'A' => {
                if elem.c == 'M' {
                    to_check.push(XmasBit {
                        c: 'A',
                        row: new_r,
                        col: new_c,
                        dr: elem.dr,
                        dc: elem.dc,
                    });
                }
            }
            'S' => {
                if elem.c == 'A' {
                    found += 1;
                }
            }
            _ => continue,
        };
    }

    println!("{:?}", found);

    let mut found_b = 0;

    while let Some(a) = to_check_b.pop() {
        let mut diagonals = 0;
        // Down Right
        let both_offsets = vec![vec![(-1, -1), (1, 1)], vec![(-1, 1), (1, -1)]];
        for offsets in both_offsets {
            let mut items: String = "".to_string();
            for offset in offsets {
                let new_r = a.row + offset.0;
                let new_c = a.col + offset.1;
                if is_in_bounds(rows, cols, new_r, new_c) {
                    items.push(lines[new_r as usize][new_c as usize]);
                }
            }

            if items == "MS" || items == "SM" {
                diagonals += 1;
            }
        }
        if diagonals == 2 {
            found_b += 1;
        }
    }

    println!("{:?}", found_b);
}

fn day3() {
    let content = fs::read_to_string("./inputs/day3.txt").expect("Couldn't read input");

    let parent =
        Regex::new(r#"(?<do>do\(\))|(?<mul>mul\([0-9]+,[0-9]+\))|(?<dont>don't\(\))"#).unwrap();
    let matcher = Regex::new(r#"mul\(([0-9]+),([0-9]+)\)"#).unwrap();

    let mut total = 0;
    let mut on = true;

    for line in content.lines() {
        let _ = parent.captures_iter(line).for_each(|caps| {
            if let Some(_) = caps.name("do") {
                on = true;
            } else if let Some(_) = caps.name("dont") {
                on = false
            } else if let Some(x) = caps.name("mul") {
                if on {
                    let cap = matcher.captures(x.as_str()).unwrap();
                    let (_, [a, b]) = cap.extract();
                    let ai: i32 = a.parse().unwrap();
                    let bi: i32 = b.parse().unwrap();
                    total += ai * bi;
                }
            }
        });
    }

    println!("{:?}", total);
}

#[derive(Clone)]
enum LevelState {
    Unknown,
    Increasing,
    Decreasing,
}

enum CheckState {
    Match,
    FailAt(usize),
}

fn check(items: Vec<i32>) -> CheckState {
    let mut state: LevelState = LevelState::Unknown;
    let mut new_state = LevelState::Unknown;
    let mut previous: Option<i32> = None;

    for (i, val) in items.iter().enumerate() {
        match state {
            LevelState::Unknown => {
                if let Some(x) = previous {
                    if *val > x {
                        new_state = LevelState::Increasing;
                    } else if *val < x {
                        new_state = LevelState::Decreasing;
                    } else {
                        return CheckState::FailAt(i);
                    }
                }
            }
            LevelState::Increasing => {
                if let Some(x) = previous {
                    if *val <= x {
                        return CheckState::FailAt(i);
                    }
                }
            }
            LevelState::Decreasing => {
                if let Some(x) = previous {
                    if *val >= x {
                        return CheckState::FailAt(i);
                    }
                }
            }
        }

        if let Some(x) = previous {
            if (*val - x).abs() > 3 {
                return CheckState::FailAt(i);
            }
        }

        previous = Some(*val);
        state = new_state.clone();
    }

    return CheckState::Match;
}

fn day2() {
    let content = fs::read_to_string("./inputs/day2.txt").expect("Couldn't read input");

    let mut safe = 0;

    for line in content.lines() {
        let items: Vec<i32> = line.split(" ").map(|x| x.parse().unwrap()).collect();
        match check(items.clone()) {
            CheckState::Match => {
                safe += 1;
            }
            CheckState::FailAt(i) => {
                for a in 0..items.len() {
                    let mut rest = items.clone();
                    rest.remove(a);
                    if let CheckState::Match = check(rest) {
                        safe += 1;
                        break;
                    }
                }
            }
        }
    }

    println!("{}", safe);
}

fn day1() {
    let content = fs::read_to_string("./inputs/day1.txt").expect("Couldn't read input");

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let mut right_map: HashMap<i32, i32> = HashMap::new();

    for line in content.lines() {
        let (l, r) = line.split_once("   ").unwrap();
        left.push(l.parse().unwrap());
        right.push(r.parse().unwrap());
        right_map
            .entry(r.parse().unwrap())
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    let mut total = 0;

    left.sort();
    right.sort();

    for (a, b) in zip(left.clone(), right) {
        total += (b - a).abs();
    }

    println!("{:?}", total);

    let mut total_two = 0;

    for v in left.clone() {
        if let Some(x) = right_map.get(&v) {
            total_two += x * v;
        }
    }

    println!("{:?}", total_two);
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    info!("Tracing Setup");
    /*
    println!("day 1");
    day1();
    println!("day 2");
    day2();
    println!("day 3");
    day3();
    println!("day 4");
    day4();
    println!("day 5");
    day5();
    */
    println!("day 6");
    day6();
}
