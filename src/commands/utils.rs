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
