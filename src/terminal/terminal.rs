use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};

pub struct Terminal {
    pub name: String,
}

impl Terminal {
    pub fn new(name: &str) -> Terminal {
        Terminal {
            name: name.to_string(),
        }
    }

    pub fn readline(&self) -> String {
        print!("RustnessCloud » ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().to_string()
    }

    pub fn write_line(&self, message: &str) {
        println!("{}", message);
    }

    pub fn clear(&self) {
        execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0));
    }

    pub fn get_current_terminal(&self) -> &Terminal {
        self
    }
}