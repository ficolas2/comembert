use std::process::Command;

use fzf_wrapped::Fzf;

fn get_files(path: &str) -> Vec<String> {
    let mut files: Vec<String> = vec![];
    let paths = std::fs::read_dir(path).expect("Failed to read directory");
    for path in paths {
        let path = path.expect("Failed to get path").path();
        if path.is_file() {
            files.push(path.to_str().unwrap().to_string());
        }
        if path.is_dir() {
            files.append(get_files(path.to_str().unwrap()).as_mut());
        }
    }

    files
}

pub fn insert_command(context: &str) {
    let prefix = format!("{}/", context);
    let command_list: Vec<String> = get_files(context)
        .iter()
        .map(|p| p.strip_prefix(&prefix).unwrap().to_string())
        .collect();

    let mut fzf = Fzf::default();
    fzf.run().expect("Failed to start fzf");
    fzf.add_items(command_list)
        .expect("Failed to add items to fzf");
    let users_selection = fzf.output().expect("Failed to get the user's output");

    // read file in users_selection
    let command = std::fs::read_to_string(format!("{}/{}", context, users_selection))
        .expect("Failed to read command file");
    let command = command.trim_end();

    Command::new("xdotool")
        .arg("type")
        .arg(command)
        .spawn()
        .expect("Failed to execute xdotool");
}
