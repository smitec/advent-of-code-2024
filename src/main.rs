use std::{collections::HashMap, fs, iter::zip};

use regex::Regex;

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
    println!("day 1");
    day1();
    println!("day 2");
    day2();
    println!("day 3");
    day3();
}
