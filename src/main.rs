#[allow(unused_imports)]
use std::fs::{self, File};
use std::io::{self};
use std::path::Path;

fn main() {
    loop {
        println!("Enter a command (list, create, delete, exit):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let command = input.trim();

        match command {
            "list" => {
                println!("Enter the directory path:");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let path = input.trim();
                let dir = Path::new(path);
                if dir.is_dir() {
                    for entry in fs::read_dir(dir).expect("Failed to read directory") {
                        if let Ok(entry) = entry {
                            println!("{}", entry.path().display());
                        }
                    }
                } else {
                    println!("{} is not a directory", path);
                }
            }
            "create" => {
                println!("Enter the file path:");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let path = input.trim();
                File::create(path).expect("Failed to create file");
                println!("Created file {}", path);
            }
            "delete" => {
                println!("Enter the file path:");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let path = input.trim();
                fs::remove_file(path).expect("Failed to delete file");
                println!("Deleted file {}", path);
            }
            "exit" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid command");
            }
        }
    }
}

