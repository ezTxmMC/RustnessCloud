use super::terminal::Terminal;
use crate::config::json_config::JsonConfig;

pub struct TerminalManager {
    pub(crate) launch_config: JsonConfig,
    pub(crate) main_terminal: Terminal,
    pub(crate) setup_terminal: Terminal,
    pub(crate) group_setup_terminal: Terminal,
}

impl TerminalManager {
    
    pub fn new(launch_config: JsonConfig) -> Self {
        let launch_config = launch_config;
        Self {
            launch_config,
            main_terminal: Terminal::new("main"),
            setup_terminal: Terminal::new("setup"),
            group_setup_terminal: Terminal::new("group-setup")
        }
    }

    pub fn start_terminal(&mut self) {
        let mut current_terminal = self.main_terminal.get_current_terminal();

        let need_setup = !self.launch_config.get("Host").is_some();
        let mut setup_step = 0;

        current_terminal.clear();

        if need_setup {
            current_terminal = self.setup_terminal.get_current_terminal();
            current_terminal.write_line("On which ip address should run the cloud?");
        }

        loop {
            let input = current_terminal.readline();
            let current_terminal_name = current_terminal.name.as_str();

            match current_terminal_name {
                "main" => {
                    if input == "clear" || input == "cls" {
                        current_terminal.clear();
                        continue;
                    }
                    if input == "shutdown" || input == "exit" || input == "quit" || input == "stop" {
                        break;
                    }
                }
                "setup" => match setup_step {
                    0 => {
                        self.launch_config.set("Host", input.clone());
                        current_terminal.write_line("Which port should the cloud run on?");
                        setup_step += 1;
                        continue;
                    }
                    1 => {
                        self.launch_config.set("Port", input.clone());
                        current_terminal
                            .write_line("How many memory should use the cloud? (in GB)");
                        setup_step += 1;
                        continue;
                    }
                    2 => {
                        self.launch_config.set("Memory", input.clone());
                        current_terminal.clear();
                        current_terminal = self.main_terminal.get_current_terminal();
                        setup_step += 1;
                        continue;
                    }
                    _ => {
                        current_terminal
                            .write_line("Setup has been cancelled caused an unknown error.");
                        break;
                    }
                },
                "group-setup" => {}
                _ => {
                    if current_terminal_name.starts_with("service-") {
                        if input == "leave" {
                            current_terminal = self.main_terminal.get_current_terminal();
                            continue;
                        }
                        // TODO: Send Command into java application in terminal
                        continue;
                    }
                    current_terminal.write_line("Unknown terminal.");
                }
            }
            current_terminal.write_line(&format!("Unknown command: {}", input));
        }
    }
}
