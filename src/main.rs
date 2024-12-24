use std::{
    collections::{HashMap, HashSet},
    fs,
};

use anyhow::{Context, Result};
use itertools::{Itertools, Permutations};
use tracing::{Level, debug, error, event, info, instrument, warn};
use tracing_subscriber::{EnvFilter, field::debug};

mod done;
mod util;

#[instrument]
pub fn dayxx(filename: String, part_b: bool) -> Result<()> {
    let content = fs::read_to_string(filename).context("Couldn't read input")?;

    Ok(())
}

#[derive(Clone)]
enum Gate {
    And,
    Or,
    Xor,
}

#[derive(Clone)]
struct GateConfig {
    left: String,
    right: String,
    op: Gate,
}

#[derive(Clone)]
enum GateNode {
    Const(bool),
    Gate(GateConfig),
}

fn eval(
    name: String,
    nodes: &HashMap<String, GateNode>,
    visited: &mut HashSet<String>,
) -> Option<bool> {
    /*
        if visited.contains(&name) {
            return None;
        }
    */

    visited.insert(name.clone());

    let node = nodes.get(&name).unwrap();
    match node {
        GateNode::Const(x) => {
            return Some(*x);
        }
        GateNode::Gate(gate_config) => {
            let mut a_clone = visited.clone();
            let a = eval(gate_config.left.clone(), nodes, &mut a_clone);

            let mut b_clone = visited.clone();
            let b = eval(gate_config.right.clone(), nodes, &mut b_clone);

            if let None = a {
                return None;
            }

            if let None = b {
                return None;
            }

            match gate_config.op {
                Gate::And => {
                    return Some(a.unwrap() & b.unwrap());
                }
                Gate::Or => {
                    return Some(a.unwrap() | b.unwrap());
                }
                Gate::Xor => {
                    return Some(a.unwrap() ^ b.unwrap());
                }
            };
        }
    };
}

fn mark_clean(name: String, nodes: &HashMap<String, GateNode>, clean_nodes: &mut HashSet<String>) {
    let node = nodes.get(&name).unwrap();
    match node {
        GateNode::Const(_) => {
            return;
        }
        GateNode::Gate(gate_config) => {
            clean_nodes.insert(gate_config.left.clone());
            clean_nodes.insert(gate_config.right.clone());

            mark_clean(gate_config.left.clone(), nodes, clean_nodes);
            mark_clean(gate_config.right.clone(), nodes, clean_nodes);
        }
    };
}

