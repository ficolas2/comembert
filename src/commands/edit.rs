use crate::commands::utils::get_command_path;

pub fn edit_command(editor: &str, context: &str, path: Option<&str>) {
    let path = if let Some(path) = path {
        path
    } else {
        &get_command_path(context)
    };
    println!("Editing command: {}", path);
    let path = format!("{}/{}", context, path);

    if !std::path::Path::new(&path).exists() {
        eprintln!("Command does not exist");
        std::process::exit(1);
    }

    std::process::Command::new(editor)
        .arg(path)
        .status()
        .expect(&format!("failed to execute {}", editor));
}
