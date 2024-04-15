use std::{collections::HashMap, path::PathBuf};

use clap::Args;
use toml::Value;

use crate::{metadata::Metadata, rv_to_map};

#[derive(Args, Debug)]
pub struct Get {
    pub key: String,
}

impl Get {
    pub fn get(&self) {
        let metadata = Metadata::load();

        let current_dir = std::env::current_dir().unwrap();
        let rv_path = PathBuf::from(&current_dir).join("rv.toml");
        let mut result: HashMap<String, String> = HashMap::new();
        if rv_path.exists() {
            if let Some(current_pwd) = metadata
                .profiles
                .get(&rv_path) {

                let current_profile = current_pwd.name.clone();
            
                let rv_file = std::fs::read_to_string(rv_path.to_str().unwrap()).unwrap();

                let mut rv: Value = toml::from_str(&rv_file).unwrap();
                for (key, value) in rv.as_table().unwrap() {
                    if let Value::String(value) = value {
                        result.insert(key.clone(), value.clone());
                    }
                }
                for value in current_profile.split('.') {
                    rv = rv.get(value).unwrap().clone();
                }

                rv_to_map(None, &mut rv, &mut result);

                println!("{}", result.get(&self.key).unwrap_or(&String::from("null")));
            }
        }
    }
}
