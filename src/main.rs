use std::io::Result;

mod cloud_terminal;
use cloud_terminal::CloudTerminal;
mod json_config;

fn main() -> Result<()> {
    let main_terminal = CloudTerminal::new("main");
    let setup_terminal = CloudTerminal::new("setup");

    let mut current_terminal = main_terminal.get_current_terminal();

    let need_setup = true;

    if need_setup {
        current_terminal = setup_terminal.get_current_terminal();
        setup_terminal.writeline("Welcome to the setup terminal.");
    }

    loop {
        let input = current_terminal.readline();
        let current_terminal_name = current_terminal.name.as_str();

        

        match current_terminal_name {
            "main" => {
                if input == "clear" {
                    current_terminal.clear()?;
                    continue;
                }
                if input == "shutdown" {
                    return Ok(());
                }
            }
            "setup" => {
                
            }
            _ => {
                println!("Unknown terminal.");
                return Ok(());
            }
        }
        current_terminal.writeline(&format!("Unknown command: {}", input));
    }
}
