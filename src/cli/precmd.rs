use std::path::PathBuf;

use clap::Args;
use toml::Value;

use crate::{config::Config, metadata::Metadata, parse_rv};

#[derive(Args, Debug)]
pub struct Precmd;

impl Precmd {
    pub fn precmd(&self) {
        let config = Config::load();

        let previous_dir = std::env::var("OLDPWD").unwrap();
        let current_dir = std::env::current_dir().unwrap();
        let check = std::env::var("RV_CHECK").ok();

        let mut metadata = Metadata::load();

        let mut cmd = String::new();

        let rv_path = PathBuf::from(&previous_dir).join("rv.toml");
        let mut unset = String::new();
        let mut unset_changed = false;
        let mut previous_profile_name = String::new();

        if check.is_some() {
            // Directory changed
            if let Some(previous_profile) = metadata
                .profiles
                .get(&rv_path) {

                previous_profile_name = previous_profile.name.clone();
                if let Some(previous_vars) = previous_profile.variables.clone() {
                    unset_changed = true;
                    for var in previous_vars {
                        cmd.push_str(format!("unset {}\n", var).as_str());
                        unset.push_str(&config.removed.paint(&var));
                    }
                }
            }
        }

        let rv_path = PathBuf::from(&current_dir).join("rv.toml");
        let mut export = String::new();
        let mut export_changed = false;
        let mut current_profile_name = String::new();
        if rv_path.exists() {
            if let Some(current_profile) = metadata
                .profiles
                .get_mut(&rv_path) {

                current_profile_name = current_profile.name.clone();
            
                let rv_file = std::fs::read_to_string(rv_path.to_str().unwrap()).unwrap();
                current_profile.variables = Some(Vec::new());

                let mut rv: Value = toml::from_str(&rv_file).unwrap();
                for (key, value) in rv.as_table().unwrap() {
                    if let Value::String(value) = value {
                        current_profile.variables.as_mut().unwrap().push(key.clone());
                        if let Ok(val) = std::env::var(key) {
                            if val != *value {
                                export_changed = true;
                                cmd.push_str(format!("export {}={}\n", key, value).as_str());
                                export.push_str(&config.changed.paint(key));
                            }
                        } else {
                            export_changed = true;
                            cmd.push_str(format!("export {}={}\n", key, value).as_str());
                            export.push_str(&config.added.paint(key));
                        }
                    }
                }
                for value in current_profile_name.split('.') {
                    rv = rv.get(value).unwrap().clone();
                }

                parse_rv(None, &mut rv, current_profile, &mut export_changed, &mut cmd, &mut export, &config);
            }
            metadata.save();
        }

        let home_dir = dirs::home_dir().unwrap();
        let home_dir = home_dir.to_str().unwrap();

        let mut previous_dir = previous_dir.replace(home_dir, "~");
        previous_dir.push(':');
        previous_dir.push_str(previous_profile_name.as_str());

        let mut current_dir = current_dir.to_str().unwrap().replace(home_dir, "~");
        current_dir.push(':');
        current_dir.push_str(current_profile_name.as_str());

        let mut unset_len = previous_dir.len();
        let mut export_len = current_dir.len();
        if unset_len > export_len {
            export_len = unset_len - export_len;
            unset_len = 0;
        } else {
            unset_len = export_len - unset_len;
            export_len = 0;
        }

        if unset_changed {
            println!(
                "echo '{}{}{:>unset_len$}{}'",
                config.deactivated.paint(""),
                config.deactivated_dir.paint(&previous_dir),
                "",
                unset,
            );
        }

        if export_changed {
            println!(
                "echo '{}{}{:>export_len$}{}'",
                config.activated.paint(""),
                config.activated_dir.paint(&current_dir),
                "",
                export,
            );
        }

        println!("unset RV_CHECK");
        println!("{}", cmd);
    }
}
