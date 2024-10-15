use crate::shell::insert_text;

use super::utils::get_command_path;

pub fn insert_command(context: &str, path: Option<&str>) {
    let path = path.map(|s| s.to_string()).unwrap_or_else(|| {
        get_command_path(context)
    });
    if path == "" {
        println!("No command selected.");
        return;
    }
    let command_path = format!("{}/{}", context, path);

    match std::fs::read_to_string(command_path) {
        Ok(command) => {
            let command = command.trim_end();
            insert_text(command);
        },
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                println!("Command does not exist");
            } else {
                println!("Error reading command: {}", e);
            }
        }
    }
}
