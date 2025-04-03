use std::{collections::HashMap, env::{self}};

use todo_manager::{fetch_data, Commands, Task};



fn main() {
    let mut list: HashMap<usize, Task> = fetch_data();
    // let mut list: HashMap<usize, Task> = HashMap::new();
    let mut args = env::args();

    if args.len() < 2 {
        panic!("Error: arguments too small")
    }

    args.next();
    let input_command = args.next();
    let value = args.next();

   let command = match input_command {

    Some(cmd) => {
        match cmd.as_str() {
            "add" => {
                let val = value.expect("Error: reading title");
                Commands::Add(val)
            },
            "list" => Commands::List,
            "done" => {
                let val: usize = value.expect("Error: reading index").parse().expect("Error: invalid index");
                Commands::Done(val)
            },
            "remove" => {
                let val: usize = value.expect("Error: reading index").parse().expect("Error: invalid index");
                Commands::Done(val)
            },
            "empty" => Commands::Empty,
            _ => panic!("Error: Invalid Command")
        }
    },
    None => panic!("Error: arguments too small")
   };

   command.run(&mut list);
    

}