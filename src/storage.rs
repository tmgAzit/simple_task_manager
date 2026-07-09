use crate::models::Task;

use::serde;
use::fs;

const FILE_PATH: &str = "task.json";

pub fn load_tasks()-> Vec<Task> {
    // To check and load the file if not create empty vec
    match fs::read_to_string(FILE_PATH) {
        Ok(contents) =>{
             if contents.trim().is_empty() {
                return vec![];
              };
         serde_json::from_str::<Vec<Task>>(&contents).unwrap_or_else(|_| vec![])
        },
        Err(_) => return vec![]; 
       } 
}

pub fn save_tasks(tasks: &[Task]) {
    // To create a new file if doesn't exist
   let content = serde_json::to_string_pretty(tasks).expect("Failed to serialize");

    fs::write(FILE_PATH, content).expect("Failed to write the tasks in the file"); 

}
    
