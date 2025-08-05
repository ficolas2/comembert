use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    version, 
    about, 
    long_about = None,
    args_conflicts_with_subcommands = true
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(
        short = 'c',
        long = "command",
        help = "Inserts the specified command into the shell"
    )]
    pub insert_command: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Add a command", visible_aliases = &["a"])]
    Add { command: String, path: String },
    #[command(about = "Save the last command", visible_aliases = &["p", "prev"])]
    Previous { path: Option<String> },
    #[command(about = "Edit a command", visible_aliases = &["e"])]
    Edit { path: Option<String> },
    #[command(about = "Move a command", visible_aliases = &["mv"])]
    Move {
        path: Option<String>,
        new_path: Option<String>,
    },
    #[command(about = "Delete a command", visible_aliases = &["d", "del"])]
    Delete {
        path: Option<String>,
        #[arg(short, long)]
        dir: bool,
    },
    #[command(about = "List all stored commands", visible_aliases = &["l", "ls"])]
    List {},
}
