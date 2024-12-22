use crate::util::*;
use itertools::Itertools;
use rayon::prelude::*;

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs, io,
    iter::zip,
    usize,
};

use anyhow::{Context, Result};
use regex::Regex;
use tracing::{Level, debug, error, event, info, instrument, warn};

fn directions_to_keys(directions: &Vec<Direction>) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    for dir in directions {
        result.push(match dir {
            Direction::North => '^',
            Direction::East => '>',
            Direction::West => '<',
            Direction::South => 'v',
        });
    }
    result.push('A');
    return result;
}

fn generate_short_path(
    keypad: &HashMap<char, (i32, i32)>,
    empty: (i32, i32),
    target: &String,
    tricky: bool,
) -> Result<Vec<String>> {
    let mut current: (i32, i32) = keypad.get(&'A').context("No A Key Found!")?.clone();

    let mut result: Vec<String> = Vec::new();
    result.push("".to_string());

    for c in target.chars() {
        let mut directions_v: Vec<Direction> = Vec::new();
        let mut directions_h: Vec<Direction> = Vec::new();
        let target = keypad.get(&c).context("Unexpected Character Requested")?;
        let dr: i32 = target.0 - current.0;
        let dc: i32 = target.1 - current.1;

        for _ in 0..dc.abs() {
            if dc < 0 {
                directions_h.push(Direction::West);
            } else if dc > 0 {
                directions_h.push(Direction::East);
            }
        }

        for _ in 0..dr.abs() {
            if dr < 0 {
                directions_v.push(Direction::North);
            } else if dr > 0 {
                directions_v.push(Direction::South);
            }
        }

        let mut new_results: Vec<String> = Vec::new();

        // Surely only LR then UD or UD then LR matter
        let mut vh = directions_v.clone();
        vh.extend(directions_h.clone());

        let mut hv = directions_h.clone();
        hv.extend(directions_v.clone());
        /*
        for permutation in directions
            .iter()
            .cloned()
            .permutations(directions.len())
            .unique()
        */
        let mut choices: Vec<Vec<Direction>> = Vec::new();
        if tricky {
            if dc == 0 || dr == 0 {
                choices.push(hv);
            } else if dr > 0 && dc > 0 {
                choices.push(vh);
            } else if dr > 0 && dc < 0 {
                if dc == -1 {
                    choices.push(hv);
                    choices.push(vh);
                } else {
                    choices.push(vh);
                }
            } else if dr < 0 && dc < 0 {
                if dc == -1 {
                    choices.push(hv);
                    choices.push(vh);
                } else {
                    choices.push(vh);
                }
            } else {
                // dr < 0 && dc > 0
                if dc == 1 {
                    choices.push(vh);
                    choices.push(hv);
                } else {
                    choices.push(hv);
                }
            }
        } else {
            choices.push(vh);
            choices.push(hv);
        }

        for permutation in choices.iter() {
            let mut test = current;
            let mut keep = true;
            for step in permutation.iter().cloned() {
                let new_pos = move_direction(&test, &step);
                if new_pos == empty {
                    keep = false;
                    // We can never actually get here apparently based on how we've set things up
                }
                test = new_pos;
            }
            if keep {
                for current_string in result.iter().cloned() {
                    let mut new_string = current_string.clone();
                    new_string.extend(directions_to_keys(&permutation));
                    new_results.push(new_string);
                }
                if tricky {
                    break;
                }
            }
        }
        result = new_results.iter().unique().cloned().collect();
        current = target.clone();
    }

    Ok(result)
}

