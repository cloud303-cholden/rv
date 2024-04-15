use std::path::PathBuf;

use clap::Args;

use crate::{config::Config, metadata::Metadata};

#[derive(Args, Debug)]
pub struct Clear;

impl Clear {
    pub fn clear(&self) {
        let config = Config::load();
        let mut metadata = Metadata::load();

        let current_dir = std::env::current_dir().unwrap();
        let rv_path = PathBuf::from(&current_dir).join("rv.toml");
        let mut cmd = String::new();

        let mut unset = String::new();
        let mut unset_changed = false;
        if let Some(current_profile) = metadata
            .profiles
            .get(&rv_path) {

            if let Some(current_vars) = current_profile.variables.clone() {
                unset_changed = true;
                for var in current_vars {
                    cmd.push_str(format!("unset {}\n", var).as_str());
                    unset.push_str(&config.removed.paint(&var));
                }
            }
            metadata.profiles.remove(&rv_path);
        }
        if unset_changed {
            println!(
                "{}{}{}",
                config.deactivated.paint(""),
                config.deactivated_dir.paint(current_dir.to_str().unwrap()),
                unset,
            );
        }
        metadata.save();
    }
}
