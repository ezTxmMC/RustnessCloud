mod terminal {
    pub(crate) mod cloud_terminal;
    pub(crate) mod terminal_manager;
}
mod database;
mod config {
    pub(crate) mod json_config;
}
mod service;

mod groups {
    mod group;
    mod grouptypes;
}

use terminal::terminal_manager::TerminalManager;
use config::json_config::JsonConfig;

fn main() {
    let config: JsonConfig = JsonConfig::new("data", "launch");
    let mut terminal_manager: TerminalManager = TerminalManager::new(config);
    terminal_manager.start_terminal();
}
