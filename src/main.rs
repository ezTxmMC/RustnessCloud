use std::io::Result;

mod cloud_terminal;
use cloud_terminal::CloudTerminal;
mod json_config;

fn main() -> Result<()> {
    let main_terminal = CloudTerminal::new("main");
    let setup_terminal = CloudTerminal::new("setup");

    main_terminal.writeline("Welcome to the main terminal.");
    main_terminal.writeline("Type 'setup' to switch to the setup terminal, 'clear' to clear the screen, or 'exit' to quit.");

    let mut current_terminal = main_terminal.get_current_terminal();

    loop {
        let input = current_terminal.readline();
        let current_terminal_name = current_terminal.name.as_str();

        match current_terminal_name {
            "main" => {
                if input == "setup" {
                    current_terminal.change_to_another_terminal(&setup_terminal);
                    current_terminal = setup_terminal.get_current_terminal();
                    setup_terminal.writeline("Welcome to the setup terminal.");
                    setup_terminal.writeline("Type 'main' to switch back to the main terminal, 'clear' to clear the screen, or 'exit' to quit.");
                    continue;
                }
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
