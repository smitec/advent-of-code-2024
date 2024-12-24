use std::{
    collections::{HashMap, HashSet},
    fs,
};

use anyhow::{Context, Result};
use genetic_algorithm::{
    crossover::{CrossoverMultiPoint, CrossoverSinglePoint},
    fitness::{Fitness, FitnessChromosome, FitnessGenotype, FitnessValue},
    genotype::{Genotype, ListGenotype},
    mutate::{MutateMultiGene, MutateSingleGene},
    select::SelectTournament,
    strategy::{
        Strategy,
        evolve::{Evolve, EvolveReporterSimple},
    },
};
use itertools::{Combinations, Itertools, Permutations};
use tracing::{Level, debug, error, event, info, instrument, warn};
use tracing_subscriber::{EnvFilter, field::debug};

mod done;
mod util;

#[instrument]
pub fn dayxx(filename: String, part_b: bool) -> Result<()> {
    let content = fs::read_to_string(filename).context("Couldn't read input")?;

    Ok(())
}

#[derive(Clone, Debug)]
enum Gate {
    And,
    Or,
    Xor,
}

#[derive(Clone, Debug)]
struct GateConfig {
    left: String,
    right: String,
    op: Gate,
}

#[derive(Clone, Debug)]
enum GateNode {
    Const(bool),
    Gate(GateConfig),
}

fn eval(
    name: String,
    nodes: &HashMap<String, GateNode>,
    visited: &mut HashSet<String>,
) -> Option<bool> {
    if visited.contains(&name) {
        return None;
    }

    visited.insert(name.clone());

    let node = nodes.get(&name).unwrap();
    match node {
        GateNode::Const(x) => {
            return Some(*x);
        }
        GateNode::Gate(gate_config) => {
            let mut a_clone = visited.clone();
            a_clone.insert(gate_config.right.clone());
            let a = eval(gate_config.left.clone(), nodes, &mut a_clone);

            let mut a_clone = visited.clone();
            a_clone.insert(gate_config.left.clone());
            let b = eval(gate_config.right.clone(), nodes, &mut a_clone);

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

fn eval_z(z_gates: &Vec<String>, gates: &HashMap<String, GateNode>) -> Option<i64> {
    let mut z_clone = z_gates.clone();
    z_clone.sort_by_key(|x| {
        let v: i32 = x[1..].parse().unwrap();
        return -v;
    });

    let mut result: i64 = 0;
    for gate in z_clone.iter().cloned() {
        result = result << 1;
        let mut visited: HashSet<String> = HashSet::new();
        if let Some(x) = eval(gate, &gates, &mut visited) {
            if x {
                result += 1;
            }
        } else {
            return None;
        }
    }

    return Some(result);
}

#[derive(Debug, Clone)]
struct SwapFitness<'a> {
    gates: &'a HashMap<String, GateNode>,
    all_gates: &'a Vec<(String, String)>,
    z_gates: &'a Vec<String>,
    target: i64,
}
impl Fitness for SwapFitness<'_> {
    type Genotype = ListGenotype;

    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        // Check for overlapping swaps and bail if they exist
        let indicies: Vec<usize> = chromosome.genes.clone();
        let swaps: Vec<(String, String)> = indicies
            .iter()
            .map(|x| self.all_gates.get(*x).unwrap())
            .cloned()
            .collect();

        let mut gate_clone = self.gates.clone();
        let mut unique: HashSet<String> = HashSet::new();
        for i in 0..4 {
            let a = swaps[i].0.clone();
            let b = swaps[i].1.clone();

            let a_v = self.gates.get(&a).unwrap();
            let b_v = self.gates.get(&b).unwrap();

            gate_clone.insert(a.clone(), b_v.clone());
            gate_clone.insert(b.clone(), a_v.clone());

            if unique.contains(&a) || unique.contains(&b) {
                return None;
            } else {
                unique.insert(a);
                unique.insert(b);
            }
        }

        let result = eval_z(self.z_gates, &gate_clone);

        if let Some(x) = result {
            let diff: i64 = self.target ^ x;
            return Some((-1 * diff.count_ones() as i32).try_into().unwrap());
        } else {
            return None;
        }
    }
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

    let result = eval_z(&z_gates, &gates);

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

    let all_gates: Vec<String> = gates
        .keys()
        .filter(|x| (x.chars().nth(0).unwrap() != 'x') && (x.chars().nth(0).unwrap() != 'y'))
        .cloned()
        .collect();

    let mut all_pairs: Vec<(String, String)> = Vec::new();
    for pair in all_gates.iter().combinations(2).unique() {
        all_pairs.push((pair[0].to_string(), pair[1].to_string()));
    }

    let genotype = ListGenotype::builder()
        .with_genes_size(4)
        .with_allele_list((0..all_pairs.len()).collect())
        .build()
        .unwrap();

    let fitness = SwapFitness {
        gates: &gates,
        all_gates: &all_pairs,
        z_gates: &z_gates,
        target: (x_val + y_val) as i64,
    };

    let mut evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(1000)
        .with_fitness(fitness)
        .with_target_fitness_score(0)
        .with_mutate(MutateSingleGene::new(0.9))
        .with_crossover(CrossoverSinglePoint::new())
        .with_select(SelectTournament::new(4, 0.9))
        .with_reporter(EvolveReporterSimple::new(100))
        .with_par_fitness(true)
        .build()
        .unwrap();

    evolve.call();

    if let Some(best_genes) = evolve.best_genes() {
        let selected_items = best_genes.clone();
        let swapped = selected_items.iter().map(|x| all_gates.get(*x).unwrap());
        info!(final=swapped.sorted().join(","), "done");
    } else {
        info!("All Garbage");
    }

    Ok(())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    info!("Tracing Setup");

    // day24("./inputs/day24small.txt".to_string(), false).context("Small Example")?;
    day24("./inputs/day24.txt".to_string(), false).context("Big Example")?;

    /*
        day24("./inputs/day24small.txt".to_string(), true).context("Small Example")?;
        day24("./inputs/day24.txt".to_string(), true).context("Big Example")?;
    */

    Ok(())
}
