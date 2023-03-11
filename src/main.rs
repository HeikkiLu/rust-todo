// Simple cli todo application

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

        let input_str = prompt_for_input(String::from("> ")).trim_end().to_owned();

        let input_num = input_str.parse::<u8>();

        match input_num {
            Ok(num) => match num {
                4 => {
                    println!("Exiting...");
                    break;
                }
                1 => list(&todo_list),
                2 => {
                    let title = prompt_for_input(String::from("What do you want to do?"));
                    let urgent =
                        prompt_for_input(String::from("Should the item marked as urgent? (y/n)"));
                    let is_urgent = urgent.trim_end().eq_ignore_ascii_case("y");
                    add(title.trim_end().to_owned(), is_urgent, &mut todo_list)
                }
                3 => {
                    if contains_todos(&todo_list) {
                        list(&todo_list);
                        let todo_idx_str = prompt_for_input(String::from("What todo to remove?"));
                        let todo_idx_num = todo_idx_str.trim_end().parse::<usize>();
                        match todo_idx_num {
                            Ok(idx) => remove(&mut todo_list, idx),
                            Err(_) => println!("Invalid input. Please enter a number."),
                        }
                    } else {
                        println!("No todos saved.");
                    }
                }
                _ => println!("Invalid input. Please try again."),
            },
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    }
}
