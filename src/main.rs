use clap::{Arg, Command};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    task: String,
    done: bool,
}

const FILE_NAME: &str = "todos.json";

fn get_todo_file() -> PathBuf {
    let proj_dirs = ProjectDirs::from("com", "example", "todo_cli").unwrap();
    let data_dir = proj_dirs.data_local_dir();
    fs::create_dir_all(data_dir).unwrap();
    data_dir.join(FILE_NAME)
}

fn load_todos() -> Vec<Todo> {
    let path = get_todo_file();
    if path.exists() {
        let content = fs::read_to_string(path).unwrap();
        serde_json::from_str(&content).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}

fn save_todos(todos: &Vec<Todo>) {
    let path = get_todo_file();
    let content = serde_json::to_string_pretty(todos).unwrap();
    fs::write(path, content).unwrap();
}

fn main() {
    let matches = Command::new("todo")
        .version("1.0")
        .about("CLI TODO App in Rust")
        .subcommand(Command::new("add").arg(Arg::new("task").required(true)))
        .subcommand(Command::new("list"))
        .subcommand(Command::new("done").arg(Arg::new("index").required(true)))
        .subcommand(Command::new("remove").arg(Arg::new("index").required(true)))
        .get_matches();

    let mut todos = load_todos();

    if let Some(matches) = matches.subcommand_matches("add") {
        let task = matches.get_one::<String>("task").unwrap().to_string();
        todos.push(Todo { task, done: false });
        save_todos(&todos);
        println!("Added new task!");
    }

    if matches.subcommand_matches("list").is_some() {
        for (i, todo) in todos.iter().enumerate() {
            let status = if todo.done { "[Done]" } else { "[ ]" };
            println!("{} {} - {}", i, status, todo.task);
        }
    }

    if let Some(matches) = matches.subcommand_matches("done") {
        let index: usize = matches.get_one::<String>("index").unwrap().parse().unwrap();
        if index < todos.len() {
            todos[index].done = true;
            save_todos(&todos);
            println!("Marked as done!");
        }
    }

    if let Some(matches) = matches.subcommand_matches("remove") {
        let index: usize = matches.get_one::<String>("index").unwrap().parse().unwrap();
        if index < todos.len() {
            todos.remove(index);
            save_todos(&todos);
            println!("Removed task!");
        }
    }
}