#[instrument]
pub fn day24(filename: String, part_b: bool) -> Result<()> {
    let content = fs::read_to_string(filename).context("Couldn't read input")?;

    let mut gates: HashMap<String, GateNode> = HashMap::new();
    let mut z_gates: Vec<String> = Vec::new();
    let mut x_gates: Vec<String> = Vec::new();
    let mut y_gates: Vec<String> = Vec::new();

    let mut constants = true;

    for line in content.lines() {
        if line.len() == 0 {
            constants = false;
            continue;
        }

        if constants {
            let (l, r) = line.split_once(": ").unwrap();
            gates.insert(l.to_string(), GateNode::Const(r == "1"));
            if l.chars().nth(0).unwrap() == 'y' {
                x_gates.push(l.to_string());
            }
            if l.chars().nth(0).unwrap() == 'x' {
                y_gates.push(l.to_string());
            }
        } else {
            let (l, name) = line.split_once(" -> ").unwrap();
            let parts: Vec<&str> = l.split(" ").collect();
            let a = parts[0];
            let b = parts[2];
            if name.chars().nth(0).unwrap() == 'z' {
                z_gates.push(name.to_string());
            }
            match parts[1] {
                "AND" => {
                    gates.insert(
                        name.to_string(),
                        GateNode::Gate(GateConfig {
                            left: a.to_string(),
                            right: b.to_string(),
                            op: Gate::And,
                        }),
                    );
                }
                "OR" => {
                    gates.insert(
                        name.to_string(),
                        GateNode::Gate(GateConfig {
                            left: a.to_string(),
                            right: b.to_string(),
                            op: Gate::Or,
                        }),
                    );
                }
                "XOR" => {
                    gates.insert(
                        name.to_string(),
                        GateNode::Gate(GateConfig {
                            left: a.to_string(),
                            right: b.to_string(),
                            op: Gate::Xor,
                        }),
                    );
                }
                _ => {
                    error!("Unexpected Operation");
                }
            };
        }
    }

    z_gates.sort_by_key(|x| {
        let v: i32 = x[1..].parse().unwrap();
        return -v;
    });

    let mut result: u64 = 0;
    for gate in z_gates.iter().cloned() {
        debug!(gate, "Next Z Gate");
        result = result << 1;
        let mut visited: HashSet<String> = HashSet::new();
        if let Some(x) = eval(gate, &gates, &mut visited) {
            if x {
                result += 1;
            }
        }
    }

    info!(result, "Done");

    // Part b
    x_gates.sort_by_key(|x| {
        let v: i32 = x[1..].parse().unwrap();
        return -v;
    });

    y_gates.sort_by_key(|x| {
        let v: i32 = x[1..].parse().unwrap();
        return -v;
    });

    let mut x_val: u64 = 0;
    let mut y_val: u64 = 0;

    for xgate in x_gates.iter().cloned() {
        x_val <<= 1;
        let mut visited: HashSet<String> = HashSet::new();
        if let Some(x) = eval(xgate, &gates, &mut visited) {
            if x {
                x_val += 1;
            }
        }
    }

    for ygate in y_gates.iter().cloned() {
        y_val <<= 1;
        let mut visited: HashSet<String> = HashSet::new();
        if let Some(x) = eval(ygate, &gates, &mut visited) {
            if x {
                y_val += 1;
            }
        }
    }

    debug!(x_val, y_val, target = x_val + y_val, "Target Values");

    // What if we mark all the nodes in the tree of okay binary values as 'clean' to limit the
    // search space...

    let mut clean_nodes: HashSet<String> = HashSet::new();
    clean_nodes.extend(x_gates.clone());
    clean_nodes.extend(y_gates.clone());

    let target = x_val + y_val;
    let mut shift = z_gates.len();
    let mut clean_z = 0;

    debug!("{:b}", target);
    for node in z_gates.iter().cloned() {
        let val = (target >> shift - 1) & 1;
        debug!(val, shift, node, "Checking");

        let mut visited: HashSet<String> = HashSet::new();
        if let Some(x) = eval(node.clone(), &gates, &mut visited) {
            if x {
                if val == 1 {
                    clean_z += 1;
                    mark_clean(node, &gates, &mut clean_nodes);
                }
            } else {
                if val == 0 {
                    clean_z += 1;
                    mark_clean(node, &gates, &mut clean_nodes);
                }
            }
        }

        shift -= 1;
    }

    let mut dirty_nodes: Vec<String> = Vec::new();
    for (gate, _) in gates.iter() {
        if !clean_nodes.contains(gate) {
            dirty_nodes.push(gate.to_string());
        }
    }

    debug!(
        len = clean_nodes.len(),
        dirty = dirty_nodes.len(),
        total = gates.len(),
        clean_z,
        "Added Cleaned Nodes"
    );

    // Loop through all permutations of the nodes. I have a strong feeling that this could get
    // fucked by creating loops though
    // Seems like loops are okay, probably needs to be looping up that's bad
    // Probably some way to progressively lock more nodes but not toally sure here
    // Worth a try in the morning
    let mut gates_current = gates.clone();

    while clean_z != z_gates.len() {
        for pick in dirty_nodes.iter().permutations(8).unique() {
            let mut node_clone = gates_current.clone();
            for i in 0..4 {
                let a = pick[2 * i];
                let b = pick[2 * i + 1];

                let a_v = gates.get(a).unwrap();
                let b_v = gates.get(b).unwrap();

                node_clone.insert(a.to_string(), b_v.clone());
                node_clone.insert(b.to_string(), a_v.clone());
            }

            let mut result: u64 = 0;
            let mut okay = true;
            for gate in z_gates.iter().cloned() {
                result = result << 1;
                let mut visited: HashSet<String> = HashSet::new();
                if let Some(x) = eval(gate, &node_clone, &mut visited) {
                    if x {
                        result += 1;
                    }
                } else {
                    // Found a loop, kill it
                    okay = false;
                    break;
                }
            }

            if okay {
                let mut new_clean_nodes: HashSet<String> = HashSet::new();
                clean_nodes.extend(x_gates.clone());
                clean_nodes.extend(y_gates.clone());

                let mut shift = z_gates.len();
                let mut new_clean_z = 0;

                for node in z_gates.iter().cloned() {
                    let val = (target >> shift - 1) & 1;

                    let mut visited: HashSet<String> = HashSet::new();
                    if let Some(x) = eval(node.clone(), &node_clone, &mut visited) {
                        if x {
                            if val == 1 {
                                new_clean_z += 1;
                                mark_clean(node, &node_clone, &mut new_clean_nodes);
                            }
                        } else {
                            if val == 0 {
                                new_clean_z += 1;
                                mark_clean(node, &node_clone, &mut new_clean_nodes);
                            }
                        }
                    }

                    shift -= 1;
                }

                if new_clean_z > clean_z {
                    debug!(new_clean_z, clean_z, "Closer");
                    let mut dirty_new: Vec<String> = Vec::new();
                    for (gate, _) in gates.iter() {
                        if !clean_nodes.contains(gate) {
                            dirty_new.push(gate.to_string());
                        }
                    }
                    dirty_nodes = dirty_new;
                    clean_z = new_clean_z;
                    gates_current = node_clone;
                    break;
                }
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    info!("Tracing Setup");

    day24("./inputs/day24small.txt".to_string(), false).context("Small Example")?;
    /*
        day24("./inputs/day24.txt".to_string(), false).context("Big Example")?;

        day24("./inputs/day24small.txt".to_string(), true).context("Small Example")?;
        day24("./inputs/day24.txt".to_string(), true).context("Big Example")?;
    */

    Ok(())
}
