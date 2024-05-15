use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Write};

#[derive(Serialize, Deserialize)]
struct JsonConfig {
    file_path: String,
    values: HashMap<String, String>,
    auto_save: bool,
}

impl JsonConfig {
    fn get_value(&self, key: &str) -> Option<&String> {
        self.values.get(key)
    }

    fn set_value(&mut self, key: String, value: String) {
        self.values.insert(key, value);
    }

    fn file_path(&self) -> &str {
        &self.file_path
    }

    fn auto_save(&self) -> bool {
        self.auto_save
    }
}

impl JsonConfig {
    fn from_file(file_path: &str) -> Result<Self, Error> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        serde_json::from_str(&contents).map_err(|e| Error::new(ErrorKind::Other, e))
    }
    
    fn to_file(&self, file_path: &str) -> Result<(), Error> {
        let mut file = File::create(file_path)?;
        let json = serde_json::to_string_pretty(self)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}
