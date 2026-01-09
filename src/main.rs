use std::fs::File;
use std::{fs, io::Write};

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// JSON file with tasks list.
    #[arg(short, long, value_name = "FILE")]
    tasks_path: PathBuf,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all tasks.
    List {},

    /// Add new task to list.
    Add {
        /// Name of the task
        #[arg(short, long)]
        name: String,
    },

    /// Remove task from list.
    Remove {
        /// Name of the task
        #[arg(short, long)]
        name: String,
    },
}

fn write_to_file(file_path: &PathBuf, tasks: Vec<String>) {
    match File::create(file_path) {
        Ok(mut file) => {
            let json_data = serde_json::to_string_pretty(&tasks).unwrap();
            file.write_all(json_data.as_bytes()).unwrap();
        }
        Err(e) => {
            panic!("Could not read the file ! - {e}");
        }
    };
}

fn main() {
    let cli = Cli::parse();

    let data: String = match fs::read_to_string(&cli.tasks_path) {
        Ok(data) => data,
        Err(e) => {
            panic!("Could not read the file ! - {e}");
        }
    };

    let mut tasks: Vec<String> = match serde_json::from_str(&data) {
        Ok(data) => data,
        Err(e) => {
            panic!("Could not read the file ! - {e}");
        }
    };

    match &cli.command {
        Some(Commands::List {}) => {
            println!("Current tasks list: ");
            for task in &tasks {
                println!("- {task}");
            }
        }
        Some(Commands::Add { name }) => {
            tasks.push(name.to_string());
            write_to_file(&cli.tasks_path, tasks)
        }
        Some(Commands::Remove { name }) => {
            match tasks.iter().position(|x| x == name) {
                Some(index) => {
                    tasks.remove(index);
                    write_to_file(&cli.tasks_path, tasks)
                }
                None => {
                    println!("No such tusk in a list !")
                }
            };
        }
        None => {
            println!("Not action were selected!");
        }
    }
}
