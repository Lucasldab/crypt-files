mod cli;
mod commands;

use clap::Parser;
use cli::{Args, Commands};

fn main() {
    let args = Args::parse();

    match args.command {
        Some(Commands::Init(init)) => {
            commands::init::run(init);
        }
        Some(Commands::Get(get)) => {
            commands::get::run(get);
        }
        Some(Commands::Exit) => {
            commands::exit::run();
        }
        None => {
            eprintln!("No command provided. Use --help.");
        }
    }
}
