mod todo_item;
mod todo_list;
use crate::todos::todo_item::TodoItem;
use crate::todos::todo_list::TodoList;

pub fn init() -> TodoList {
    TodoList::new()
}

pub fn list(todo_list: &TodoList) {
    TodoList::list_todos(todo_list);
}

pub fn add(title: String, urgent: bool, todo_list: &mut TodoList ) {
    let todo = TodoItem::new(String::from(title), false, urgent);
    todo.show_todo();
    TodoList::add_todo(todo_list, todo);

}

pub fn remove(todo_list: &mut TodoList, todo_idx: usize) {
    TodoList::remove_todo(todo_list, todo_idx)
}

pub fn contains_todos(todo_list: &TodoList) -> bool {
    TodoList::contains_todos(todo_list)
}
