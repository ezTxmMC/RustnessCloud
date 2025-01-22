mod terminal {
    pub(crate) mod terminal;
    pub(crate) mod terminal_manager;
}
mod config {
    pub(crate) mod json_config;
}
mod database {
    pub(crate) mod database_processor;
    pub(crate) mod mongodb;
    pub(crate) mod mysql;
}
mod downloader {
    pub(crate) mod url_downloader;
    pub(crate) mod proxy_downloader;
    pub(crate) mod software_downloader;
}
mod group {
    pub(crate) mod group;
    pub(crate) mod grouptypes;
}
mod service {
    pub(crate) mod service;
}

use terminal::terminal_manager::TerminalManager;
use config::json_config::JsonConfig;

fn main() {
    let config: JsonConfig = JsonConfig::new(".", "launch");
    let mut terminal_manager: TerminalManager = TerminalManager::new(config);
    terminal_manager.start_terminal();
}
