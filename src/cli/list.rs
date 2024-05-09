use std::{collections::HashMap, path::PathBuf};

use clap::{Args, ValueEnum};
use convert_case::{Case as ConvertCase, Casing};
use toml::Value;

use crate::{metadata::Metadata, rv_to_map};

#[derive(Args, Debug)]
pub struct List {
    #[arg(long, value_enum)]
    pub case: Option<Case>,
    #[arg(long, value_enum, default_value_t)]
    pub format: Format,
    #[arg(long)]
    pub profile: Option<String>,
    #[arg(long)]
    pub path: Option<PathBuf>,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Case {
    /// lower case
    Lower,
    /// UPPER CASE
    Upper,
    /// camelCase
    Camel,
    /// UpperCamelCase
    UpperCamel,
    /// snake_case
    Snake,
    /// SCREAMING_SNAKE_CASE
    ScreamingSnake,
    /// kebab-case
    Kebab,
    /// Upper-Kebab-Case
    UpperKebab,
    /// COBOL-CASE
    Cobol,
    /// flatcase
    Flat,
    /// UPPERFLATCASE
    UpperFlat,
}

impl From<Case> for ConvertCase {
    fn from(value: Case) -> ConvertCase {
        match value {
            Case::Lower => ConvertCase::Lower,
            Case::Upper => ConvertCase::Upper,
            Case::Camel => ConvertCase::Camel,
            Case::UpperCamel => ConvertCase::UpperCamel,
            Case::Snake => ConvertCase::Snake,
            Case::ScreamingSnake => ConvertCase::ScreamingSnake,
            Case::Kebab => ConvertCase::Kebab,
            Case::UpperKebab => ConvertCase::UpperKebab,
            Case::Cobol => ConvertCase::Cobol,
            Case::Flat => ConvertCase::Flat,
            Case::UpperFlat => ConvertCase::UpperFlat,
        }
    }
}

#[derive(Clone, Debug, Default, ValueEnum)]
pub enum Format {
    /// `"var": "value"`   .json
    #[default]
    Json,
    /// `var = value`      .toml
    Toml,
    /// `var = "value"`    .tfvars
    Tfvars,
    /// `var=value`        .env
    Env,
    /// `export var=value` .envrc
    Envrc,
    /// `var=value`        inline CLI arguments
    Args,
    /// `-e var=value`     inline Docker CLI envrionment variables
    DockerArgs,
    /// `-var var=value`   inline Terraform CLI variables
    TfvarsArgs,
}

impl List {
    pub fn list(&self) {
        let metadata = Metadata::load();

        let rv_path = match self.path.as_ref() {
            Some(inner) => inner.clone().join("rv.toml"),
            None => {
                let current_dir = std::env::current_dir().unwrap();
                PathBuf::from(&current_dir).join("rv.toml")
            },
        };
        if !rv_path.exists() {
            return
        }

        let mut result: HashMap<String, String> = HashMap::new();

        let current_profile = match self.profile.as_ref() {
            Some(inner) => inner.clone(),
            None => metadata
               .profiles
               .get(&rv_path)
               .unwrap()
               .name
               .clone(),
        };

        let rv_file = std::fs::read_to_string(rv_path.to_str().unwrap()).unwrap();

        let mut rv: Value = toml::from_str(&rv_file).unwrap();
        for (key, value) in rv.as_table().unwrap() {
            if let Value::String(value) = value {
                let mut key = key.clone();
                if let Some(case) = self.case.as_ref() {
                    key = key.to_case(case.clone().into());
                }
                result.insert(key, value.clone());
            }
        }
        for value in current_profile.split('.') {
            rv = rv.get(value).unwrap().clone();
        }

        rv_to_map(None, &mut rv, &mut result, &self.case);

        let list: String = match self.format {
            Format::Json => serde_json::to_string_pretty(&result).unwrap(),
            Format::Toml => toml::to_string(&result).unwrap(),
            Format::Env => result
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<String>>()
                .join("\n"),
            Format::Envrc => result
                .iter()
                .map(|(k, v)| format!("export {}={}", k, v))
                .collect::<Vec<String>>()
                .join("\n"),
            Format::Args => result
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<String>>()
                .join(" "),
            Format::DockerArgs => result
                .iter()
                .map(|(k, v)| format!("-e {}={}", k, v))
                .collect::<Vec<String>>()
                .join(" "),
            Format::TfvarsArgs => result
                .iter()
                .map(|(k, v)| format!("-var {}={}", k, v))
                .collect::<Vec<String>>()
                .join(" "),
            Format::Tfvars => {
                let longest = result
                    .keys()
                    .max_by_key(|k| k.len())
                    .unwrap()
                    .len();
                result
                    .iter()
                    .map(|(k, v)| format!("{:<longest$} = {:?}", k, v))
                    .collect::<Vec<String>>()
                    .join("\n")
            },
        };
        println!("{}", list);
    }
}
