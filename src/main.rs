// Simple cli todo application
//
// This app takes function arguments list, show, add and delete. If the argument is item specific
// such as delete or show, then a third argument is needed, which is the number of the todo item.

use std::io;
mod todos;
mod utils;
use todos::{add, contains_todos, init, list, remove};
use utils::prompt_for_input;

fn main() {
    let art = r"
                   ___          _____          ___     
       ___        /  /\        /  /::\        /  /\    
      /  /\      /  /::\      /  /:/\:\      /  /::\   
     /  /:/     /  /:/\:\    /  /:/  \:\    /  /:/\:\  
    /  /:/     /  /:/  \:\  /__/:/ \__\:|  /  /:/  \:\ 
   /  /::\    /__/:/ \__\:\ \  \:\ /  /:/ /__/:/ \__\:\
  /__/:/\:\   \  \:\ /  /:/  \  \:\  /:/  \  \:\ /  /:/
  \__\/  \:\   \  \:\  /:/    \  \:\/:/    \  \:\  /:/ 
       \  \:\   \  \:\/:/      \  \::/      \  \:\/:/  
        \__\/    \  \::/        \__\/        \  \::/   
                  \__\/                       \__\/    
";
    println!("{}", art);
    let mut todo_list = init();
    loop {
        println!(
            "

(1) List todos
(2) Add todo
(3) Remove todo
(4) Exit
"
        );

        let mut raw_input = String::new();
        let _ = io::stdin().read_line(&mut raw_input);

        let input: u8 = raw_input.trim_end().parse().expect("Input not a number!");

        if input == 4 {
            println!("Exiting...");
            break;
        }
        if input == 1 {
            list(&todo_list)
        }
        if input == 2 {
            let title = prompt_for_input(String::from("What should the title be?"));
            let urgent = prompt_for_input(String::from("Should the item marked as urgent? (y/n)"));
            if urgent.trim_end().eq_ignore_ascii_case("y") {
                add(title.trim_end().to_owned(), true, &mut todo_list)
            } else {
                add(title.trim_end().to_owned(), false, &mut todo_list)
            }
        }
        if input == 3 {
            if contains_todos(&todo_list) {
                list(&todo_list);
                let todo_idx = prompt_for_input(String::from("What todo to remove?"));
                remove(&mut todo_list, todo_idx.trim_end().parse().unwrap())
            } else {
                println!("No todos saved.");
            }
        }
    }
}
