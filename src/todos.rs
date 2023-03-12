mod todo_item;
mod todo_list;
use crate::todos::todo_item::TodoItem;
use crate::todos::todo_item::Status;
use crate::todos::todo_list::TodoList;

pub fn init() -> TodoList {
    TodoList::new()
}

pub fn list_undone_todos(todo_list: &TodoList) {
    TodoList::list_undone_todos(todo_list);
}

pub fn list_done_todos(todo_list: &TodoList) {
    TodoList::list_done_todos(todo_list);
}

pub fn show(todo_list: &mut TodoList, todo_idx: usize) {
   TodoList::show_todo(todo_list, todo_idx) 
}

pub fn add(title: String, urgent: bool, todo_list: &mut TodoList ) {
    TodoList::add_todo(todo_list, title, urgent);
}

pub fn remove(todo_list: &mut TodoList, todo_idx: usize) {
    TodoList::remove_todo(todo_list, todo_idx)
}

pub fn mark_as_done(todo_list: &mut TodoList, todo_idx: usize) {
    if todo_list.todos.len() < todo_idx {
        println!("Check the provided todo id.");
    } else {
        let todo_item = todo_list.todos.get_mut(todo_idx).unwrap();
        TodoItem::change_status(todo_item, Status::DONE)
    } 
}

pub fn contains_todos(todo_list: &TodoList) -> bool {
    TodoList::contains_todos(todo_list)
}
