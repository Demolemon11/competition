use std::process;

use clap::Parser;
use competition::arguments::Args;
use competition::pattern::{Inspect, Pattern};

fn main() {
    let k = Args::parse().k;
    if k < 4 || k > 7 {
        println!("学校数太多或太少!");
        process::exit(0)
    }
    Pattern::new(k).inspector()
}
