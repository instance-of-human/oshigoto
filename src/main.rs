use std::{fs, io::Write};
use std::fs::File;

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

fn write_to_file(&file_path: PathBuf, tasks: Vec<String){
    
}

fn main() {
    let cli = Cli::parse();

    let data: String = match fs::read_to_string(&cli.tasks_path) {
        Ok(data) => data,
        Err(_) => {
            println!("Could not read the file !");
            return;
        }
    };

    let mut tasks: Vec<String> = match serde_json::from_str(&data) {
        Ok(data) => data,
        Err(_) => {
            println!("Could not parse the file !");
            return;
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

            match File::create(&cli.tasks_path){
                Ok(mut file) => {
                    let json_data = serde_json::to_string_pretty(&tasks).unwrap();
                    file.write_all(json_data.as_bytes()).unwrap();
                    },
                Err(e) => {
                    println!("Error while reading the file!");
                    return;
                }
            };



        }
        Some(Commands::Remove { name }) => {
        }
        None => {
            println!("Not action were selected!");
        }
    }
}
