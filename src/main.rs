use args::{Args, Commands};
use clap::Parser;
use commands::{
    delete::delete_command, edit::edit_command, insert::insert_command, list::list_commands, mov::move_command, prev::save_prev_command, utils::save_command
};

pub mod args;
pub mod config;
pub mod constants;
pub mod shell;

pub mod commands {
    pub mod delete;
    pub mod edit;
    pub mod insert;
    pub mod mov;
    pub mod prev;
    pub mod list;
    pub mod utils;
}

fn main() {
    let cfg = config::get_config();
    let args = Args::parse();

    match args.command {
        Some(Commands::Prev { path }) => save_prev_command(path, &cfg.context),
        Some(Commands::Add { command, path }) => save_command(&command, &path, &cfg.context),
        Some(Commands::Edit { path }) => edit_command(&cfg.editor, &cfg.context, path.as_deref()),
        Some(Commands::Move { path, new_path }) => {
            move_command(&cfg.context, path.as_deref(), new_path.as_deref())
        }
        Some(Commands::Delete { path, dir }) => delete_command(&cfg.context, path.as_deref(), dir),
        Some(Commands::List {}) => list_commands(&cfg.context),
        None => insert_command(&cfg.context, args.insert_command.as_deref()),
    }
}
