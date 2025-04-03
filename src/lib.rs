use std::{collections::HashMap, fs::File, io::{ErrorKind, Read, Write}};
use colored::Colorize;
use serde::{Serialize, Deserialize};
use serde_json;


#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub is_completed: bool
}

impl Task {
    pub fn new(title: String)-> Task {
        Task { title, is_completed: false }
    }

    pub fn mark_as_done(&mut self) {
        self.is_completed = true;
    }
}

pub enum Commands {
    Add(String),
    List,
    Done(usize),
    Remove(usize),
    Empty
}

impl Commands {
    pub fn run(&self, list: &mut HashMap<usize, Task>) {
        match self {
            Commands::Add(str) => {
                    add_task(list, &str);
            },
            Commands::Done(index) => {
                mark_task(list, *index);
            },
            Commands::List => {
                list_tasks(list);
            },
            Commands::Remove(index) => {
                remove_task(list, *index);
            },
            Commands::Empty => {
                empty_list(list);
            }
        };
    }
}

pub fn add_task(list: &mut HashMap<usize, Task>, title: &str) {
    let task = Task::new(title.to_owned());
    list.insert(list.len() + 1, task);
    save_data(list);
}

pub fn list_tasks(list: &HashMap<usize, Task>) {
    for (index, todo) in list.iter() {
        match todo.is_completed {
            true => {
                println!("[{}] {}", index.to_string().green(), todo.title.as_str().green())
            },
            false => {
                println!("[{}] {}", index.to_string().yellow(), todo.title.as_str().yellow())
            }
        }
    }
}

pub fn mark_task(list: &mut HashMap<usize, Task>, index: usize) {
    if let Some(todo) = list.get_mut(&index) {
        todo.mark_as_done();
    }
    save_data(list);
}

pub fn remove_task(list: &mut HashMap<usize, Task>, index: usize) {
    list.remove(&index);
    save_data(list);
}

pub fn empty_list(list: &mut HashMap<usize, Task>) {
    list.clear();
}


pub fn fetch_data() -> HashMap<usize, Task> {
    let file = File::open("data.json");

    let mut file = match file {
        Ok(f) => f,
        Err(ref e) => if e.kind() == ErrorKind::NotFound {
            let f = File::create("data.json");
            f.expect("Error Creating file")
        } else {
            panic!("Error reading file")
        }
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => {
            panic!("Something went wrong: {}", e)
        }
    };

    let deserialized: HashMap<usize, Task>  = serde_json::from_str(&contents).expect("Error reading file contents");

    return deserialized
}


pub fn save_data(list: &mut HashMap<usize, Task>) {
    let serialized_data = serde_json::to_string(list).expect("Error saving data");

    let mut file = File::open("data.json").expect("error: saving file");

    file.write(serialized_data.as_bytes()).expect("error writing to file");
    
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_task() {
        let mut list: HashMap<usize, Task> = HashMap::new();
        let title = String::from("Do The Dishes");
        add_task(&mut list, &title);

        let todo = list.get(&1).unwrap();

            assert_eq!(todo.title, title);
            assert!(!todo.is_completed);
    }
}
