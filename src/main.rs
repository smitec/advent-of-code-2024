use std::{collections::HashMap, fs, iter::zip};

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
    day1();
}
