// Simple cli todo application

mod todos;
mod utils;
use todos::{
    add, contains_todos, init, list_done_todos, list_undone_todos, mark_as_done, remove, show,
};
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

(1) Add todo
(2) Mark todo as done
(3) List undone todos
(4) Show todo
(5) Remove todo
(6) Exit
"
        );

        let input_str = prompt_for_input("> ").trim_end().to_owned();

        let input_num = input_str.parse::<u8>();

        match input_num {
            Ok(num) => match num {
                6 => {
                    println!("Exiting...");
                    break;
                }
                3 => list_undone_todos(&todo_list),
                4 => {
                    if contains_todos(&todo_list) {
                        println!("Undone tasks:");
                        list_undone_todos(&todo_list);
                        println!("Done tasks:");
                        list_done_todos(&todo_list);
                        println!("");
                        let todo_idx_str = prompt_for_input("What todo to show?");
                        let todo_idx_num = todo_idx_str.trim_end().parse::<usize>();
                        match todo_idx_num {
                            Ok(idx) => {
                                show(&mut todo_list, idx);
                            }
                            Err(_) => println!("Invalid input. Please enter a number."),
                        }
                    } else {
                        println!("No todos saved.");
                    }
                }
                2 => {
                    if contains_todos(&todo_list) {
                        list_undone_todos(&todo_list);
                        let todo_idx_str = prompt_for_input("What task have you completed?");
                        let todo_idx_num = todo_idx_str.trim_end().parse::<usize>();
                        match todo_idx_num {
                            Ok(idx) => {
                                show(&mut todo_list, idx);
                                let is_done = prompt_for_input("Mark as done? (y/n)");
                                if is_done.trim_end().eq_ignore_ascii_case("y") {
                                    mark_as_done(&mut todo_list, idx)
                                }
                            }
                            Err(_) => println!("Invalid input. Please enter a number."),
                        }
                    } else {
                        println!("No todos saved.");
                    }
                }
                1 => {
                    let title = prompt_for_input("What do you want to do?");
                    let urgent = prompt_for_input("Should the item marked as urgent? (y/n)");
                    let is_urgent = urgent.trim_end().eq_ignore_ascii_case("y");
                    add(title.trim_end().to_owned(), is_urgent, &mut todo_list)
                }
                5 => {
                    if contains_todos(&todo_list) {
                        println!("Undone tasks:");
                        list_undone_todos(&todo_list);
                        println!("Done tasks:");
                        list_done_todos(&todo_list);
                        let todo_idx_str = prompt_for_input("What todo to remove?");
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
