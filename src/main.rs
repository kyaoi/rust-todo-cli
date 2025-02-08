mod commands;
mod models;
mod storage;

use clap::Parser;
use commands::{add_todo_action, list_todos, select_todo_action};
use models::{Args, Commands};
use promptuity::Error;
use storage::load_todos;

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let mut todos = load_todos();

    // TODO:
    // - すべてのTODOを削除するコマンド
    // - 何もTODOがないときにはそれ用の表示をする
    // - TODOを編集するコマンド
    match args.command {
        Some(Commands::Add) => add_todo_action(&mut todos)?,
        Some(Commands::List) => list_todos(&mut todos),
        Some(Commands::Select) => select_todo_action(&mut todos)?,
        None => println!("Please select a command."),
    }

    Ok(())
}
