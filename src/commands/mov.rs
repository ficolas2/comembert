use crate::commands::utils::read_user_input;

use super::utils::get_command_path;

pub fn move_command(context: &str, path: Option<&str>, new_path: Option<&str>) {
    let path = path.map(|s| s.to_string()).unwrap_or_else(|| {
        get_command_path(context)
    });

    if path.is_empty() {
        println!("No command selected.");
        return;
    }

    println!("Moving command: {}", path);
    let path = format!("{}/{}", context, path);

    let new_path = new_path.map(|s| s.to_string()).unwrap_or_else(|| {
        read_user_input("Enter the new path for the command:")
    });
    println!("To path: {}", new_path);
    let new_path = format!("{}/{}", context, new_path);

    if !std::path::Path::new(&path).exists() {
        eprintln!("Command does not exist");
        std::process::exit(1);
    }

    std::fs::rename(&path, &new_path).expect("Failed to move command");
}
