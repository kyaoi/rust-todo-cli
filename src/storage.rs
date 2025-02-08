use crate::models::Todo;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

const FILE_NAME: &str = "todos.json";

fn get_todo_file() -> PathBuf {
    let proj_dirs = ProjectDirs::from("com", "example", "todo_cli").unwrap();
    let data_dir = proj_dirs.data_local_dir();
    fs::create_dir_all(data_dir).unwrap();
    data_dir.join(FILE_NAME)
}

pub fn load_todos() -> Vec<Todo> {
    let path = get_todo_file();
    if path.exists() {
        let content = fs::read_to_string(path).unwrap();
        serde_json::from_str(&content).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}

pub fn save_todos(todos: &Vec<Todo>) {
    let path = get_todo_file();
    let content = serde_json::to_string_pretty(todos).unwrap();
    fs::write(path, content).unwrap();
}
