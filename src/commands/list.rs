use std::collections::HashMap;

use super::utils::get_commands;

pub fn print_commands(commands: &Vec<String>, indent: usize) {
    let mut parent_map: HashMap<String, Vec<String>> = HashMap::new();
    for command in commands {
        if let Some((parent, child)) = command.split_once("/") {
            parent_map.entry(parent.to_string()).or_default().push(child.to_string());
        } else {
            println!("{:indent$}{}", "", command, indent = indent);
        }
    }

    for (parent, children) in parent_map {
        println!("{:indent$}/{}", "", parent, indent = indent);
        print_commands(&children, indent + 4);
    }
}

pub fn list_commands(context: &str) {
    let commands = get_commands(context);

    print_commands(&commands, 0);
}
