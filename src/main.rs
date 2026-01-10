use std::fs::File;
use std::{fs, io::Write};

use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

use anyhow::Result;

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
    List,

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

fn write_to_file(file_path: &Path, tasks: &Vec<String>) -> Result<()> {
    let mut file: File = File::create(file_path)?;
    let json_data = serde_json::to_string_pretty(&tasks)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}

fn print_tasks(tasks: &Vec<String>) {
    println!("------------------");
    println!("Tasks:");
    for task in tasks {
        println!("- {task}");
    }
    println!("------------------");
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let data = match fs::read_to_string(&cli.tasks_path) {
        Ok(data) => data,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => "[]".to_string(),
        Err(e) => {
            return Err(e.into());
        }
    };
    let mut tasks: Vec<String> = serde_json::from_str(&data)?;

    match &cli.command {
        Some(Commands::List) => {
            print_tasks(&tasks);
        }
        Some(Commands::Add { name }) => {
            tasks.push(name.to_string());
            write_to_file(&cli.tasks_path, &tasks)?;
        }
        Some(Commands::Remove { name }) => {
            match tasks.iter().position(|x| x == name) {
                Some(index) => {
                    tasks.remove(index);
                    write_to_file(&cli.tasks_path, &tasks)?;
                }
                None => {
                    println!("No such tusk in a list !");
                }
            };
        }
        None => {}
    }

    Ok(())
}
