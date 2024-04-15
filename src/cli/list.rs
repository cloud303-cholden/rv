use std::{collections::HashMap, path::PathBuf};

use clap::Args;
use toml::Value;

use crate::{metadata::Metadata, rv_to_map};

#[derive(Args, Debug)]
pub struct List {
    #[arg(long, group = "output_format")]
    pub json: bool,
    #[arg(long, group = "output_format")]
    pub toml: bool,
    #[arg(long, group = "output_format")]
    pub env: bool,
    #[arg(long, group = "output_format")]
    pub envrc: bool,
}

impl List {
    pub fn list(&self) {
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

                let list: String;
                if self.json {
                    list = serde_json::to_string_pretty(&result).unwrap();
                } else if self.toml {
                    list = toml::to_string(&result).unwrap();
                } else if self.env {
                    list = result
                        .iter()
                        .map(|(k, v)| format!("{}={}", k, v))
                        .collect::<Vec<String>>()
                        .join("\n");
                } else if self.envrc {
                    list = result
                        .iter()
                        .map(|(k, v)| format!("export {}={}", k, v))
                        .collect::<Vec<String>>()
                        .join("\n");
                } else {
                    list = serde_json::to_string_pretty(&result).unwrap();
                }
                println!("{}", list);
            }
        }
    }
}
