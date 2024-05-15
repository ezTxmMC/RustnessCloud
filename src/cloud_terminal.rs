use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{self, Result, Write};

pub struct CloudTerminal {
    pub name: String,
}

impl CloudTerminal {
    pub fn new(name: &str) -> CloudTerminal {
        CloudTerminal {
            name: name.to_string(),
        }
    }

    pub fn readline(&self) -> String {
        print!("{}> ", self.name);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().to_string()
    }

    pub fn writeline(&self, message: &str) {
        println!("{}", message);
    }

    pub fn clear(&self) -> Result<()> {
        execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
        Ok(())
    }

    pub fn change_to_another_terminal(&self, other: &CloudTerminal) {
        println!("Switching to terminal: {}", other.name);
    }

    pub fn get_current_terminal(&self) -> &CloudTerminal {
        self
    }
}