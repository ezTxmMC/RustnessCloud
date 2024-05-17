use serde_json::{json, Value, Map};
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

pub struct JsonConfig {
    config_path: String,
    config_name: String,
    json_object: Map<String, Value>,
}

impl JsonConfig {
    pub fn new(path: &str, config_name: &str) -> Self {
        let config_name = format!("{}.json", config_name);
        let config_path = path.to_string();
        let full_path = format!("{}/{}", path, config_name);

        if !Path::new(path).exists() {
            fs::create_dir_all(path).expect("Failed to create directory");
        }

        let json_object = if !Path::new(&full_path).exists() {
            let file = File::create(&full_path).expect("Failed to create config file");
            drop(file);
            let json_object = Map::new();
            let config = JsonConfig {
                config_path: config_path.clone(),
                config_name: config_name.clone(),
                json_object: json_object.clone(),
            };
            config.save();
            json_object
        } else {
            let config_json = Self::read_file(&full_path);
            serde_json::from_str(&config_json).unwrap_or_else(|_| Map::new())
        };

        JsonConfig {
            config_path,
            config_name,
            json_object,
        }
    }

    pub fn set(&mut self, key: &str, value: String) {
        self.json_object.insert(key.to_string(), serde_json::Value::String(value));
        self.save();
    }

    pub fn remove(&mut self, key: &str) {
        self.json_object.remove(key);
        self.save();
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.json_object.get(key)
    }

    pub fn add_default(&mut self, key: &str, value: String) {
        if self.get(key).is_none() {
            self.set(key, value);
        }
    }

    fn save(&self) {
        let full_path = format!("{}/{}", self.config_path, self.config_name);
        let json_string = serde_json::to_string(&self.json_object).expect("Failed to serialize JSON");
        let mut file = File::create(&full_path).expect("Failed to open config file for writing");
        file.write_all(json_string.as_bytes()).expect("Failed to write JSON to file");
    }

    fn read_file(file_path: &str) -> String {
        let file = File::open(file_path).expect("Failed to open config file for reading");
        let reader = BufReader::new(file);
        reader.lines().collect::<Result<String, _>>().expect("Failed to read config file")
    }
}