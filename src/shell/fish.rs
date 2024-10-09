use core::str;



pub fn get_last_command() -> String{
    let output = std::process::Command::new("fish")
        .arg("-c")
        .arg("history --max 1")
        .output()
        .expect("Failed to execute fish history command");

    let last_command = str::from_utf8(&output.stdout).unwrap();

    let output = std::process::Command::new("fish")
        .arg("-c")
        .arg("history --max 2")
        .output()
        .expect("Failed to execute fish history command");

    let last_two_commands = str::from_utf8(&output.stdout).unwrap();

    last_two_commands.strip_prefix(last_command).unwrap().strip_suffix("\n").unwrap().to_string()
}
