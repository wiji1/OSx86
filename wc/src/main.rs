use std::{env, io};
use std::fs::File;
use std::io::{Error, Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    let arg_result = parse_args(args[1..].to_vec());
    let mut parsed_args = match arg_result {
        Ok(parsed_args) => parsed_args,
        Err(err) => panic!("{}", err),
    };

    if parsed_args.modifiers.is_empty() && parsed_args.char_modifiers.is_empty() {
        parsed_args.char_modifiers.push('l');
        parsed_args.char_modifiers.push('w');
        parsed_args.char_modifiers.push('c');
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

    if parsed_args.files.is_empty() {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        content = input;
    } else {
        for file in &parsed_args.files {
            content.push_str(&open_file(&file).unwrap());
            content.push('\n');
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
                output.push_str("     ");
            },
            s if s == "m" || s.contains("chars") => {
                output.push_str(format!("{}",  &content.chars().count()).as_str());
                output.push_str("     ");
            },
            s if s == "l" || s.contains("lines") => {
                output.push_str(format!("{}",  &content.lines().count()).as_str());
                output.push_str("     ");
            }
            s if s == "w" || s.contains("words") => {
                output.push_str(format!("{}",  &content.split_whitespace().count()).as_str());
                output.push_str("     ");
            },
            s if s == "L" || s.contains("max-line-length") => {
                output.push_str(format!("{}", get_longest_line_length(&content)).as_str());
                output.push_str("     ");
            }
            _ => {}
        }
    }

    if parsed_args.files.len() == 1 {
        output.push_str(format!("{}", parsed_args.files[0]).as_str());
    }
    println!("{}", output);
}

fn parse_args(args: Vec<String>) -> Result<Arguments, Error> {
    let mut modifiers: Vec<String> = Vec::new();
    let mut char_modifiers: Vec<char> = Vec::new();
    let mut files: Vec<String> = Vec::new();

    for arg in args.into_iter() {
        match arg {
            s if s.starts_with("--files0-from=") => {
                let content = open_file(&s[14..].to_string())?;
                content.lines().for_each(|l| files.push(l.to_string()));
            },
            s if s.starts_with("--") => modifiers.push(s[2..].to_string()),
            s if s.starts_with("-") => {
                for char in s[1..].chars() { char_modifiers.push(char); }
            },
            s => {
                if files.len() == 0 { files.push(s.to_string()) }
            }
        }
    }

    Ok(Arguments {
        modifiers,
        char_modifiers,
        files
    })
}

struct Arguments {
    modifiers: Vec<String>,
    char_modifiers: Vec<char>,
    files: Vec<String>,
}

fn open_file(path: &String) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn get_longest_line_length(content: &String) -> usize {
    let mut line_length: usize = 0;

    for line in content.lines() { if line.len() > line_length { line_length = line.len(); } }

    line_length
}

fn display_version() {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

fn display_help() {
    print!("
        Usage: wc [OPTION]... [FILE]...
          or:  wc [OPTION]... --files0-from=F
        Print newline, word, and byte counts for each FILE, and a total line if
        more than one FILE is specified.  A word is a non-zero-length sequence of
        characters delimited by white space.

        With no FILE, or when FILE is -, read standard input.

        The options below may be used to select which counts are printed, always in
        the following order: newline, word, character, byte, maximum line length.
          -c, --bytes            print the byte counts
          -m, --chars            print the character counts
          -l, --lines            print the newline counts
              --files0-from=F    read input from the files specified by
                                   NUL-terminated names in file F;
                                   If F is - then read names from standard input
          -L, --max-line-length  print the maximum display width
          -w, --words            print the word counts
              --help     display this help and exit
              --version  output version information and exit

        GNU coreutils online help: <https://www.gnu.org/software/coreutils/>
        Report any translation bugs to <https://translationproject.org/team/>
        Full documentation <https://www.gnu.org/software/coreutils/wc>
        or available locally via: info '(coreutils) wc invocation'
    ");
}
