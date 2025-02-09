mod commands;
mod models;
mod storage;

use clap::Parser;
use commands::{
    add_todo_action, delete_todo_action, edit_todo_action, list_todos, select_todo_action,
};
use models::{Args, Commands};
use promptuity::Error;
use storage::load_todos;

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let mut todos = load_todos();

    match args.command {
        Some(Commands::Add) => add_todo_action(&mut todos)?,
        Some(Commands::List) => list_todos(&mut todos),
        Some(Commands::Select) => select_todo_action(&mut todos)?,
        Some(Commands::Edit) => edit_todo_action(&mut todos)?,
        Some(Commands::Delete) => delete_todo_action(&mut todos)?,
        None => println!("Please select a command."),
    }

    Ok(())
}
