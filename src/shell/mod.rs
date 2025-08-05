use std::{
    env,
    io::Write,
    process::{Command, Stdio},
};

use sysinfo::{get_current_pid, System};

use crate::constants::{CLIPBOARD_OUTPUT, OUTPUT_ENV_VAR, TMP_COMMAND_FILE, TMP_OUTPUT};

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
        Shell::Zsh => zsh::get_last_command(),
        _ => panic!("Currently unsuported shell"),
    }
}

fn insert_text_tmp(text: &str) {
    let mut file = std::fs::File::create(TMP_COMMAND_FILE)
        .unwrap_or_else(|_| panic!("Failed to create {}", TMP_COMMAND_FILE));
    file.write_all(text.as_bytes())
        .unwrap_or_else(|_| panic!("Failed to write to {}", TMP_COMMAND_FILE));
}

fn insert_text_clipboard(text: &str) {
    if env::var("XAUTHORITY").is_ok() {
        let mut child = Command::new("xclip")
            .arg("-selection")
            .arg("clipboard")
            .stdin(Stdio::piped())
            .spawn()
            .expect("Failed to start xclip");
        child.wait().expect("Failed to wait for xclip process");

        if let Some(stdin) = child.stdin.as_mut() {
            stdin
                .write_all(text.as_bytes())
                .expect("Failed to write to xclip");
        }

        let mut child = Command::new("xdotool")
            .arg("key")
            .arg("ctrl+shift+v")
            .spawn()
            .expect("Failed to execute xdotool");

        child.wait().expect("Failed to wait for xdotool process");
    } else {
        panic!("Not using X11, run from the fish or zsh script.")
    }
}

pub fn insert_text(text: &str) {
    let output = env::var(OUTPUT_ENV_VAR);
    match output {
        Ok(ref value) if value == TMP_OUTPUT => insert_text_tmp(text),
        Ok(ref value) if value == CLIPBOARD_OUTPUT => insert_text_clipboard(text),
        _ => {
            println!("Running without using any of the scripts. Cannot insert.\n\
                    If using bash or zsh, to add a command to the command line, you need to run it using a keybinding (by default, Ctrl+F)\n\
                    More info about this in the '#Insertion options' section of the README.md.\n");
            println!("This is your command:\n{}", text)
            // insert_text_clipboard(text)
        }
    }
}
