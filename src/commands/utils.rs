use fzf_wrapped::Fzf;

pub fn save_command(command: &str, path: &str, context: &str) {
    let path = format!("{}/{}", context, path);
    std::fs::write(&path, command).expect("Failed to write command to file");
}

pub fn read_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");
    input.trim().to_string()
}

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

pub fn get_commands(context: &str) -> Vec<String> {
    let prefix = format!("{}/", context);
    get_files(context)
        .iter()
        .map(|p| p.strip_prefix(&prefix).unwrap().to_string())
        .collect()
}

pub fn get_command_path(context: &str) -> String {
    let command_list = get_commands(context);

    let mut fzf = Fzf::default();
    fzf.run().expect("Failed to start fzf");
    fzf.add_items(command_list)
        .expect("Failed to add items to fzf");
    let users_selection = fzf.output().expect("Failed to get the user's output");

    users_selection
}
