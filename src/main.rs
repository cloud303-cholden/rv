use std::{collections::HashMap, env, path::PathBuf};

use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use toml::Value;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(hide = true)]
    Chpwd,
    #[clap(hide = true)]
    Precmd,
    Set(Set),
}

#[derive(Args, Debug)]
struct Set {
    profile:  String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Metadata {
    #[serde(flatten)]
    activated: HashMap<PathBuf, Activated>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Activated {
    profile: String,
    variables: Option<Vec<String>>,
}

fn main() {

    let cli = Cli::parse();

    match &cli.command {
        Commands::Set(args) => {
            let metadata_file = dirs::data_dir().unwrap().join("rv").join("metadata.json");
            let metadata_str = std::fs::read_to_string(&metadata_file).unwrap();
            let mut metadata: Metadata = serde_json::from_str(&metadata_str).unwrap();
            let profile = &args.profile;
            let current_pwd = env::current_dir().unwrap().join("rv.toml");
            metadata
                .activated
                .entry(current_pwd)
                .and_modify(|pwd| {
                    pwd.profile = profile.to_string();
                })
                .or_insert(Activated {
                    profile: profile.to_string(),
                    variables: None,
                });
            std::fs::write(&metadata_file, serde_json::to_string(&metadata).unwrap()).unwrap();
        },
        Commands::Chpwd => {
            println!("export RV_CHECK=1");
        },
        Commands::Precmd => {
            let previous_pwd = env::var("OLDPWD").unwrap();
            let current_pwd = env::var("PWD").unwrap();
            let check = env::var("RV_CHECK").ok();

            let metadata_file = dirs::data_dir().unwrap().join("rv").join("metadata.json");
            let metadata_str = std::fs::read_to_string(&metadata_file).unwrap();
            let mut metadata: Metadata = serde_json::from_str(&metadata_str).unwrap();

            let mut cmd = String::new();

            let previous_rv = PathBuf::from(&previous_pwd).join("rv.toml");
            let mut unset = String::new();
            let mut unset_changed = false;

            if check.is_some() {
                // Directory changed
                if let Some(previous_pwd) = metadata
                    .activated
                    .get(&previous_rv) {

                    if let Some(previous_vars) = previous_pwd.variables.clone() {
                        unset_changed = true;
                        for var in previous_vars {
                            cmd.push_str(format!("unset {}\n", var).as_str());
                            unset.push_str(format!(" \x1b[1;31m-{}\x1b[0m", var).as_str());
                        }
                    }
                }
            }

            let current_rv = PathBuf::from(&current_pwd).join("rv.toml");
            let mut export = String::new();
            let mut export_changed = false;
            if current_rv.exists() {
                if let Some(current_pwd) = metadata
                    .activated
                    .get_mut(&current_rv) {

                    let current_profile = current_pwd.profile.clone();
                
                    let file = std::fs::read_to_string(current_rv.to_str().unwrap()).unwrap();
                    let mut config: Value = toml::from_str(&file).unwrap();
                    for value in current_profile.split('.') {
                        config = config.get(value).unwrap().clone();
                    }

                    current_pwd.variables = Some(Vec::new());
                    if let Some(variables) = config.as_table() {
                        for (key, value) in variables {
                            let key = key.to_uppercase();
                            let value = value.as_str().unwrap();
                            current_pwd.variables.as_mut().unwrap().push(key.clone());
                            if let Ok(val) = env::var(&key) {
                                if val == value {
                                    continue
                                } else {
                                    export_changed = true;
                                    cmd.push_str(format!("export {}={}\n", key, value).as_str());
                                    export.push_str(format!(" \x1b[1m\x1b[38;5;208m~{}\x1b[0m", key).as_str());
                                }
                            } else {
                                export_changed = true;
                                cmd.push_str(format!("export {}={}\n", key, value).as_str());
                                export.push_str(format!(" \x1b[1;32m+{}\x1b[0m", key).as_str());
                            }
                        }
                    }
                }
                std::fs::write(&metadata_file, serde_json::to_string(&metadata).unwrap()).unwrap();
            }

            let home_dir = dirs::home_dir().unwrap();
            let home_dir = home_dir.to_str().unwrap();
            let previous_pwd = previous_pwd.replace(home_dir, "~");
            let current_pwd = current_pwd.replace(home_dir, "~");
            let mut unset_len = previous_pwd.len();
            let mut export_len = current_pwd.len();
            if unset_len > export_len {
                export_len = unset_len - export_len;
                unset_len = 0;
            } else {
                unset_len = export_len - unset_len;
                export_len = 0;
            }

            if unset_changed {
                println!("echo '\x1b[1;31mrv ↓\x1b[0m {}{:>unset_len$}{}'", previous_pwd, "", unset);
            }

            if export_changed {
                println!("echo '\x1b[1;32mrv ↑\x1b[0m {}{:>export_len$}{}'", current_pwd, "", export);
            }

            println!("unset RV_CHECK");
            println!("{}", cmd);
        }
    }
}
