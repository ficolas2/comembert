use crate::{commands::utils::get_command_path, shell::{get_current_shell, Shell}};

pub fn edit_command(editor: &str, context: &str, path: Option<&str>) {
    let path = if let Some(path) = path {
        path
    } else {
        &get_command_path(context)
    };
    if path.is_empty() {
        println!("No command selected.");
        return;
    }

    println!("Editing command: {}", path);
    let path = format!("{}/{}", context, path);

    if !std::path::Path::new(&path).exists() {
        eprintln!("Command does not exist");
        std::process::exit(1);
    }

    let mut command = std::process::Command::new(editor);
    command.arg(path);

    if editor == "vim" || editor == "nvim" {
        match get_current_shell() {
            Shell::Zsh => command.args(["-c", "set syntax=zsh"]),
            Shell::Fish => command.args(["-c", "set syntax=fish"]),
            Shell::Bash => command.args(["-c", "set syntax=bash"]),
        };
    }

    command.status()
        .unwrap_or_else(|_| panic!("failed to execute {}", editor));
}
