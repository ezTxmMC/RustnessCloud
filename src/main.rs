use std::{io::Result, ptr::null};

mod cloud_terminal;
use cloud_terminal::CloudTerminal;
use json_config::JsonConfig;
mod json_config;

fn main() -> Result<()> {
    let mut config = JsonConfig::new("data", "launch");

    let main_terminal = CloudTerminal::new("main");
    let setup_terminal = CloudTerminal::new("setup");
    let group_setup_terminal = CloudTerminal::new("group-setup");

    let mut current_terminal = main_terminal.get_current_terminal();

    let need_setup = !config.get("Host").is_some();
    let mut setup_step = 0;

    if need_setup {
        current_terminal = setup_terminal.get_current_terminal();
        current_terminal.writeline("On which ip addresse should run the cloud?");
    }

    current_terminal.clear();

    loop {
        let input = current_terminal.readline();
        let current_terminal_name = current_terminal.name.as_str();

        match current_terminal_name {
            "main" => {
                if input == "clear" || input == "cls" {
                    current_terminal.clear()?;
                    continue;
                }
                if input == "shutdown" {
                    return Ok(());
                }
            }
            "setup" => {
                match setup_step {
                    0 => {
                        config.set("Host", input.clone());
                        current_terminal.writeline("On which port should run the cloud?");
                        setup_step += 1;
                        continue;
                    }
                    1 => {
                        config.set("Port", input.clone());
                        current_terminal.writeline("How many memory should use the cloud? (in GB)");
                        setup_step += 1;
                        continue;
                    }
                    2 => {
                        config.set("Memory", input.clone());
                        current_terminal.clear();
                        current_terminal = main_terminal.get_current_terminal();
                        setup_step += 1;
                        continue;
                    }
                    _ => {
                        current_terminal
                            .writeline("Setup has been cancelled caused an unknown error.");
                        return Ok(());
                    }
                }
            }
            "group-setup" => {}
            _ => {
                if current_terminal_name.starts_with("service-") {
                    if input == "leave" {
                        current_terminal = main_terminal.get_current_terminal();
                        continue;
                    }
                    // TODO: Send Command into java application in terminal
                    continue;
                }
                current_terminal.writeline("Unknown terminal.");
            }
        }
        current_terminal.writeline(&format!("Unknown command: {}", input));
    }
}
