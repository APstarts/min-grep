use cli_tool_rust::search_file;
use std::path::{Path, PathBuf};
use std::{
    env,
    fs::{self, File},
    io::{BufRead, BufReader, Read},
    sync::{Arc, Mutex, mpsc},
    thread,
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
    let mut commands = env::args(); //capturing the arguments with which the program was started.
    commands.next();
    let file_path = commands.next().expect("No file path was provided."); //parsing the file_path from the arguments with which which the program was started
    let query = Arc::new(commands.next().expect("Missing argument: query")); //parsing the search "query" with which the program was started to look for files which might contain this search query.
    let entries = fs::read_dir(&file_path) //reading the directory provided in the arguments to get the list of files inside the directory.
        .unwrap_or_else(|_| panic!("Failed to read directory: {}", file_path));
    let files: Vec<PathBuf> = entries
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect(); //storing the list of files with their complete path into a vector.
    // the following code was a bottle neck because when we do Vec::new() rust allocates zero memory. As you push items into it, the vector fills. Once it hits capacity, Rust has to:
    // Talk to the OS to allocate a new, larger chunk of memory(usually doubling the size), copy all the existing file paths from old memory location to the new one and then deallocate memory. This process continues multiple times which is expensive.
    // why the above code is better than the below commented code?
    // collect() uses an optimization called size hinting.
    // Every iterator in Rust implements a method called size_hint(). This method returns a tuple: (lower_bound, Option<upper_bound>), telling the consumer how many items are left in the iterator.

    // When you call .collect() on an iterator chain, Rust checks this size hint before allocating memory.

    // Your Manual Loop: Rust starts with a capacity of 0. It has no idea if you plan to push 1 item or 1,000,000 items. It has to guess, resize, copy, and reallocate incrementally as you push.

    // The collect() Method: The fs::read_dir iterator tells collect(), "Hey, the OS says there are exactly 1,000 files in this directory." Even though filter_map might discard a few broken entries, collect() looks at that upper limit and uses Vec::with_capacity(1000) under the hood.

    // Instead of resizing 10 or 15 times, collect() allocates the exact memory needed exactly once. No copying, no heap thrashing, no repeatedly bothering the OS.
    // ---commented the below inefficient code---
    // let mut files = Vec::new();
    // for entry in file_names {
    //     if let Ok(dir_entry) = entry {
    //         println!("{:?}", dir_entry.path()); //"files/nigga.txt"
    //         files.push(dir_entry.path());
    //     }
    // }
    // for file in &files {
    //     if search_file(file, &query) {
    //         println!("Query found in file: {file:?}");
    //     }
    // }
    //
    let number_of_cores = thread::available_parallelism().unwrap().get(); //get the number of logical cores of cpu available.
    let (tx, rx) = mpsc::channel(); //threadpool
    let rx = Arc::new(Mutex::new(rx)); //This allows multiple ownerships with Arc and Mutex allows locked access.
    for file in files {
        tx.send(file).expect("Failed to send job"); //sending the jobs to channel as a que to the threadpool
    }
    drop(tx);

    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
    //creating 4 workers
    for _ in 0..number_of_cores {
        let rx = Arc::clone(&rx);

        let worker_query = Arc::clone(&query);

        let handle = thread::spawn(move || {
            loop {
                let job = rx.lock().unwrap().recv();

                match job {
                    Ok(file) => {
                        if search_file(&file, &worker_query) {
                            println!("Found in {:?}", file);
                        }
                    }

                    Err(_) => break,
                }
            }
        });

        handles.push(handle);
    }

    //waiting for the workers to complete their jobs
    for handle in handles {
        handle.join().unwrap();
    }
}
