#![allow(unused)]

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "AirPods battery checker")]
#[command(about = "Short about", long_about = "Long about")]
struct Cli {
    #[clap(short, long, default_value = "1")]
    interval: i8,
}

fn main() {
    let args = Cli::parse();
    println!("{}", args.interval);
}
