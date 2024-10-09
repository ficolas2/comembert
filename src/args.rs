use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Add a command")]
    Add { command: String, path: String},
    #[command(about = "Save the last command")]
    Prev { path: Option<String> },
    #[command(about = "Edit a command")]
    Edit { path: String},
    #[command(about = "Move a command")]
    Move {},
}
