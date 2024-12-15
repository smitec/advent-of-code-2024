use crate::util::*;

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs, io,
    iter::zip,
    usize,
};

use regex::Regex;
use tracing::{debug, info, instrument};

#[derive(Clone)]
struct Robot {
    pr: i32,
    pc: i32,
    vr: i32,
    vc: i32,
}

#[instrument]
pub fn day14(filename: String, rows: i32, cols: i32) {
    let content = fs::read_to_string(filename).expect("Couldn't read input");

    let mut robots: Vec<Robot> = Vec::new();

    for line in content.lines() {
        let (pos, velocity) = line.split_once(' ').unwrap();

        let pos = &pos.replace("p=", "");
        let velocity = &velocity.replace("v=", "");

        let (pc, pr) = pos.split_once(',').unwrap();
        let (vc, vr) = velocity.split_once(',').unwrap();

        robots.push(Robot {
            pr: pr.parse().unwrap(),
            pc: pc.parse().unwrap(),
            vr: vr.parse().unwrap(),
            vc: vc.parse().unwrap(),
        });
    }

    debug!("Got {:?} Robots", robots.len());
    debug!("Half Points r={:?} c={:?}", rows / 2, cols / 2);

    let mut factors = vec![0, 0, 0, 0];
    robots.clone().into_iter().for_each(|robot| {
        let f_r = (robot.pr + 100 * robot.vr).rem_euclid(rows);
        let f_c = (robot.pc + 100 * robot.vc).rem_euclid(cols);

        debug!("fr={:?} fc={:?}", f_r, f_c);

        let mut i = 0;

        if !(f_r == rows / 2 || f_c == cols / 2) {
            if f_c > cols / 2 {
                i = 1;
            }

            if f_r > rows / 2 {
                i += 2;
            }

            factors[i] += 1;
        }
    });

    let mut total = 1;
    debug!("Factors: {:?}", factors);
    for v in factors {
        total *= v;
    }

    info!("Final Factors {:?}", total);

    let mut display: HashSet<(i32, i32)> = HashSet::new();
    let mut steps = 0;
    let mut buffer = String::new();
    let stdin = io::stdin();

    while steps < 50000 {
        /*
                stdin.read_line(&mut buffer).unwrap();
                if buffer.trim() == "a" {
                    break;
                }
        */
        steps += 1;
        display = HashSet::new();
        let mut double = false;
        robots.clone().into_iter().for_each(|robot| {
            let f_r = (robot.pr + steps * robot.vr).rem_euclid(rows);
            let f_c = (robot.pc + steps * robot.vc).rem_euclid(cols);
            if display.contains(&(f_r, f_c)) {
                double = true;
            }
            display.insert((f_r, f_c));
        });
        if double {
            continue;
        }
        println!("Step {:?}", steps);
        for c in 0..cols {
            for r in 0..rows {
                if display.contains(&(r, c)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
    }
}

struct PrizeMatrix {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    p_x: i64,
    p_y: i64,
}

#[instrument]
pub fn day13(filename: String) {
    let content = fs::read_to_string(filename).expect("Couldn't read input");

    let mut machine = PrizeMatrix {
        a_x: 0,
        a_y: 0,
        b_x: 0,
        b_y: 0,
        p_x: 0,
        p_y: 0,
    };
    let mut machines: Vec<PrizeMatrix> = Vec::new();
    for (i, line) in content.lines().enumerate() {
        match i % 4 {
            0 => {
                // Button A Line
                let matcher = Regex::new(r#"Button A: X\+([0-9]+), Y\+([0-9]+)"#).unwrap();
                let captures = matcher.captures(line).unwrap();
                machine.a_x = captures.get(1).unwrap().as_str().parse().unwrap();
                machine.a_y = captures.get(2).unwrap().as_str().parse().unwrap();
            }
            1 => {
                // Button B Line
                let matcher = Regex::new(r#"Button B: X\+([0-9]+), Y\+([0-9]+)"#).unwrap();
                let captures = matcher.captures(line).unwrap();
                machine.b_x = captures.get(1).unwrap().as_str().parse().unwrap();
                machine.b_y = captures.get(2).unwrap().as_str().parse().unwrap();
            }
            2 => {
                // Prize Line
                let matcher = Regex::new(r#"Prize: X=([0-9]+), Y=([0-9]+)"#).unwrap();
                let captures = matcher.captures(line).unwrap();
                machine.p_x = captures.get(1).unwrap().as_str().parse().unwrap();
                machine.p_y = captures.get(2).unwrap().as_str().parse().unwrap();
            }
            3 => {
                // Blank Line
                machines.push(machine);
                machine = PrizeMatrix {
                    a_x: 0,
                    a_y: 0,
                    b_x: 0,
                    b_y: 0,
                    p_x: 0,
                    p_y: 0,
                };
            }
            _ => {}
        }
    }

    let mut total: u64 = 0;
    for (_idx, machine) in machines.iter().enumerate() {
        let det_denom = machine.a_x * machine.b_y - machine.a_y * machine.b_x;
        if det_denom == 0 {
            continue;
        }

        let mut i: f64 = 0.0;
        let mut j: f64 = 0.0;

        // Get to 10000000000000 in steps to avoid an overflow
        /*
        let i_step = (machine.b_y as f64 * 100000 as f64 - machine.b_x as f64 * 100000 as f64)
            / det_denom as f64;
        let j_step = (-machine.a_y as f64 * 100000 as f64 + machine.a_x as f64 * 100000 as f64)
            / det_denom as f64;

        let add: u64 = 10000000000000 / 100000;

        debug!("Adding {:?} steps of {:?} {:?}", add, i_step, j_step);

        i += add as f64 * i_step;
        j += add as f64 * j_step;
        */

        let p_x_a: f64 = (10000000000000.0 + machine.p_x as f64) / det_denom as f64;
        let p_y_a: f64 = (10000000000000.0 + machine.p_y as f64) / det_denom as f64;
        i += machine.b_y as f64 * p_x_a - machine.b_x as f64 * p_y_a;
        j += -machine.a_y as f64 * p_x_a + machine.a_x as f64 * p_y_a;

        if i < 0.0 || j < 0.0 {
            continue;
        }

        debug!("Prize at {:?} {:?}", i, j);

        let i_rem = (i.round() - i).abs();
        let j_rem = (j.round() - j).abs();
        if i_rem < 0.001 && j_rem < 0.001 {
            total += 3 * (i.round() as u64) + (j.round() as u64);
        }
    }

    info!("Total Presses Needed {:?}", total);
}

#[derive(Debug, Clone)]
enum DirectionType {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone)]
struct Edge {
    pos: (i32, i32),
    other: i32,
    direction: DirectionType,
}

#[derive(Clone)]
struct GardenEntry {
    label: char,
    area: i32,
    perimiter: i32,
    edges: Vec<Edge>,
}

#[instrument]
pub fn day12(filename: String) {
    let content = fs::read_to_string(filename).expect("Couldn't read input");

    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    let mut labels: HashMap<(i32, i32), i32> = HashMap::new();

    let mut rows: i32 = 0;
    let mut cols: i32 = 0;

    for (r, line) in content.lines().enumerate() {
        rows += 1;
        cols = line.len() as i32;
        for (c, char) in line.chars().enumerate() {
            map.insert((r as i32, c as i32), char);
        }
    }

    let mut sizes: HashMap<i32, GardenEntry> = HashMap::new();
    let mut label_index: i32 = 0;

    // Two split reigons are not the same reigon
    for r in 0..rows {
        for c in 0..cols {
            let char = map.get(&(r, c)).unwrap();
            let mut perimiter = 0;
            let mut label: i32 = *labels.get(&(r, c)).unwrap_or(&label_index);
            let mut adopted = false;

            let mut edges: Vec<Edge> = Vec::new();
            for (dr, dc) in LURD {
                let test = (r + dr, c + dc);
                let d_type: DirectionType;
                let other: i32;

                if dr == 0 {
                    d_type = DirectionType::Vertical;
                    other = dc;
                } else {
                    d_type = DirectionType::Horizontal;
                    other = dr;
                }

                if let Some(val) = map.get(&test) {
                    if val != char {
                        perimiter += 1;
                        edges.push(Edge {
                            pos: (r, c),
                            other,
                            direction: d_type,
                        });
                    } else {
                        if let Some(test_label_val) = labels.get(&test) {
                            let test_val: i32 = *test_label_val;
                            if label != test_val {
                                // Adopt the existing label, remap everything else.
                                debug!(
                                    "Merging {:?} and {:?} which are both {:?}",
                                    test_val, label, char
                                );
                                adopted = true;

                                let mut to_change: Vec<(i32, i32)> = Vec::new();
                                for (k, v) in labels.iter() {
                                    if *v == label {
                                        to_change.push(*k);
                                    }
                                }

                                debug!("Changing {:?} labels", to_change.len());
                                for k in to_change {
                                    labels.insert(k, test_val);
                                }

                                // Get the old sizes and merge into the new
                                let mut size_entry = sizes
                                    .get(&label)
                                    .unwrap_or(&GardenEntry {
                                        label: '#',
                                        area: 0,
                                        perimiter: 0,
                                        edges: Vec::new(),
                                    })
                                    .clone();
                                sizes
                                    .entry(test_val)
                                    .and_modify(|x| {
                                        x.area += size_entry.area;
                                        x.perimiter += size_entry.perimiter;
                                        x.edges.append(&mut size_entry.edges);
                                    })
                                    .or_insert(size_entry);
                                sizes.remove(&label);

                                label = test_val;
                            }
                        }
                    }
                } else {
                    perimiter += 1;
                    edges.push(Edge {
                        pos: (r, c),
                        other,
                        direction: d_type,
                    });
                }
            }

            sizes
                .entry(label)
                .and_modify(|x| {
                    x.area += 1;
                    x.perimiter += perimiter;
                    x.edges.append(&mut edges);
                })
                .or_insert(GardenEntry {
                    label: *char,
                    area: 1,
                    perimiter,
                    edges,
                });

            labels.insert((r, c), label);

            if !adopted {
                label_index += 1;
            }
        }
    }

    let mut total = 0;
    for (char, entry) in sizes.iter() {
        debug!(
            "Reigon {:?} area={:?}, perimiter={:?}",
            char, entry.area, entry.perimiter
        );
        total += entry.area * entry.perimiter;
    }

    info!("Total Fence Cost {:?}", total);

    let mut part_b_price = 0;

    for (_, entry) in sizes.iter() {
        let mut total_edges = 0;
        debug!("Total Edges To Check {:?}", entry.edges.len());
        debug!("{:?}", entry.edges);
        for other in [-1, 1] {
            // Process Vertial Edges
            for c in 0..cols {
                let mut filtered_edges: Vec<&Edge> = entry
                    .edges
                    .iter()
                    .filter(|x| {
                        (x.pos.1 == c)
                            && (other == x.other)
                            && matches!(x.direction, DirectionType::Vertical)
                    })
                    .collect();
                filtered_edges.sort_by(|x, y| x.pos.0.cmp(&y.pos.0));
                let mut previous_r = -2;
                for e in filtered_edges {
                    if e.pos.0 - previous_r > 1 {
                        total_edges += 1;
                    }
                    previous_r = e.pos.0;
                }
            }
            // Process Horizontal Edges
            for r in 0..rows {
                let mut filtered_edges: Vec<&Edge> = entry
                    .edges
                    .iter()
                    .filter(|x| {
                        (x.pos.0 == r)
                            && (other == x.other)
                            && matches!(x.direction, DirectionType::Horizontal)
                    })
                    .collect();
                filtered_edges.sort_by(|x, y| x.pos.1.cmp(&y.pos.1));
                let mut previous_c = -2;
                for e in filtered_edges {
                    if e.pos.1 - previous_c > 1 {
                        total_edges += 1;
                    }
                    previous_c = e.pos.1;
                }
            }
        }
        debug!(
            "Reigon {:?} area={:?}, total_edges={:?}",
            entry.label, entry.area, total_edges
        );
        part_b_price += total_edges * entry.area;
    }

    info!("Total Fence Cost B {:?}", part_b_price);
}

#[instrument(skip(cache))]
pub fn split_stones(stone: u64, n: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if n == 0 {
        return 1;
    } else {
        if stone == 0 {
            if let Some(x) = cache.get(&(1, n - 1)) {
                return *x;
            } else {
                let val = split_stones(1, n - 1, cache);
                cache.insert((1, n - 1), val);
                return val;
            }
        } else if stone.to_string().len() % 2 == 0 {
            let mut stone_string = stone.to_string();
            let a = stone_string.split_off(stone_string.len() / 2);

            let mut parts = 0;
            for part in [
                stone_string.parse::<u64>().unwrap(),
                a.parse::<u64>().unwrap(),
            ] {
                if let Some(x) = cache.get(&(part, n - 1)) {
                    parts += *x;
                } else {
                    let val = split_stones(part, n - 1, cache);
                    cache.insert((part, n - 1), val);
                    parts += val;
                }
            }

            return parts;
        } else {
            if let Some(x) = cache.get(&(stone * 2024, n - 1)) {
                return *x;
            } else {
                let val = split_stones(stone * 2024, n - 1, cache);
                cache.insert((stone * 2024, n - 1), val);
                return val;
            }
        }
    }
}

#[instrument]
pub fn day11(filename: String) {
    let content = fs::read_to_string(filename).expect("Couldn't read input");

    let mut stones: Vec<u64> = content
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    stones.sort();

    // Maps (current_value, n_steps) -> Number of Stones
    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();
    let mut total = 0;

    for i in 0..100 {
        for j in 1..75 {
            split_stones(i, j, &mut cache);
        }
    }

    for stone in &stones {
        total += split_stones(*stone, 75, &mut cache);
        debug!("Finished {:?} Cache Size {:?}", stone, cache.len());
    }
    info!("Final Stones Recursive {:?}", total);
}

const LURD: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

struct Step {
    pos: (i32, i32),
    val: i32,
}

#[instrument]
pub fn day10(filename: String) {
    let content = fs::read_to_string(filename).expect("Couldn't read input");

    let mut starts: Vec<(i32, i32)> = Vec::new();
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    for (r, line) in content.lines().enumerate() {
        for (c, char) in line.chars().enumerate() {
            let parsed: i32 = char.to_string().parse().unwrap();
            map.insert((r as i32, c as i32), parsed);
            if parsed == 0 {
                starts.push((r as i32, c as i32));
            }
        }
    }

    let mut score = 0;
    let mut routes = 0;
    debug!("Got {:?} Trailheds to Check", starts.len());
    for start in starts.iter() {
        let mut to_check: Vec<Step> = Vec::new();
        let mut end_points: HashSet<(i32, i32)> = HashSet::new();
        to_check.push(Step {
            pos: *start,
            val: 0,
        });

        while let Some(x) = to_check.pop() {
            if x.val == 9 {
                end_points.insert(x.pos);
                routes += 1;
                continue;
            }

            for (dr, dc) in LURD {
                let test = (x.pos.0 + dr, x.pos.1 + dc);
                if let Some(v) = map.get(&test) {
                    if *v == x.val + 1 {
                        to_check.push(Step { pos: test, val: *v });
                    }
                }
            }
        }

        score += end_points.len();
    }

    info!("Final Score {:?}", score);
    info!("Final Rating {:?}", routes);
}

#[derive(Debug, Clone)]
struct Chunk {
    start: usize,
    id: i64,
    length: usize,
}

pub fn defrag(chunks: Vec<Chunk>) -> Vec<Chunk> {
    let mut known: usize = chunks.len() - 1;
    let mut chunky = chunks.clone();

    // Not convinced this will always go through the chunks by file ID in order
    let mut moved: HashSet<i64> = HashSet::new();

    while known > 0 {
        let elem = chunky[known].clone();
        if !moved.contains(&elem.id) {
            debug!("Searching for a gap for {:?}", elem.id);
            moved.insert(chunky[known].id);

            let mut set = false;
            for i in 0..chunks.len() - 1 {
                let first = chunky[i].clone();
                let second = chunky[i + 1].clone();
                let gap = second.start - (first.start + first.length);
                debug!(
                    "Gap Between {:?} and {:?} is {:?} needs to be <= {:?}",
                    first.id, second.id, gap, elem.length
                );

                if elem.length <= gap && first.start + first.length < elem.start {
                    chunky[known].start = first.start + first.length;
                    debug!("Moving {:?} to {:?}", chunky[known].id, chunky[known].start);
                    set = true;
                    break;
                }
            }
            chunky.sort_by(|a, b| a.start.cmp(&b.start));
            if !set {
                known -= 1;
            }
        } else {
            known -= 1;
        }
    }

    chunky.sort_by(|a, b| a.start.cmp(&b.start));
    chunky
}

#[instrument]
pub fn day9(filename: String) {
    let content = fs::read_to_string(filename).expect("Couldn't read input");

    let mut chunks: Vec<Chunk> = Vec::new();
    let mut id = 0;
    let mut offset = 0;
    let mut is_file = true;

    for c in content.chars() {
        if c == '\n' {
            break;
        }

        let size: usize = c.to_string().parse().unwrap();
        if is_file {
            if size > 0 {
                chunks.push(Chunk {
                    start: offset,
                    id: id,
                    length: size,
                });
                id += 1;
                offset += size;
            }
            is_file = false;
        } else {
            offset += size;
            is_file = true;
        }
    }

    let mut checksum: u64 = 0;

    let mut cursor_left = 0;
    let mut offset_left: usize = 0;

    let mut cursor_right = chunks[chunks.len() - 1].start + chunks[chunks.len() - 1].length - 1;
    let mut offset_right: usize = 0;

    loop {
        let current = chunks[offset_left].clone();

        // Are we in current or between current and next
        if cursor_left < current.start + current.length {
            // Still in current
            let add = cursor_left as u64 * current.id as u64;
            checksum += add;
            debug!(current.id, cursor_left, add, checksum);
            cursor_left += 1;
        } else {
            if offset_left + 1 == chunks.len() {
                break;
            }
            let next = chunks[&offset_left + 1].clone();
            if cursor_left < next.start {
                // In a gap

                let back_current = chunks[chunks.len() - 1 - offset_right].clone();

                let add = cursor_left as u64 * back_current.id as u64;
                checksum += add;
                debug!(back_current.id, cursor_left, add, checksum);

                cursor_right -= 1;

                if cursor_right < back_current.start {
                    let back_next = chunks[chunks.len() - 2 - offset_right].clone();
                    cursor_right = back_next.start + back_next.length - 1;
                    offset_right += 1;
                }

                cursor_left += 1;
            }

            if cursor_left == next.start {
                offset_left += 1;
            }
        }

        // If left passed right, break
        if cursor_left > cursor_right {
            break;
        }
    }

    info!("Total Checksum {:?}", checksum);

    chunks = defrag(chunks);
    info!("Defragment Complete");

    let mut checksum_two: u64 = 0;
    for chunk in chunks {
        for p in chunk.start..chunk.start + chunk.length {
            checksum_two += p as u64 * chunk.id as u64;
        }
    }

    info!("Total Checksum B {:?}", checksum_two);
}

#[instrument]
pub fn day8(filename: String) {
    let content = fs::read_to_string(filename).expect("Couldn't read input");

    let mut nodes: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut rows: i32 = 0;
    let mut cols: i32 = 0;

    for (row, line) in content.lines().enumerate() {
        cols = line.len() as i32;
        rows += 1;
        for (col, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            nodes
                .entry(c)
                .and_modify(|x| x.push((row as i32, col as i32)))
                .or_insert(vec![(row as i32, col as i32)]);
        }
    }

    // For each node type, check each node against each other node
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    let mut antinodes_all: HashSet<(i32, i32)> = HashSet::new();
    for (_, node) in nodes.iter() {
        for (i, (a, b)) in node.iter().enumerate() {
            for (j, (c, d)) in node.iter().enumerate() {
                if i == j {
                    continue;
                }
                let x = 2 * c - a;
                let y = 2 * d - b;
                if is_in_bounds(rows, cols, x, y) {
                    //debug!("Found {:?}", (x, y));
                    antinodes.insert((x, y));
                }

                let dx = c - a;
                let dy = d - b;

                for copy in 1..rows {
                    let x = a + copy * dx;
                    let y = b + copy * dy;
                    if is_in_bounds(rows, cols, x, y) {
                        //debug!("Found {:?}", (x, y));
                        antinodes_all.insert((x, y));
                    }
                }
            }
        }
    }

    info!("Total Antinodes {:?}", antinodes.len());
    info!("Total Antinodes B {:?}", antinodes_all.len());
}

#[instrument]
pub fn try_math(target: i64, current: i64, parts: &[i64], can_concat: bool) -> bool {
    if parts.len() == 0 {
        return false;
    }

    if current > target {
        return false;
    }

    let next = parts[0];

    if parts.len() == 1 {
        let no_concat = current * next == target || current + next == target;
        if can_concat {
            let cc: i64 = (current.to_string() + &next.to_string()).parse().unwrap();
            return (cc == target) || no_concat;
        } else {
            return no_concat;
        }
    } else {
        let no_concat = try_math(target, current * next, &parts[1..], can_concat)
            || try_math(target, current + next, &parts[1..], can_concat);
        if can_concat {
            let cc: i64 = (current.to_string() + &next.to_string()).parse().unwrap();
            return try_math(target, cc, &parts[1..], can_concat) || no_concat;
        } else {
            return no_concat;
        }
    }
}

#[instrument]
pub fn day7(filename: String) {
    let content = fs::read_to_string(filename).expect("Couldn't read input");

    let mut inputs: Vec<(i64, Vec<i64>)> = Vec::new();

    for line in content.lines() {
        let (target, rest) = line.split_once(": ").unwrap();
        let parts: Vec<i64> = rest.split(' ').map(|x| x.parse().unwrap()).collect();

        inputs.push((target.parse().unwrap(), parts));
    }

    let mut total = 0;
    let mut total_b = 0;

    for (target, parts) in inputs {
        if try_math(target, parts[0], &parts[1..], false) {
            total += target;
        }
        if try_math(target, parts[0], &parts[1..], true) {
            total_b += target;
        }
    }

    info!("Final Total {:?}", total);
    info!("Final Total B {:?}", total_b);
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Bearing {
    pos: (i32, i32),
    d: Direction,
}

#[instrument]
pub fn day6(filename: String) {
    let content = fs::read_to_string(filename).expect("Couldn't read input");

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

    let mut additions: HashSet<(i32, i32)> = HashSet::new();

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

            if !is_in_bounds(rows, cols, position.pos.0, position.pos.1) {
                break;
            }

            let fake_blocker = move_direction(&position.pos, &position.d);

            if is_in_bounds(rows, cols, fake_blocker.0, fake_blocker.1)
                && !visited_pos.contains(&fake_blocker)
            {
                let mut test_bearing = Bearing {
                    pos: position.pos.clone(),
                    d: turn(&position.d, Rotation::Right),
                };
                // Need to store the extra locations
                let mut test_visited: HashSet<Bearing> = HashSet::new();
                loop {
                    // If OOB Break
                    if !is_in_bounds(rows, cols, test_bearing.pos.0, test_bearing.pos.1) {
                        break;
                    }

                    if test_visited.contains(&test_bearing) {
                        additions.insert(fake_blocker.clone());
                        break;
                    }

                    test_visited.insert(test_bearing.clone());

                    let test_pos = move_direction(&test_bearing.pos, &test_bearing.d);

                    if map.contains(&test_pos) || test_pos == fake_blocker {
                        test_bearing.d = turn(&test_bearing.d, Rotation::Right);
                    } else {
                        test_bearing.pos = test_pos;
                    }
                }
            }
        }

        visited_pos.insert(position.pos.clone());

        position = Bearing {
            pos: new_pos,
            d: new_direction,
        };
    }

    info!("Total Visited Locations {:?}", visited_pos.len());
    info!("Total Potential Additions {:?}", additions.len());
}

pub fn day5() {
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

pub fn day4() {
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

pub fn day3() {
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

pub fn check(items: Vec<i32>) -> CheckState {
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

pub fn day2() {
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

pub fn day1() {
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
