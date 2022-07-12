use std::{env, fs};

fn main() {
    let arg_result = env::args().nth(1);
    let current_dir_result = env::current_dir();

    // If path specified, give priority
    // If no path specified, try working directory
    // If working directory failed, ask for path and quit
    match (arg_result, current_dir_result) {
        (Some(path), _) => read_dir(path),
        (None, Ok(current_dir)) => read_dir(current_dir.to_str().unwrap().into()),
        _ => eprintln!("Please specify a directory path"),
    }
}

fn read_dir(path: String) {
    // Read directory's contents
    let read_dir_result = fs::read_dir(path);

    match read_dir_result {
        Ok(dir_contents) => {
            let mut count = 0;
            let mut final_output: Vec<String> = vec![];

            // Parse entries and count # of entries
            for dir_entry in dir_contents {
                if let Ok(entry) = dir_entry {
                    count += 1;
                    final_output.push(entry.file_name().to_str().unwrap().into());
                }
            }

            // Output entry count & entries
            println!("Found {count} entries\n");
            for item in final_output.iter() {
                println!("{item}");
            }
        }
        Err(e) => {
            eprintln!("{}", e.to_string());
        }
    }
}
