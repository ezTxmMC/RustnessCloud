use std::fs;
use std::path::Path;
use serde_json::Number;
use crate::config::json_config::JsonConfig;
use super::grouptypes::{ProxySoftwareType, ServerSoftwareType};

pub(crate) fn create_proxy_group(name: &str,
                      min_memory: Number, max_memory: Number, max_players: Number,
                      staticType: bool, proxy_software_type: String, version: String,
                      priority: Number, port: Number, maintenance: bool, permission: String) {
    let mut group_file = JsonConfig::new("groups/proxies", name);
    group_file.set_string("name", name.to_string());
    group_file.set_string("templateName", name.to_string());
    group_file.set_integer("minimumMemory", min_memory);
    group_file.set_integer("maximumMemory", max_memory);
    group_file.set_integer("maximumPlayers", max_players);
    group_file.set_boolean("static", staticType);
    group_file.set_string("software", proxy_software_type.to_string());
    group_file.set_string("version", version.to_string());
    group_file.set_integer("priority", priority);
    group_file.set_integer("port", port);
    group_file.set_boolean("maintenance", maintenance);
    group_file.set_string("permission", permission.to_string());
    if !Path::new(format!("templates/ {}", name.to_string()).as_str()).exists() {
        fs::create_dir_all(format!("templates/ {}", name.to_string()).as_str()).expect("Failed to create template directory");
    }
}

fn create_lobby_group(name: &str,
                      min_memory: Number, max_memory: Number, max_players: Number,
                      store_type: String, proxy_software_type: ProxySoftwareType, version: String,
                      priority: Number, port: Number, permission: String, java: String,
                      new_service_procent: Number, min_online_count: Number, max_online_count: Number) {

}

fn create_server_group(name: &str,
                       min_memory: Number, max_memory: Number, max_players: Number,
                       store_type: String, server_software_type: ServerSoftwareType, version: String,
                       priority: Number, port: Number, permission: String, java: String,
                       new_service_procent: Number, min_online_count: Number, max_online_count: Number) {

}
