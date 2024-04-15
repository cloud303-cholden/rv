use clap::Args;

use crate::{metadata::Metadata, Profile};

#[derive(Args, Debug)]
pub struct Set {
    pub profile: String,
}

impl Set {
    pub fn set(&self) {
        let mut metadata = Metadata::load();
        let current_dir = std::env::current_dir().unwrap().join("rv.toml");
        metadata
            .profiles
            .entry(current_dir)
            .and_modify(|profile| {
                profile.name = self.profile.to_string();
            })
            .or_insert(Profile {
                name: self.profile.to_string(),
                variables: None,
            });
        metadata.save();
    }
}
