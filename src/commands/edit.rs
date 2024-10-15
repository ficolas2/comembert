use crate::{commands::utils::get_command_path, shell::{get_current_shell, Shell}};

pub fn edit_command(editor: &str, context: &str, path: Option<&str>) {
    let path = if let Some(path) = path {
        path
    } else {
        &get_command_path(context)
    };
    if path == "" {
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
            Shell::Zsh => command.arg("-c set syntax=zsh"),
            Shell::Fish => command.arg("-c set syntax=fish"),
            Shell::Bash => command.arg("-c set syntax=bash"),
        };
    }

    command.status()
        .expect(&format!("failed to execute {}", editor));
}
