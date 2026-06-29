use cli_tool_rust::{search_case_insensitive, search_case_sensitive};
use std::{
    env,
    fs::{self, File},
    io::{BufRead, BufReader, Read},
};

struct Config {
    ignore_case: bool,
    file_path: String,
    query: String,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Self {
        args.next();

        let args_1 = args.next().expect("file path required");
        let args_2 = args.next().expect("search query is required");
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Self {
            ignore_case,
            file_path: args_1,
            query: args_2,
        }
    }
}

fn main() {
    // let commands = env::args();
    // let config = Config::build(commands);
    // let contents = fs::read_to_string(config.file_path)
    //     .expect("Failed to read file from the file path provided");
    // if config.ignore_case {
    //     let result = search_case_insensitive(&config.query, &contents);
    //     for line in result {
    //         println!("Line: {line}");
    //     }
    // } else {
    //     let result = search_case_sensitive(&config.query, &contents);
    //     for line in result {
    //         println!("Line: {line}");
    //     }
    // }
    let mut commands = env::args();
    commands.next();
    let file_path = commands.next().expect("No file path was provided.");
    let query = commands.next().expect("Missing argument: query");
    let file_names = fs::read_dir(&file_path)
        .unwrap_or_else(|_| panic!("Failed to read directory: {}", file_path));
    let mut files = Vec::new();
    for entry in file_names {
        if let Ok(dir_entry) = entry {
            println!("{:?}", dir_entry.path()); //"files/nigga.txt"
            files.push(dir_entry.path());
        }
    }
    for file in &files {
        let f = File::open(file).expect("failed to open file");
        let reader = BufReader::new(f);
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.contains(&query) {
                    println!("Query found in file: {file:?}");
                    break;
                }
            }
        }
    }
}
