mod terminal {
    pub(crate) mod terminal;
    pub(crate) mod terminal_manager;
}
mod config {
    pub(crate) mod json_config;
}
mod database;
mod downloader {
    pub(crate) mod url_downloader;
    pub(crate) mod proxy_downloader;
    pub(crate) mod software_downloader;
}
mod groups {
    pub(crate) mod group;
    pub(crate) mod grouptypes;
}
mod service;

use terminal::terminal_manager::TerminalManager;
use config::json_config::JsonConfig;

fn main() {
    let config: JsonConfig = JsonConfig::new("data", "launch");
    let mut terminal_manager: TerminalManager = TerminalManager::new(config);
    terminal_manager.start_terminal();
}
