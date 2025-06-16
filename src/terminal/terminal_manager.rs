use std::net::ToSocketAddrs;
use std::ptr::null;
use super::terminal::Terminal;
use crate::config::json_config::JsonConfig;
use crate::group::group;
use serde_json::Number;
use crate::downloader::software_downloader;

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
            group_setup_terminal: Terminal::new("group-setup"),
        }
    }

    pub async fn start_terminal(&mut self) {
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
            let command = input.split(" ").next().unwrap().to_string();
            let args = input.split(" ").skip(1).collect::<Vec<&str>>();
            let current_terminal_name = current_terminal.name.as_str();

            match current_terminal_name {
                "main" => {
                    match command.as_str() {
                        "clear" | "cls" | "clr" => {
                            current_terminal.clear();
                            continue;
                        }
                        "create" => {
                            // create group lobby
                            if args.len() < 2 {
                                current_terminal.write_line(
                                    "\
                                create group proxy\n\
                                create group lobby\n\
                                create group server",
                                );
                                continue;
                            }
                            match args[0] {
                                "group" => match args[1] {
                                    "proxy" => {
                                        if args.len() < 13 {
                                            current_terminal.write_line("create group proxy <name> <minMem> <maxMem> <maxPlayers> <static> <software> <version> <priority> <port> <maintenance> <permission>");
                                            continue;
                                        }
                                        let permission = if args[12].to_lowercase() == "none" {
                                            None
                                        } else {
                                            Some(args[12].to_lowercase())
                                        };
                                        group::create_proxy_group(
                                            args[2],
                                            args[3].parse::<Number>().unwrap(),
                                            args[4].parse::<Number>().unwrap(),
                                            args[5].parse::<Number>().unwrap(),
                                            if args[6].to_lowercase() == "yes" {
                                                true
                                            } else {
                                                false
                                            },
                                            args[7].to_string(),
                                            args[8].to_string(),
                                            args[9].parse::<Number>().unwrap(),
                                            args[10].parse::<Number>().unwrap(),
                                            if args[11].to_lowercase() == "yes" {
                                                true
                                            } else {
                                                false
                                            },
                                            permission.unwrap(),
                                        );
                                        let _ = software_downloader::download("PROXY".to_string(), args[7].to_uppercase(), args[8].to_string()).await;
                                        current_terminal.write_line(
                                            format!("Created group {}.", args[2]).as_str(),
                                        );
                                        continue;
                                    }
                                    "lobby" => {
                                        if args.len() < 16 {
                                            current_terminal.write_line("create group lobby <name> <minMem> <maxMem> <maxPlayers> <static> <software> <version> <priority> <port> <permission> <java> <newServiceProcent> <minOnlineCount> <maxOnlineCount>");
                                            continue;
                                        }
                                        let permission = if args[12].to_lowercase() == "none" {
                                            None
                                        } else {
                                            Some(args[12].to_lowercase())
                                        };
                                        group::create_lobby_group(
                                            args[2],
                                            args[3].parse::<Number>().unwrap(),
                                            args[4].parse::<Number>().unwrap(),
                                            args[5].parse::<Number>().unwrap(),
                                            if args[6].to_lowercase() == "yes" {
                                                true
                                            } else {
                                                false
                                            },
                                            args[7].to_string(),
                                            args[8].to_string(),
                                            args[9].parse::<Number>().unwrap(),
                                            args[10].parse::<Number>().unwrap(),
                                            args[11].to_string(),
                                            permission.unwrap(),
                                            args[13].parse::<Number>().unwrap(),
                                            args[14].parse::<Number>().unwrap(),
                                            args[15].parse::<Number>().unwrap(),
                                        );
                                        let _ = software_downloader::download("SERVER".to_string(), args[7].to_uppercase(), args[8].to_string()).await;
                                        current_terminal.write_line(
                                            format!("Created group {}.", args[2]).as_str(),
                                        );
                                        continue;
                                    }
                                    "server" => {
                                        if args.len() < 16 {
                                            current_terminal.write_line("create group server <name> <minMem> <maxMem> <maxPlayers> <static> <software> <version> <priority> <port> <permission> <java> <newServiceProcent> <minOnlineCount> <maxOnlineCount>");
                                            continue;
                                        }
                                        let permission = if args[12].to_lowercase() == "none" {
                                            None
                                        } else {
                                            Some(args[12].to_lowercase())
                                        };
                                        group::create_server_group(
                                            args[2],
                                            args[3].parse::<Number>().unwrap(),
                                            args[4].parse::<Number>().unwrap(),
                                            args[5].parse::<Number>().unwrap(),
                                            if args[6].to_lowercase() == "yes" {
                                                true
                                            } else {
                                                false
                                            },
                                            args[7].to_string(),
                                            args[8].to_string(),
                                            args[9].parse::<Number>().unwrap(),
                                            args[10].parse::<Number>().unwrap(),
                                            args[11].to_string(),
                                            permission.unwrap(),
                                            args[13].parse::<Number>().unwrap(),
                                            args[14].parse::<Number>().unwrap(),
                                            args[15].parse::<Number>().unwrap(),
                                        );
                                        let _ = software_downloader::download("SERVER".to_string(), args[7].to_uppercase(), args[8].to_string()).await;
                                        current_terminal.write_line(
                                            format!("Created group {}.", args[2]).as_str(),
                                        );
                                        continue;
                                    }
                                    _ => {
                                        current_terminal.write_line(
                                            format!("Group {} type doesn't exist.", args[0])
                                                .as_str(),
                                        );
                                    }
                                },
                                _ => {
                                    current_terminal
                                        .write_line(format!("{} doesn't exist.", args[0]).as_str());
                                    continue;
                                }
                            }
                        }
                        "delete" => {}
                        "shutdown" | "exit" | "quit" | "stop" => {
                            break;
                        }
                        _ => {
                            current_terminal.write_line(&format!("Unknown command: {}", input));
                            continue;
                        }
                    }
                }
                "setup" => match setup_step {
                    0 => {
                        self.launch_config.set_string("Host", input.clone());
                        current_terminal.write_line("Which port should the cloud run on?");
                        setup_step += 1;
                        continue;
                    }
                    1 => {
                        self.launch_config
                            .set_integer("Port", input.clone().parse::<Number>().unwrap());
                        current_terminal
                            .write_line("How many memory should use the cloud? (in MB)");
                        setup_step += 1;
                        continue;
                    }
                    2 => {
                        self.launch_config
                            .set_integer("Memory", input.clone().parse::<Number>().unwrap());
                        current_terminal.write_line("Do you agree with the minecraft eula? (yes/no)");
                        setup_step += 1;
                        continue;
                    }
                    3 => {
                        if (input.clone() != "yes") {
                            current_terminal.write_line(
                                "You must agree with the minecraft eula to use this cloud.",
                            );
                            continue;
                        }
                        self.launch_config.set_boolean("Eula", true);
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
                _ => {
                    if current_terminal_name.starts_with("service-") {
                        if command == "leave" {
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
