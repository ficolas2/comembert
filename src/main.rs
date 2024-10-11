use args::{Args, Commands};
use clap::Parser;
use commands::{edit::edit_command, insert::insert_command, prev::save_prev_command, utils::save_command};

pub mod args;
pub mod config;
pub mod constants;
pub mod shell;

pub mod commands {
    pub mod edit;
    pub mod insert;
    pub mod mov;
    pub mod prev;
    pub mod utils;
}

fn main() {
    let cfg = config::get_config();
    let args = Args::parse();

    match args.command {
        Some(Commands::Prev { path }) => save_prev_command(path, &cfg.context),
        Some(Commands::Add { command, path }) => save_command(&command, &path, &cfg.context),
        Some(Commands::Edit { path }) => edit_command(&cfg.editor, &cfg.context, path.as_deref()),
        Some(Commands::Move {}) => println!("Not implemented"),
        None => insert_command(&cfg.context),
    }
}
