use std::fs;

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

fn main() {
    let cli = Cli::parse();

    let data: String = match fs::read_to_string(cli.tasks_path) {
        Ok(data) => data,
        Err(_) => {
            println!("Could not read the file !");
            return;
        }
    };

    let tasks: Vec<String> = match serde_json::from_str(&data){
        Ok(data) => data,
        Err(_) => {
            println!("Could not parse the file !");
            return;
        }
    };

    match &cli.command {
        Some(Commands::List {}) => {
            println!("Current tasks list: ");
            for task in &tasks{
                println!("- {task}");
            }
        }
        Some(Commands::Add { name }) => {

        }
        Some(Commands::Remove { name }) => {

        }
        None => {
            println!("Not action were selected!");
        }
    }



}