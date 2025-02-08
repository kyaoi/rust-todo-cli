use clap::Parser;
use directories::ProjectDirs;
use promptuity::prompts::{Select, SelectOption};
use promptuity::themes::FancyTheme;
use promptuity::{Error, Promptuity, Term};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    title: String,
    description: String,
    done: bool,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    add: bool,

    #[arg(short, long)]
    list: bool,

    #[arg(short, long)]
    select: bool,
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

fn prompt_input(prompt: &str) -> String {
    print!("{}: ", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn select_todo_action(todos: &mut Vec<Todo>) -> Result<(), Error> {
    let mut term = Term::default();
    let mut theme = FancyTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    p.term().clear()?;
    p.with_intro("Select a task to modify.").begin()?;

    let select_options: Vec<SelectOption<usize>> = todos
        .iter()
        .enumerate()
        .map(|(i, todo)| {
            SelectOption::new(
                format!("{} - {}", if todo.done { "✅" } else { "❌" }, todo.title),
                i,
            )
        })
        .collect();

    if let Some(selected_task) =
        p.prompt(Select::new("Which task do you want to modify?", select_options).as_mut())?
    {
        let task_value = selected_task.value; // SelectOption<usize> の value を取得

        p.with_outro(format!("SELECTED: {}", task_value)).finish()?;
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let mut todos = load_todos();

    if args.add {
        let title = prompt_input("📌 Enter task title");
        let description = prompt_input("📝 Enter task description");

        let todo = Todo {
            title,
            description,
            done: false,
        };

        todos.push(todo);
        save_todos(&todos);
        println!("✅ Added new task successfully!");
    }

    if args.list {
        println!("\n📝 TODO List:");
        for (i, todo) in todos.iter().enumerate() {
            let status = if todo.done {
                "✅ Done"
            } else {
                "❌ Not Done"
            };
            println!("{} - [{}] {}: {}", i, status, todo.title, todo.description);
        }
    }

    if args.select {
        select_todo_action(&mut todos)?;
    }

    Ok(())
}
