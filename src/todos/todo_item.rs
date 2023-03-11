pub struct TodoItem {
    pub(crate) title: String,
    done: bool,
    urgent: bool,
}

impl TodoItem {
    pub fn new(title: String, done: bool, urgent: bool) -> TodoItem {
        TodoItem {
            title,
            done,
            urgent,
        }
    }

    pub fn show_todo(self: &Self) {
        println!("Title: {}\nUrgent: {}\nDone: {}", self.title, self.urgent, self.done)
    }
}
