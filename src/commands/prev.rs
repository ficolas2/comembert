use crate::commands::utils::{print_command, read_user_input, save_command};
use crate::shell::get_last_command;

pub fn save_prev_command(path: Option<String>, context: &str) {
    let last_command = get_last_command();
    println!("Saving last command:");
    print_command(&last_command, 4);
    let path = if let Some(path) = path {
        path
    } else {
        read_user_input("Enter the path: ")
    };
    save_command(&last_command, &path, context);
}
