use std::fs;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// JSON file with tasks list.
    #[arg(short, long, value_name = "FILE")]
    tasks_path: PathBuf,

}

fn main() {
    let cli = Cli::parse();


    println!("Value for config: {:#}", cli.tasks_path.display());

    let json_data = fs::read_to_string("tasks.json").unwrap();
    let task_list: Vec<String> = serde_json::from_str(&json_data).unwrap();
    println!("{:?}", task_list);
}