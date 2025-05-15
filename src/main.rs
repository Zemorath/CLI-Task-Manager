use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct TaskList {
    tasks: Vec<Task>,
    next_id:u32,
}

#[derive(Parser)]
#[clap(name = "tasker", about = "A simple CLI task manager")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { description: String },
    List,
    Update { id: u32, description: String },
    Delete { id: u32 },
    Complete { id: u32},
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let mut task_list = load_tasks()?;

    match cli.command {
        Commands::Add { description } => {
            let task = Task {
                id: task_list.next_id,
                description,
                completed: false,
            };
            task_list.tasks.push(task);
            task_list.next_id += 1;
            save_tasks(&task_list)?;
            println!("Task added: {}", task_list.next_id - 1);
        }
        Commands::List => {
            if task_list.tasks.is_empty() {
                println!("No tasks found.");
            } else {
                for task in &task_list.tasks {
                    let status = if task.completed { "[x]" } else { "[ ]" };
                    println!("{} {}: {}", status, task.id, task.description);
                }
            }
        }
        Commands::Update { id, description} => {
            if let Some(task) = task_list.tasks.iter_mut().find(|t| t.id == id) {
                task.description = description;
                save_tasks(&task_list)?;
                println!("Task {} updated.", id);
            } else {
                println!("Task {} not found.", id);
            }
        }
        Commands::Delete { id } => {
            if let Some(pos) = task_list.tasks.iter().position(|t| t.id == id) {
                task_list.tasks.remove(pos);
                save_tasks(&task_list)?;
                println!("Task {} deleted.", id);
            } else {
                println!("Task {} not found.", id);
            }
        }
        Commands::Complete { id } => {
            if let Some(task) = task_list.tasks.iter_mut().find(|t| t.id == id) {
                task.completed = true;
                save_tasks(&task_list)?;
                println!("Task {} marked as complete.", id);
            } else {
                println!("Task {} not found.", id);
            }
        }
    }
    Ok(())
}

fn load_tasks() -> io::Result<TaskList> {
    let path = Path::new("tasks.json");
    if path.exists() {
        let data = fs::read_to_string(path)?;
        let task_list: TaskList = serde_json::from_str(&data)?;
        Ok(task_list)
    } else {
        Ok(TaskList {
            tasks: Vec::new(),
            next_id: 1,
        })
    }
}

fn save_tasks(task_list: &TaskList) -> io::Result<()> {
    let data = serde_json::to_string_pretty(task_list)?;
    let mut file = File::create("tasks.json")?;
    file.write_all(data.as_bytes())?;
    Ok(())
}
