#![allow(unused)]

use clap::{Parser, Subcommand};
use std::thread;
use std::time::Duration;
use std::process::Command;

#[derive(Parser)]
#[command(name = "AirPods battery checker")]
#[command(about = "Short about", long_about = "Long about")]
struct Cli {
    #[clap(short, long, default_value = "1")]
    interval: u8,
}

fn main() {
    let args = Cli::parse();

    loop {
        clear_console();
        println!("Battery status:");

        thread::sleep(Duration::from_secs(args.interval as u64));
    }
    println!("{}", args.interval);
}

fn clear_console() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").arg("/c").arg("cls").status();
    } else {
        let _ = Command::new("clear").status();
    }
}
