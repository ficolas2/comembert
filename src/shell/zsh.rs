use std::fs;

pub fn get_last_command() -> String {
    let hist_path = dirs::home_dir().unwrap().join(".zsh_history");
    let bytes = fs::read(hist_path).expect("Could not read zsh history");

    let contents = String::from_utf8_lossy(&bytes);
    let mut lines = contents
        .lines()
        .rev()
        .filter(|line| !line.trim().is_empty());

    let _ = lines.next();
    if let Some(last_command) = lines.next() {
        last_command.split(";").nth(1).unwrap().to_string()
    } else {
        String::new()
    }
}