#[instrument]
pub fn day21(filename: String, part_b: bool) -> Result<()> {
    let content = fs::read_to_string(filename).context("Couldn't read input")?;

    let codes: Vec<String> = content.lines().map(|x| x.to_string()).collect();

    let numeric_pad: HashMap<char, (i32, i32)> = HashMap::from([
        ('0', (3, 1)),
        ('A', (3, 2)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
    ]);
    let numeric_empty: (i32, i32) = (3, 0);

    let key_pad: HashMap<char, (i32, i32)> = HashMap::from([
        ('<', (1, 0)),
        ('v', (1, 1)),
        ('>', (1, 2)),
        ('^', (0, 1)),
        ('A', (0, 2)),
    ]);

    let keypad_empty: (i32, i32) = (0, 0);

    let mut total = 0;

    let steps: i32;

    if part_b {
        steps = 25;
    } else {
        steps = 2;
    }

    for code in codes {
        let step_1: Vec<String> = generate_short_path(&numeric_pad, numeric_empty, &code, false)
            .context("Step 1 Failed")?;

        let mut step_n: Vec<String> = step_1.clone();
        let mut completed = 0;

        for _ in 0..steps {
            debug!(len = step_n.len(), "Continuing");

            let next_step: Vec<String> = step_n
                .par_iter()
                .map(|input| generate_short_path(&key_pad, keypad_empty, &input, true).unwrap())
                .flatten()
                .collect();

            let min_len = next_step
                .iter()
                .map(|x| x.len())
                .min()
                .context("Nothing to len")?;

            // Only keep the shortest sequences
            step_n = next_step
                .iter()
                .filter(|x| x.len() == min_len)
                .cloned()
                .collect();
            debug!(pre_l = next_step.len(), post_l = step_n.len(), "Filtered");

            completed += 1;

            if step_n.len() == 1 {
                debug!(completed, "Leaving Part 1");
                break;
            }
        }

        if step_n.len() != 1 {
            error!("More than 1 Option Left");
        }
        let next_step: String = step_n[0].clone();
        let next_parts: Vec<String> = next_step
            .split_inclusive('A')
            .map(|x| x.to_string())
            .collect();
        let mut next_hist: HashMap<String, u64> = HashMap::new();

        for part in next_parts {
            next_hist.entry(part).and_modify(|x| *x += 1).or_insert(1);
        }

        for _ in completed..steps {
            let mut new_hist: HashMap<String, u64> = HashMap::new();
            for (k, v) in next_hist.clone() {
                let chunk = generate_short_path(&key_pad, keypad_empty, &k, true).unwrap();
                let chunk_parts: Vec<String> = chunk[0]
                    .split_inclusive('A')
                    .map(|x| x.to_string())
                    .collect();
                for part in chunk_parts {
                    new_hist.entry(part).and_modify(|x| *x += v).or_insert(v);
                }
            }
            next_hist = new_hist;
        }

        let mut len = 0;
        for (k, v) in next_hist.clone() {
            len += k.len() as u64 * v;
        }
        let digits: u64 = code[..3].parse().context("Couldn't parse code digits")?;
        let complexity: u64 = digits * len as u64;
        debug!(complexity, "Code Done");

        total += complexity;
    }

    info!(total, "Done");

    Ok(())
}

#[instrument]
pub fn day20(filename: String, part_b: bool) -> Result<()> {
    let content = fs::read_to_string(filename).context("Couldn't read input")?;

    let mut walls: HashSet<(i32, i32)> = HashSet::new();
    let mut not_walls: HashSet<(i32, i32)> = HashSet::new();
    let mut start: (i32, i32) = (0, 0);
    let mut end: (i32, i32) = (0, 0);

    let mut rows: i32 = 0;
    let mut cols: i32 = 0;

    let mut wall_list: Vec<(i32, i32)> = Vec::new();

    for (row, line) in content.lines().enumerate() {
        rows += 1;
        cols = line.len() as i32;
        for (col, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    walls.insert((row as i32, col as i32));
                    wall_list.push((row as i32, col as i32));
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

    let original_scores = shortest_distance(start, end, &walls, rows, cols, false, None);
    let original_scores_reverse = shortest_distance(end, start, &walls, rows, cols, false, None);
    let original_time = original_scores.get(&end).context("No Path Found")?;

    let mut cheat_routes: HashSet<((i32, i32), (i32, i32))> = HashSet::new();

    let max_d;

    if part_b {
        max_d = 20;
    } else {
        max_d = 2;
    }

    // Trace the shortest route
    // For each step, search a manhattan circle of max_d for any spots that save time
    // Sum them
    for (point, point_score) in original_scores.iter() {
        for dr in -max_d..=max_d {
            for dc in -max_d..=max_d {
                if ((dr as i32).abs() + (dc as i32).abs() > max_d)
                    || !is_in_bounds(rows, cols, point.0 + dr, point.1 + dc)
                    || walls.contains(&(point.0 + dr, point.1 + dc))
                {
                    continue;
                }

                let steps = dr.abs() + dc.abs();
                let test_point = (point.0 + dr, point.1 + dc);
                let test_score = original_scores_reverse
                    .get(&test_point)
                    .context("No Test Point Score")?;
                let new_distance = point_score + steps + test_score;
                if original_time - new_distance >= 100 {
                    debug!(?point, ?test_point, point_score, steps, "Got One");
                    cheat_routes.insert((*point, test_point));
                }
            }
        }
    }

    info!(cheats = cheat_routes.len(), "Done");
    Ok(())
}

fn can_be_made(target: String, parts: &Vec<String>, cache: &mut HashMap<String, u64>) -> u64 {
    if cache.contains_key(&target) {
        debug!(cache_size = cache.len(), target = target, "Way Cache Hit");
        return *cache.get(&target).unwrap();
    }

    // Check for the longest possible substring
    //for split in 1..target.len() {
    let mut to_try: Vec<String> = cache
        .iter()
        .filter(|&(_, v)| *v > 0)
        .map(|(k, _)| k.clone())
        .collect();
    to_try.extend(parts.clone());
    to_try.sort_by_key(|x| -(x.len() as i32));

    let mut ways: u64 = 0;
    for part in parts.clone() {
        if target.len() >= part.len() {
            let (l, r) = target.split_at(part.len());
            if l == part {
                if r.len() == 0 {
                    debug!(l = l, r = r, cache_size = cache.len(), "Way Found");
                    ways += 1;
                } else {
                    let result = can_be_made(r.to_string(), parts, cache);
                    if result > 0 {
                        debug!(
                            l = l,
                            r = r,
                            cache_size = cache.len(),
                            result = result,
                            "Way Found"
                        );
                        ways += result;
                    }
                }
            }
        }
    }

    cache.insert(target.clone(), ways);
    return ways;
}

#[instrument]
pub fn day19(filename: String, part_b: bool) -> Result<()> {
    let content = fs::read_to_string(filename).context("Couldn't read input")?;
    let mut section_one = true;

    let mut parts: Vec<String> = Vec::new();
    let mut targets: Vec<String> = Vec::new();
    let mut cache: HashMap<String, u64> = HashMap::new();

    for line in content.lines() {
        if line.len() == 0 {
            section_one = false;
            continue;
        }

        if section_one {
            parts = line.split(", ").map(|x| x.to_string()).collect();
        } else {
            targets.push(line.to_string());
        }
    }

    parts.sort_by_key(|x| -(x.len() as i32));

    let mut total = 0;
    let mut total_b = 0;
    for target in targets {
        info!(target = target, "Trying Next");
        let result = can_be_made(target.clone(), &parts, &mut cache);
        if result > 0 {
            info!(target = ?target.clone(), cache_size=cache.len(), result=result, "Found Match");
            total += 1;
            total_b += result;
        } else {
            info!(target = ?target.clone(), cache_size=cache.len(), result=result, "No Match");
        }
    }

    info!(total = total, total_b = total_b, "Done");

    Ok(())
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

#[instrument]
pub fn day17(filename: String, part_b: bool) {
    let content = fs::read_to_string(filename).expect("Couldn't read input");

    let mut register_a: u64 = 0;
    let mut register_b: u64 = 0;
    let mut register_c: u64 = 0;
    let mut instruction_pointer: usize = 0;
    let mut instructions: Vec<u8> = Vec::new();

    let mut target: String = "".to_string();

    for (i, line) in content.lines().enumerate() {
        if line.len() == 0 {
            continue;
        }

        let (_, right) = line.split_once(": ").unwrap();

        match i {
            0 => {
                register_a = right.parse().unwrap();
            }
            1 => {
                register_b = right.parse().unwrap();
            }
            2 => {
                register_c = right.parse().unwrap();
            }
            3 => {}
            4 => {
                instructions = right.split(',').map(|x| x.parse().unwrap()).collect();
            }
            _ => {}
        }
    }
    debug!(a=register_a, b=register_b, c=register_c, instructions=?instructions, "Read Program");

    let mut out_pointer;
    let mut a_test = 0;

    if part_b {
        a_test = 0; //8u64.pow(instructions.len() as u32);
    }

    loop {
        register_a = (a_test << 24) + 0b11101001011011010010011100000;
        register_b = 0;
        register_c = 0;
        instruction_pointer = 0;

        out_pointer = 0;
        loop {
            if instruction_pointer >= instructions.len() {
                debug!("End of Sequence");
                break;
            }
            let op_code = instructions[instruction_pointer];
            let operand = instructions[instruction_pointer + 1];

            let operand_value = match operand {
                0 | 1 | 2 | 3 => operand as u64,
                4 => register_a,
                5 => register_b,
                6 => register_c,
                7 => 0,
                _ => 0,
            };

            match op_code {
                0 => {
                    // adv
                    let div = 2u64.pow(operand_value as u32) as u64;
                    let result = register_a / div;
                    register_a = result;
                }
                1 => {
                    // bxl
                    let result = register_b ^ operand as u64;
                    register_b = result;
                }
                2 => {
                    // bst
                    let result = operand_value % 8;
                    register_b = result;
                }
                3 => {
                    // jnz
                    if register_a != 0 {
                        instruction_pointer = operand as usize;
                        continue;
                    }
                }
                4 => {
                    // bxc
                    register_b = register_b ^ register_c;
                }
                5 => {
                    // out
                    let result = operand_value % 8;
                    if result as u8 != instructions[out_pointer as usize] {
                        if out_pointer > 8 {
                            debug!(
                                a = a_test,
                                out = out_pointer,
                                "Bailed on output mismatch {:b}",
                                a_test
                            );
                        }
                        break;
                    }

                    out_pointer += 1;

                    if out_pointer >= instructions.len() {
                        debug!("Bailed on output Length");
                    }
                }
                6 => {
                    // bdv
                    let div = 2u64.pow(operand_value as u32) as u64;
                    let result = register_a / div;
                    register_b = result;
                }
                7 => {
                    // cdv
                    let div = 2u64.pow(operand_value as u32) as u64;
                    let result = register_a / div;
                    register_c = result;
                }
                _ => {
                    error!(op_code = op_code, "Invalid Op Code");
                }
            };
            instruction_pointer += 2;
        }
        if out_pointer == instructions.len() {
            register_a = (a_test << 24) + 0b11101001011011010010011100000;
            info!(a_test = register_a, "Finished Run");
            break;
        }

        a_test += 1;
    }
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

#[derive(Debug, Clone, Copy)]
enum Tile {
    Wall,
    Box,
    Empty,
    BoxLeft,
    BoxRight,
}

fn display_map_15(map: &HashMap<(i32, i32), Tile>, rows: i32, cols: i32, player: (i32, i32)) {
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

    let mut directions: Vec<Direction> = Vec::new();

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
                        directions.push(Direction::North);
                    }
                    'v' => {
                        directions.push(Direction::South);
                    }
                    '<' => {
                        directions.push(Direction::West);
                    }
                    '>' => {
                        directions.push(Direction::East);
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
        let test_position = move_direction(&player, &dir);
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
                    let search_position = move_direction(&test_position, &dir);
                    search_positions.push(search_position);

                    if part_b {
                        // If part b and moving up or down, add the other half of the box
                        if matches!(dir, Direction::North | Direction::South) {
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
                                        if matches!(dir, Direction::North | Direction::South) {
                                            // Add the area above the box to the list to check
                                            // Don't keep checking this one
                                            let new_search_position =
                                                move_direction(&search_position, &dir);
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
                                        if matches!(dir, Direction::North | Direction::South) {
                                            // Add the area above the box to the list to check
                                            // Don't keep checking this one
                                            /*let new_search_position = move_direction(
                                                &move_direction(&search_position, &dir),
                                                &dir,
                                            );*/
                                            let new_search_position =
                                                move_direction(&search_position, &dir);
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
                                        if matches!(dir, Direction::North | Direction::South) {
                                            continue;
                                        } else {
                                            break; // TODO: Unsure
                                        }
                                    }
                                };
                                let new_search_position = move_direction(&search_position, &dir);
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
                            if matches!(dir, Direction::East | Direction::West) {
                                if to_fill.len() != 1 {
                                    error!(len=?to_fill.len(),"More than one to_fill found");
                                }
                                let search_position = to_fill[0];
                                let first_spot = move_direction(&test_position, &dir);
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
                                    let clear_spot =
                                        move_direction(&spot, &opposite_direction(&dir));
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

    display_map_15(&map, rows, cols * 2, player);

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

pub const LURD: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

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
