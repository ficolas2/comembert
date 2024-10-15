use std::io::{self, Write};

use fzf_wrapped::{Fzf, Layout};

pub const BLACK: &str = "\x1b[30m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[37m";

pub fn save_command(command: &str, path: &str, context: &str) {
    let path = format!("{}/{}", context, path);
    let path = std::path::Path::new(&path);
    std::fs::create_dir_all(path.parent().unwrap()).expect("Failed to create command directory");
    std::fs::write(&path, command).expect("Failed to write command to file");
}

pub fn read_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    print!("    ");
    io::stdout().flush().expect("Failed to flush");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");
    input.trim().to_string()
}

fn get_files(path: &str) -> Vec<String> {
    let mut files: Vec<String> = vec![];
    std::fs::create_dir_all(&path).expect("Failed to create commands directory");
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

    let height = (command_list.len() + 3).min(25);
    let height_arg = format!("--height={}", height);

    let mut fzf = Fzf::builder()
        .layout(Layout::Reverse)
        .custom_args(vec![height_arg])
        .build()
        .expect("Failed to build fzf");
    fzf.run().expect("Failed to start fzf");
    fzf.add_items(command_list)
        .expect("Failed to add items to fzf");
    let users_selection = fzf.output().expect("Failed to get the user's output");

    users_selection
}

#[derive(PartialEq, Debug)]
enum Quote {
    Single,
    Double,
    None,
}

fn print_line(line: &str, quote: Quote) -> Quote {
    if quote == Quote::None {
        let single_quote_index = line.find('\'');
        let double_quote_index = line.find('"');

        let (quote, quote_index) = match (single_quote_index, double_quote_index) {
            (Some(single_quote_index), Some(double_quote_index)) => {
                if single_quote_index > double_quote_index {
                    (Quote::Double, double_quote_index)
                } else {
                    (Quote::Single, single_quote_index)
                }
            }
            (Some(single_quote_index), None) => (Quote::Single, single_quote_index),
            (None, Some(double_quote_index)) => (Quote::Double, double_quote_index),
            _ => {
                print!("{}{}", CYAN, line);
                return Quote::None;
            }
        };
        print!(
            "{}{}{}{}",
            CYAN,
            &line[..quote_index],
            YELLOW,
            &line[quote_index..quote_index + 1]
        );
        return print_line(&line[quote_index + 1..], quote);
    } else {
        let quote_char = match quote {
            Quote::Single => '\'',
            Quote::Double => '"',
            Quote::None => unreachable!(),
        };

        let quote_index = line.find(quote_char);
        if let Some(quote_index) = quote_index {
            print!("{}{}", YELLOW, &line[..quote_index]);
            return print_line(&line[quote_index..], Quote::None);
        } else {
            print!("{}{}", YELLOW, line);
            return quote;
        }
    }
}

pub fn print_command(command: &str, indent: usize) {
    let mut first_line = true;

    let mut quote = Quote::None;

    let total_lines = command.lines().count();
    for (i, line) in command.lines().enumerate() {
        let mut beginning = 0;
        print!("{:indent$}", " ", indent = indent);
        if first_line {
            // Print the command in blue
            print!("{}", BLUE);
            beginning = line.find(' ').unwrap_or(line.len());
            print!("{}", &line[..beginning]);

            first_line = false;
        }
        quote = print_line(&line[beginning..], quote);
        if i != total_lines - 1 {
            println!("");
        }
    }

    println!("{}", WHITE);
}
