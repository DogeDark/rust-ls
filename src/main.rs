use std::{env, fs, io};

fn main() {
    let arg_result = env::args().nth(1);
    let current_dir_result = env::current_dir();

    // If path specified, give priority
    // If no path specified, try working directory
    // If working directory failed, ask for path and quit
    let read_result = match (arg_result, current_dir_result) {
        (Some(path), _) => read_dir(&path),
        (None, Ok(current_dir)) => read_dir(current_dir.to_str().unwrap()),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Please specify a directory path",
        )),
    };

    match read_result {
        Ok(file_list) => {
            let file_count = file_list.len();
            println!("Found {file_count} entries\n---------------");

            for file_name in file_list.iter() {
                println!("{file_name}");
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}

fn read_dir(path: &str) -> Result<Vec<String>, io::Error> {
    let mut file_list = Vec::new();

    let dir_contents = fs::read_dir(path)?;
    for file in dir_contents {
        if let Ok(file) = file {
            if let Some(file_name) = file.file_name().to_str() {
                file_list.push(file_name.to_string());
            }
        }
    }

    Ok(file_list)
}
