pub struct TodoItem {
    pub(crate) title: String,
    done: bool,
    pub(crate) urgent: bool,
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
        println!("\nTitle: {}\nUrgent: {}\nDone: {}\n", self.title, self.urgent, self.done)
    }
}
