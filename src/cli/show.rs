use std::path::PathBuf;

use clap::Args;
use nu_ansi_term::{Color, Style};

use crate::{config::Config, metadata::Metadata};

#[derive(Args, Debug)]
pub struct Show;

impl Show {
    pub fn show(&self) {
        let config = Config::load();
        let metadata = Metadata::load();

        let current_dir = std::env::current_dir().unwrap();
        let rv_path = PathBuf::from(&current_dir).join("rv.toml");
        if let Some(current_profile) = metadata
            .profiles
            .get(&rv_path) {
            if let Some(variables) = &current_profile.variables {
                let list: String = variables.join(" ");
                println!(
                    "{}{} {}",
                    config.activated.paint(""),
                    config.activated_dir.paint(&current_profile.name),
                    Style::new().bold().fg(Color::Green).paint(&list),
                );
            }
        }
    }
}
