use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::Profile;

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {
    #[serde(flatten)]
    pub profiles: HashMap<PathBuf, Profile>,
}

impl Metadata {
    pub fn load() -> Metadata {
        let metadata_file = dirs::data_dir().unwrap().join("rv").join("metadata.json");
        let metadata_str = std::fs::read_to_string(metadata_file).unwrap();
        serde_json::from_str(&metadata_str).unwrap()
    }

    pub fn save(&self) {
        let metadata_file = dirs::data_dir().unwrap().join("rv").join("metadata.json");
        std::fs::write(metadata_file, serde_json::to_string(self).unwrap()).unwrap();
    }
}
