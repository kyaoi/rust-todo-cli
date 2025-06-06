use crate::models::Todo;
use crate::storage::save_todos;
use promptuity::prompts::{Confirm, Input, Select, SelectOption};
use promptuity::themes::FancyTheme;
use promptuity::{Error, Promptuity, Term};
use uuid::Uuid;

pub fn add_todo_action(todos: &mut Vec<Todo>) -> Result<(), Error> {
    let mut term = Term::default();
    let mut theme = FancyTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    p.term().clear()?;
    p.with_intro("Input a new task.").begin()?;
    let title = p.prompt(Input::new("📌 Enter task title").with_placeholder("Title"))?;
    let description =
        p.prompt(Input::new("📝 Enter task description").with_placeholder("Description"))?;

    let todo = Todo {
        id: Uuid::new_v4().to_string(),
        title,
        description,
        done: false,
    };

    todos.push(todo);
    save_todos(todos);
    p.with_outro("✅ Added new task successfully!").finish()?;

    Ok(())
}

pub fn list_todos(todos: &mut [Todo]) {
    if todos.is_empty() {
        println!("No tasks found.");
        return;
    }
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

pub fn select_todo_action(todos: &mut Vec<Todo>) -> Result<(), Error> {
    if todos.is_empty() {
        println!("No tasks found.");
        return Ok(());
    }
    let mut term = Term::default();
    let mut theme = FancyTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    p.term().clear()?;
    p.with_intro("Select a task to modify.").begin()?;

    let select_options: Vec<SelectOption<String>> = todos
        .iter()
        .map(|todo| {
            SelectOption::new(
                format!("{} - {}", if todo.done { "✅" } else { "❌" }, todo.title),
                todo.id.to_string(),
            )
        })
        .collect();
    let id: String = p.prompt(
        Select::new("Which task do you want to modify?", select_options.clone()).as_mut(),
    )?;

    let action: &str = p.prompt(
        Select::new(
            "What do you want to do?",
            vec![
                SelectOption::new("Mark as done", "done"),
                SelectOption::new("Mark as not done", "not_done"),
                SelectOption::new("Delete", "delete"),
            ],
        )
        .as_mut(),
    )?;

    if let Some(index) = todos.iter().position(|t| t.id == id) {
        match action {
            "done" => {
                todos[index].done = true;
                save_todos(todos);
                p.with_outro(format!(
                    "Task {} has been marked as done.",
                    todos[index].title
                ))
                .finish()?;
            }
            "not_done" => {
                todos[index].done = false;
                save_todos(todos);
                p.with_outro(format!(
                    "Task {} has been marked as not done.",
                    todos[index].title
                ))
                .finish()?;
            }
            "delete" => {
                let removed_task = todos.remove(index);
                save_todos(todos);
                p.with_outro(format!("Task {} has been deleted.", removed_task.title))
                    .finish()?;
            }
            _ => {
                p.with_outro("Invalid action.").finish()?;
            }
        }
    }

    Ok(())
}

pub fn edit_todo_action(todos: &mut Vec<Todo>) -> Result<(), Error> {
    if todos.is_empty() {
        let mut term = Term::default();
        let mut theme = FancyTheme::default();
        let mut p = Promptuity::new(&mut term, &mut theme);

        p.term().clear()?;
        p.with_intro("No tasks found.").begin()?;
        let confirm = p.prompt(Confirm::new("Do you want to add a new task?").with_default(true));

        if confirm? {
            let title = p.prompt(Input::new("📌 Enter task title").with_placeholder("Title"))?;
            let description =
                p.prompt(Input::new("📝 Enter task description").with_placeholder("Description"))?;

            let todo = Todo {
                id: Uuid::new_v4().to_string(),
                title,
                description,
                done: false,
            };

            todos.push(todo);
            save_todos(todos);
            p.with_outro("✅ Added new task successfully!").finish()?;
            return Ok(());
        }

        p.with_outro("Don't add a new task.").finish()?;
        return Ok(());
    }

    let mut term = Term::default();
    let mut theme = FancyTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    p.term().clear()?;
    p.with_intro("Edit a task").begin()?;

    let select_options: Vec<SelectOption<String>> = todos
        .iter()
        .map(|todo| {
            SelectOption::new(
                format!("{} - {}", if todo.done { "✅" } else { "❌" }, todo.title),
                todo.id.to_string(),
            )
        })
        .collect();
    let id: String =
        p.prompt(Select::new("Which task do you want to eidt?", select_options.clone()).as_mut())?;

    if let Some(index) = todos.iter().position(|t| t.id == id) {
        let todo = &mut todos[index];
        let title = p.prompt(
            Input::new("📌 Edit task title")
                .with_placeholder("Title")
                .with_default(todo.title.to_string()),
        )?;
        let description = p.prompt(
            Input::new("📝 Edit task description")
                .with_placeholder("Description")
                .with_default(todo.description.to_string()),
        )?;

        todos[index].title = title;
        todos[index].description = description;
        save_todos(todos);
        p.with_outro("✅ Edited task successfully!").finish()?;
        return Ok(());
    }

    p.with_outro("❌ Task not found.").finish()?;
    Ok(())
}

pub fn delete_todo_action(todos: &mut Vec<Todo>) -> Result<(), Error> {
    let mut term = Term::default();
    let mut theme = FancyTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    p.term().clear()?;
    p.with_intro("Delete all tasks.").begin()?;
    let confirm = p.prompt(Confirm::new("Do you want to delete all tasks?").with_default(true));

    if confirm? {
        todos.clear();
        save_todos(todos);
        p.with_outro("✅ Deleted all tasks successfully!")
            .finish()?;
        return Ok(());
    }

    p.with_outro("❌ Not deleted.").finish()?;
    Ok(())
}
