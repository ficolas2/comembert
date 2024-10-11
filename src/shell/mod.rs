use std::process::Command;

use sysinfo::{get_current_pid, System};

pub mod bash;
pub mod fish;
pub mod zsh;

#[derive(Debug, PartialEq)]
pub enum Shell {
    Bash,
    Fish,
    Zsh,
}

pub fn get_current_shell() -> Shell {
    let mut sys = System::new_all();
    sys.refresh_all();

    let pid = get_current_pid().expect("Failed to get current process ID, platform not supported");

    let process = sys.process(pid).unwrap();
    let parent_pid = process
        .parent()
        .expect("Failed to get parent process ID. Not running from a shell?");
    let parent_process = sys.process(parent_pid).unwrap();
    match parent_process.name().to_str().unwrap() {
        "bash" => Shell::Bash,
        "fish" => Shell::Fish,
        "zsh" => Shell::Zsh,
        _ => panic!("Unsupported shell, only bash, fish and zsh are supported"),
    }
}

pub fn get_last_command() -> String {
    let shell = get_current_shell();
    match shell {
        // Shell::Bash => bash::get_last_command(),
        Shell::Fish => fish::get_last_command(),
        // Shell::Zsh => zsh::get_last_command(),
        _ => panic!("Currently unsuported shell"),
    }
}

pub fn insert_text(text: &str) {
    Command::new("xdotool")
        .arg("type")
        .arg(text)
        .spawn()
        .expect("Failed to execute xdotool");
}
