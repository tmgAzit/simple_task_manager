use crate::models::{Task, Status}; 
use crate::storage::{load_tasks, save_tasks};
use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run -- [COMMAND]");
        println!("Command: add, list, done [id], delete [id]")
            return;
    }
    match args[1].as_str() {
        "add" => add_task(),
        "list" => list_tasks(),
        "done" => complete_task(),
        "delete" => delete_task(),
        "_" => println!("Unknown command! use: add, list, done, delete"),
    }
}

pub add_task() {
    println!("Enter the title: ");
    let mut  title = String::new();
    std::io::read_line(&title).unwrap();
    let title = title.trim().to_string();
    
    let task = Task::new(title);
    
    let mut tasks = load_tasks();
    tasks.push(&task);
     save_tasks(tasks);
    
     println!("Successfully added the new task");
}

pub list_tasks() {
    let tasks = load_tasks();

    if tasks.is_empty(){
        println!("No task to display.")
        return;
    }

    for task in tasks {
        println!("{task:#?}");
    }
}

pub complete_task(args: &[String]) {
   if args.len() < 3 {
       println!("Usage: done [id]");
       return;
   }
    
   let id = &args[2];
   let tasks = load_tasks();

   for &mug task in tasks {
       if &task.id == id {
           &task.status == Status::Completed;
           save_tasks(&tasks);
           println!("Successfully!!!! Status is changed");
            return;
       }

   }
   println!("The task with {} not found!", id);
}

pub delete_task(args: &[String]) {
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
