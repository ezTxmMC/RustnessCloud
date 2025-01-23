use serde_json::Number;
use super::grouptypes::{ProxySoftwareType, ServerSoftwareType, StoreType};

fn create_proxy_group(name: &str,
                      min_memory: Number, max_memory: Number, max_players: Number,
                      store_type: StoreType, proxy_software_type: ProxySoftwareType, version: String,
                      priority: Number, port: Number, maintenance: bool, permission: String) {
        
}

fn create_lobby_group(name: &str,
                      min_memory: Number, max_memory: Number, max_players: Number,
                      store_type: StoreType, proxy_software_type: ProxySoftwareType, version: String,
                      priority: Number, port: Number, permission: String, java: String,
                      new_service_procent: Number, min_online_count: Number, max_online_count: Number) {

}

fn create_server_group(name: &str,
                       min_memory: Number, max_memory: Number, max_players: Number,
                       store_type: StoreType, server_software_type: ServerSoftwareType, version: String,
                       priority: Number, port: Number, permission: String, java: String,
                       new_service_procent: Number, min_online_count: Number, max_online_count: Number) {

}
