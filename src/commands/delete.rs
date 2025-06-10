use super::utils::get_command_path;

pub fn delete_command(context: &str, path: Option<&str>, dir: bool) {
    let path = path.map(|s| s.to_string()).unwrap_or_else(|| {
        get_command_path(context)
    });
    if path.is_empty() {
        println!("No command selected.");
        return;
    }

    let path = format!("{}/{}", context, path);

    if !std::path::Path::new(&path).exists() {
        eprintln!("Command does not exist");
        std::process::exit(1);
    }

    // check if its a directory
    if std::fs::metadata(&path).unwrap().is_dir() {
        if !dir {
            eprintln!("Command is a directory, use -d to delete");
            std::process::exit(1);
        } else {
            std::fs::remove_dir_all(&path).expect("Failed to delete command");
            return;
        }
    }

    std::fs::remove_file(&path).expect("Failed to delete command");
}
