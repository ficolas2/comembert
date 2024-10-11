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
    Add { command: String, path: String },
    #[command(about = "Save the last command")]
    Prev { path: Option<String> },
    #[command(about = "Edit a command")]
    Edit { path: Option<String> },
    #[command(about = "Move a command")]
    Move {
        path: Option<String>,
        new_path: Option<String>,
    },
    #[command(about = "Delete a command")]
    Delete { 
        path: Option<String>, 
        #[arg(short, long)]
        dir: bool 
    },
    List {},
}
