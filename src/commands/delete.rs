use super::utils::get_command_path;

pub fn delete_command(context: &str, path: Option<&str>) {
    let path = path.map(|s| s.to_string()).unwrap_or_else(|| {
        get_command_path(context)
    });

    let path = format!("{}/{}", context, path);

    if !std::path::Path::new(&path).exists() {
        eprintln!("Command does not exist");
        std::process::exit(1);
    }

    std::fs::remove_file(&path).expect("Failed to delete command");
}
