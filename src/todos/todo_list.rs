use crate::todos::TodoItem;

pub struct TodoList {
    todos: Vec<TodoItem>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList { todos: vec![] }
    }

    pub fn list_todos(&self) {
        if !self.todos.is_empty() {
            for (i, todo) in self.todos.iter().enumerate() {
                println!("\nid: {}", i);
                TodoItem::show_todo(todo);
            }
        } else {
            println!("No todos saved.");
        }
    }

    pub fn add_todo(&mut self, todo_item: TodoItem) {
        match todo_item.urgent {
            true => {
                self.todos.insert(0, todo_item);
            }
            false => {
                    self.todos.push(todo_item);
                }
        }
        println!("Todo saved.");
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
