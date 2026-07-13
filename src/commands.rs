use crate::models::{Status, Task};
use crate::storage::{load_tasks, save_tasks};
use std::env;
use std::io::{self, Write};

pub fn run() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run -- [COMMAND]");
        println!("Command: add, list, done [id], delete [id]");
        return;
    }
    match args[1].as_str() {
        "add" => add_task(),
        "list" => list_tasks(),
        "done" => complete_task(&args),
        "delete" => delete_task(&args),
        _ => println!("Unknown command! use: add, list, done, delete"),
    }
}

pub fn add_task() {
    println!("Enter the title: ");
    io::stdout().flush().unwrap();

    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();
    let title = title.trim().to_string();

    let task = Task::new(title);

    let mut tasks = load_tasks();
    tasks.push(task);
    save_tasks(&mut tasks);

    println!("Successfully added the new task");
}

pub fn list_tasks() {
    let tasks = load_tasks();

    if tasks.is_empty() {
        println!("No task to display.");
        return;
    }

    for task in tasks {
        println!("{task:#?}");
    }
}

pub fn complete_task(args: &[String]) {
    if args.len() < 3 {
        println!("Usage: done [id]");
        return;
    }

    let id = &args[2];
    let mut tasks = load_tasks();

    for task in &mut tasks {
        if &task.id == id {
            task.status = Status::Completed;
            save_tasks(&tasks);
            println!("Successfully!!!! Status is changed");
            return;
        }
    }
}

pub fn delete_task(args: &[String]) {
    if args.len() < 3 {
        println!("Usage: delete [id]");
        return;
    };

    let id = &args[2];
    let mut tasks = load_tasks();

    let initial_len = tasks.len();
    tasks.retain(|task| &task.id != id);

    if tasks.len() < initial_len {
        save_tasks(&tasks);
        println!("Sucessfully new task with the title is added");
    } else {
        println!("The task with {} is not found!", id);
    }
}
