use std::io::{self, Write};

pub fn prompt_for_input(prompt: String) -> String {
    let _ = io::stdout().flush();
    let mut input = String::new();
    print!("{} ", prompt);
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut input);
    input
}
