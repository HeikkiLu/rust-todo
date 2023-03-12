pub enum Status {
    DONE,
    NOTDONE,
}

pub enum Urgent {
    URGENT,
    NOTURGENT,
}

pub struct TodoItem {
    pub(crate) title: String,
    pub(crate) status: Status,
    pub(crate) urgent: Urgent,
}

impl TodoItem {
    pub fn new(title: String, urgent: Urgent) -> TodoItem {
        TodoItem {
            title,
            status: Status::NOTDONE,
            urgent,
        }
    }

    pub fn change_status(&mut self, new_status: Status) {
        self.status = new_status;
    }

    pub fn show_todo(self: &Self) {
        match self.urgent {
            Urgent::URGENT => println!("{}\nUrgent: [x]", self.title),
            Urgent::NOTURGENT => println!("{}\nUrgent: [ ]", self.title),
        }
    }
}
