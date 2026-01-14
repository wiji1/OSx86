use std::env;
use std::fs::File;
use std::io::{Error, Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut parsed_args = parse_args(args);

    if parsed_args.modifiers.is_empty() && parsed_args.char_modifiers.is_empty() {
        parsed_args.char_modifiers.push('w');
        parsed_args.char_modifiers.push('c');
        parsed_args.char_modifiers.push('m');
        parsed_args.char_modifiers.push('l');
    }

    if parsed_args.modifiers.contains(&"version".to_string()) {
        display_version();

        return;
    }

    if parsed_args.modifiers.contains(&"help".to_string()) {
        display_help();

        return;
    }

    let mut content = String::new();

    match parsed_args.file {
        Some(ref file) => {
            let file_content = open_file(&file);

            match file_content {
                Ok(file_content) => content = file_content,
                Err(err) => panic!("{}", err),
            }
        }
        None => {
            panic!("No input file provided")
        }
    }

    let mut output: String = String::new();
    let mut combined_modifiers: Vec<String> = Vec::new();

    combined_modifiers.extend(parsed_args.modifiers.into_iter());
    combined_modifiers.extend(
        parsed_args.char_modifiers.into_iter().map(|s| s.to_string())
    );

    for modifier in combined_modifiers {

        match modifier {
            s if s == "c" || s.contains("bytes") => {
                output.push_str(format!("{}", &content.bytes().count()).as_str());
                output.push_str("   ");
            },
            s if s == "m" || s.contains("chars") => {
                output.push_str(format!("{}",  &content.chars().count()).as_str());
                output.push_str("   ");
            },
            s if s == "l" || s.contains("lines") => {
                output.push_str(format!("{}",  &content.lines().count()).as_str());
                output.push_str("   ");
            }
            s if s == "w" || s.contains("words") => {
                output.push_str(format!("{}",  &content.split_whitespace().count()).as_str());
                output.push_str("   ");
            }
            _ => {}
        }
    }

    output.push_str(format!("{}", parsed_args.file.unwrap()).as_str());
    println!("{}", output);
}

fn parse_args(args: Vec<String>) -> Arguments {
    let mut modifiers: Vec<String> = Vec::new();
    let mut char_modifiers: Vec<char> = Vec::new();
    let mut file: Option<String> = None;

    for arg in args.into_iter() {
        match arg {
            s if s.starts_with("--") => modifiers.push(s[2..].to_string()),
            s if s.starts_with("-") => {
                for char in s[1..].chars() {
                    char_modifiers.push(char);
                }
            }
            s => file = Some(s.to_string())
        }
    }

    Arguments {
        modifiers,
        char_modifiers,
        file
    }
}

struct Arguments {
    modifiers: Vec<String>,
    char_modifiers: Vec<char>,
    file: Option<String>,
}

fn open_file(path: &String) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn display_version() {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

fn display_help() {
    print!("Help menu!");
}
