use std::{collections::HashMap, process::Command};

use crate::shell::insert_text;

use super::{
    metadata::metadata::{Metadata, SelectorType},
    utils::{fzf_select, get_command_path},
};

pub fn insert_command(context: &str, path: Option<&str>) {
    let path = path
        .map(|s| s.to_string())
        .unwrap_or_else(|| get_command_path(context));
    if path.is_empty() {
        println!("No command selected.");
        return;
    }
    let command_path = format!("{}/{}", context, path);

    match std::fs::read_to_string(command_path) {
        Ok(command) => {
            process_command(command);
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                println!("Command does not exist");
            } else {
                println!("Error reading command: {}", e);
            }
        }
    }
}

fn process_command(command: String) {
    let parts: Vec<&str> = command.splitn(2, "\n\n\n").collect();
    let mut command = parts[0].trim_end().to_string();
    let meta_str = parts.get(1);

    if let Some(meta_str) = meta_str {
        let meta: Metadata = toml::from_str(meta_str).expect("Failed to deserialize config");
        command = process_selectors(command, &meta);
    }

    insert_text(&command);
}

fn process_selectors(mut command: String, meta: &Metadata) -> String {
    for selector in meta.selectors.iter() {
        // Obtain a list of strings
        // The strings are separated by the delimiter, and should contan as many elements as
        // there are slots
        let slot_content: Vec<String> = match selector.selector_type {
            SelectorType::Command => {
                let output = Command::new("bash")
                    .arg("-c")
                    .arg(selector.command.as_ref().expect(""))
                    .output()
                    .expect("Failed to execute selector command");

                if !output.status.success() {
                    panic!("Selector command failed: {}", output.status)
                }

                String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .to_string()
                    .split("\n")
                    .map(|s| s.to_string())
                    .collect()
            }
            SelectorType::List => selector.list.clone(),
        };

        // Obtain the lists of slot cotents
        let slot_content: Vec<Vec<&str>> = if selector.slots.len() > 1 {
            let delim = selector.delimiter.as_deref().unwrap_or("  ");
            slot_content
                .iter()
                .map(|s| s.split(delim).collect())
                .collect()
        } else {
            slot_content.iter().map(|s| vec![s.as_str()]).collect()
        };

        // Get the fzf string
        let fzf_string = selector
            .fzf_string
            .as_ref()
            .expect("Missing fzf_string field in metadata");

        let map: HashMap<String, &Vec<&str>> = slot_content
            .iter()
            .map(|s| (replace_slots(&fzf_string, &selector.slots, s), s))
            .collect();

        let res = fzf_select(map.keys().cloned().collect());

        // TODO this unwrap panics on cancel
        command = replace_slots(&command, &selector.slots, map.get(&res).unwrap()).to_string();
    }

    command
}

fn replace_slots(string: &str, slot_label_list: &Vec<String>, slot_list: &[&str]) -> String {
    let mut string = string.to_string();
    for (i, slot_label) in slot_label_list.iter().enumerate() {
        string = string.replace(
            &format!("<{}>", slot_label),
            slot_list.get(i).expect(&format!(
                "More slots specified in metadata than in the slot content list: {:?}",
                slot_list
            )),
        )
    }

    string
}
