use tracing::info;
use tracing_subscriber::EnvFilter;

mod done;
mod util;

use crate::done::*;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    info!("Tracing Setup");

    println!("day 13");
    day13("./inputs/day13small.txt".to_string());
    day13("./inputs/day13.txt".to_string());
}
