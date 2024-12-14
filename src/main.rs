use std::fs;

use tracing::{debug, info, instrument};
use tracing_subscriber::EnvFilter;

mod done;
mod util;

use crate::done::*;

#[instrument]
pub fn dayxx(filename: String) {
    let content = fs::read_to_string(filename).expect("Couldn't read input");
}

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
    for robot in robots {
        let f_r = (robot.pr + 100 * robot.vr).rem_euclid(rows);
        let f_c = (robot.pc + 100 * robot.vc).rem_euclid(cols);

        debug!("fr={:?} fc={:?}", f_r, f_c);

        let mut i = 0;

        if f_r == rows / 2 || f_c == cols / 2 {
            continue;
        }

        if f_c > cols / 2 {
            i = 1;
        }

        if f_r > rows / 2 {
            i += 2;
        }

        factors[i] += 1;
    }

    let mut total = 1;
    debug!("Factors: {:?}", factors);
    for v in factors {
        total *= v;
    }

    info!("Final Factors {:?}", total);
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    info!("Tracing Setup");

    println!("day 14");
    day14("./inputs/day14small.txt".to_string(), 7, 11);
    day14("./inputs/day14.txt".to_string(), 103, 101);
}
