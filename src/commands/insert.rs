use std::process::Command;

use super::utils::get_command_path;

pub fn insert_command(context: &str) {
    let command_path = get_command_path(context);
    let command_path = format!("{}/{}", context, command_path);

    let command = std::fs::read_to_string(command_path)
        .expect("Failed to read command file");
    let command = command.trim_end();

    Command::new("xdotool")
        .arg("type")
        .arg(command)
        .spawn()
        .expect("Failed to execute xdotool");
}