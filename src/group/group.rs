use crate::config::json_config::JsonConfig;
use serde_json::Number;
use std::fs;
use std::path::Path;

pub(crate) fn create_proxy_group(
    name: &str,
    min_memory: Number,
    max_memory: Number,
    max_players: Number,
    static_type: bool,
    proxy_software_type: String,
    version: String,
    priority: Number,
    port: Number,
    maintenance: bool,
    permission: String,
) {
    let mut group_file = JsonConfig::new("groups/proxies", name);
    group_file.set_string("name", name.to_string());
    group_file.set_string("templateName", name.to_string());
    group_file.set_integer("minimumMemory", min_memory);
    group_file.set_integer("maximumMemory", max_memory);
    group_file.set_integer("maximumPlayers", max_players);
    group_file.set_boolean("static", static_type);
    group_file.set_string("software", proxy_software_type.to_string());
    group_file.set_string("version", version.to_string());
    group_file.set_integer("priority", priority);
    group_file.set_integer("port", port);
    group_file.set_boolean("maintenance", maintenance);
    group_file.set_string("permission", permission.to_string());
    if !Path::new(format!("templates/ {}", name.to_string()).as_str()).exists() {
        fs::create_dir_all(format!("templates/ {}", name.to_string()).as_str())
            .expect("Failed to create template directory");
    }
}

pub(crate) fn create_lobby_group(
    name: &str,
    min_memory: Number,
    max_memory: Number,
    max_players: Number,
    static_type: bool,
    proxy_software_type: String,
    version: String,
    priority: Number,
    port: Number,
    permission: String,
    java: String,
    new_service_procent: Number,
    min_online_count: Number,
    max_online_count: Number,
) {
    let mut group_file = JsonConfig::new("groups/proxies", name);
    group_file.set_string("name", name.to_string());
    group_file.set_string("templateName", name.to_string());
    group_file.set_integer("minimumMemory", min_memory);
    group_file.set_integer("maximumMemory", max_memory);
    group_file.set_integer("maximumPlayers", max_players);
    group_file.set_boolean("static", static_type);
    group_file.set_string("software", proxy_software_type.to_string());
    group_file.set_string("version", version.to_string());
    group_file.set_integer("priority", priority);
    group_file.set_integer("port", port);
    group_file.set_string("permission", permission.to_string());
    group_file.set_string("java", java.to_string());
    group_file.set_integer("newServiceProcent", new_service_procent);
    group_file.set_integer("minimumOnlineCount", min_online_count);
    group_file.set_integer("maximumOnlineCount", max_online_count);
    if !Path::new(format!("templates/ {}", name.to_string()).as_str()).exists() {
        fs::create_dir_all(format!("templates/ {}", name.to_string()).as_str())
            .expect("Failed to create template directory");
    }
}

pub(crate) fn create_server_group(
    name: &str,
    min_memory: Number,
    max_memory: Number,
    max_players: Number,
    static_type: bool,
    proxy_software_type: String,
    version: String,
    priority: Number,
    port: Number,
    permission: String,
    java: String,
    new_service_procent: Number,
    min_online_count: Number,
    max_online_count: Number,
) {
    let mut group_file = JsonConfig::new("groups/proxies", name);
    group_file.set_string("name", name.to_string());
    group_file.set_string("templateName", name.to_string());
    group_file.set_integer("minimumMemory", min_memory);
    group_file.set_integer("maximumMemory", max_memory);
    group_file.set_integer("maximumPlayers", max_players);
    group_file.set_boolean("static", static_type);
    group_file.set_string("software", proxy_software_type.to_string());
    group_file.set_string("version", version.to_string());
    group_file.set_integer("priority", priority);
    group_file.set_integer("port", port);
    group_file.set_string("permission", permission.to_string());
    group_file.set_string("java", java.to_string());
    group_file.set_integer("newServiceProcent", new_service_procent);
    group_file.set_integer("minimumOnlineCount", min_online_count);
    group_file.set_integer("maximumOnlineCount", max_online_count);
    if !Path::new(format!("templates/ {}", name.to_string()).as_str()).exists() {
        fs::create_dir_all(format!("templates/ {}", name.to_string()).as_str())
            .expect("Failed to create template directory");
    }
}
