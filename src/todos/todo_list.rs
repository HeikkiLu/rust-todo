use crate::todos::todo_item::Urgent;
use crate::todos::todo_item::Status;
use crate::todos::TodoItem;

pub struct TodoList {
    pub(crate) todos: Vec<TodoItem>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList { todos: vec![] }
    }

    pub fn list_undone_todos(&self) {
        if !self.todos.is_empty() {
            for (i, todo) in self.todos.iter().enumerate() {
                match todo.status {
                    Status::NOTDONE => println!("{}: {}", i, todo.title),
                    Status::DONE => print!("")
                }
            }
        } else {
            println!("No todos saved.");
        }
    }

    pub fn list_done_todos(&self) {
        if !self.todos.is_empty() {
            for (i, todo) in self.todos.iter().enumerate() {
                match todo.status {
                    Status::NOTDONE => print!(""),
                    Status::DONE => println!("{}: {}", i, todo.title)
                }
            }
        } else {
            println!("No todos saved.");
        }
    }

    pub fn show_todo(&self, todo_idx: usize) {
        if self.todos.len() <= todo_idx {
            println!("Todo index out of bounds.");
            return;
        }
        if !self.todos.is_empty() {
            TodoItem::show_todo(&self.todos[todo_idx]);
        } else {
            println!("No todos saved.");
        }
    }

    pub fn add_todo(&mut self, title: String, urgent: bool) {
        match urgent {
            true => {
                let todo = TodoItem::new(title, Urgent::URGENT);
                self.todos.insert(0, todo);
                println!("Todo saved.");
            }
            false => {
                let todo = TodoItem::new(title, Urgent::NOTURGENT);
                self.todos.push(todo);
                println!("Todo saved.");
            }
        }
    }

    pub fn remove_todo(&mut self, todo_idx: usize) {
        if self.todos.len() > todo_idx {
            self.todos.remove(todo_idx);
        } else {
            println!("Todo not found.")
        }
    }

    pub fn contains_todos(&self) -> bool {
        !self.todos.is_empty()
    }
}
