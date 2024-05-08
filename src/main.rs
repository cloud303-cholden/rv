use std::collections::HashMap;

use clap::Parser;
use cli::Cli;
use convert_case::Casing;
use serde::{Deserialize, Serialize};
use toml::Value;

use crate::config::Config;

mod cli;
mod config;
mod metadata;

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    pub name: String,
    pub variables: Option<Vec<String>>,
}

fn main() {
    let cli = Cli::parse();
    cli.execute();
}

fn rv_to_map(
    key: Option<&String>,
    value: &mut Value,
    map: &mut HashMap<String, String>,
    case: &Option<cli::list::Case>,
) {
    match value {
        Value::Table(table) => {
            for (key, value) in table {
                rv_to_map(Some(key), value, map, case);
            }
        },
        value => {
            let value = value.as_str().unwrap();
            let mut key = key.unwrap().clone();
            if case.is_some() {
                key = key.to_case(case.clone().unwrap().into());
            }
            map.insert(key, value.to_string());
        },
    }
}

fn parse_rv(
    key: Option<&String>,
    outer: &mut Value,
    current_pwd: &mut Profile,
    export_changed: &mut bool,
    cmd: &mut String,
    export: &mut String,
    config: &Config,
) {
    match outer {
        Value::Table(inner) => {
            for (key, value) in inner {
                parse_rv(Some(key), value, current_pwd, export_changed, cmd, export, config);
            }
        },
        outer => {
            let value = outer.as_str().unwrap();
            let key = key.unwrap();
            current_pwd.variables.as_mut().unwrap().push(key.clone());
            if let Ok(val) = std::env::var(key) {
                if val != value {
                    *export_changed = true;
                    cmd.push_str(format!("export {}={}\n", key, value).as_str());
                    export.push_str(&config.changed.paint(key));
                }
            } else {
                *export_changed = true;
                cmd.push_str(format!("export {}={}\n", key, value).as_str());
                export.push_str(&config.added.paint(key));
            }
        },
    }
}
