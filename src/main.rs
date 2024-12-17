use std::{
    collections::{HashMap, HashSet},
    fs, io,
};

use tracing::{Level, debug, error, event, info, instrument, warn};
use tracing_subscriber::EnvFilter;

mod done;
mod util;

use crate::done::*;
use crate::util::*;

#[instrument]
pub fn dayxx(filename: String, part_b: bool) {
    let content = fs::read_to_string(filename).expect("Couldn't read input");
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

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    info!("Tracing Setup");

    //day17("./inputs/day17bsmall.txt".to_string(), true);
    day17("./inputs/day17.txt".to_string(), true);
}
