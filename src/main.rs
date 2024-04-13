use std::{collections::HashMap, env, path::PathBuf};

use clap::{Args, Parser, Subcommand};
use nu_ansi_term::{Color, Style};
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
    /// Activates a profile
    Set(Set),
    /// Shows the variables of the current profile
    Show,
    /// Outputs the variables and values of the current profile (default format is JSON)
    List(List),
    /// Outputs the value of a variable in the current profile
    Get(Get),
    /// Deactivates the current profile
    Clear,
}

#[derive(Args, Debug)]
struct Set {
    profile: String,
}

#[derive(Args, Debug)]
struct List {
    #[arg(long, group = "output_format")]
    json: bool,
    #[arg(long, group = "output_format")]
    toml: bool,
    #[arg(long, group = "output_format")]
    env: bool,
    #[arg(long, group = "output_format")]
    envrc: bool,
}

#[derive(Args, Debug)]
struct Get {
    key: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Metadata {
    #[serde(flatten)]
    profiles: HashMap<PathBuf, Profile>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Profile {
    name: String,
    variables: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(default = "default_activated")]
    activated: Format,
    #[serde(default = "default_activated_dir")]
    activated_dir: Format,
    #[serde(default = "default_deactivated")]
    deactivated: Format,
    #[serde(default = "default_deactivated_dir")]
    deactivated_dir: Format,
    #[serde(default = "default_added")]
    added: Format,
    #[serde(default = "default_removed")]
    removed: Format,
    #[serde(default = "default_changed")]
    changed: Format,
}

impl Config {
    fn load() -> Config {
        let config_path = dirs::config_dir().unwrap().join("rv").join("config.toml");
        match std::fs::read_to_string(config_path) {
            Ok(config) => toml::from_str(config.as_str()).unwrap(),
            Err(_) => Config::default(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            activated: default_activated(),
            activated_dir: default_activated_dir(),
            deactivated: default_deactivated(),
            deactivated_dir: default_deactivated_dir(),
            added: default_added(),
            removed: default_removed(),
            changed: default_changed(),
        }
    }
}

fn default_activated() -> Format {
    Format {
        symbol: Some("rv ↑ ".to_string()),
        style: Some(Style::new().bold().fg(Color::Green)),
    }
}

fn default_activated_dir() -> Format {
    Format {
        symbol: Some("".to_string()),
        style: Some(Style::new().fg(Color::White)),
    }
}

fn default_deactivated() -> Format {
    Format {
        symbol: Some("rv ↓ ".to_string()),
        style: Some(Style::new().bold().fg(Color::Red)),
    }
}

fn default_deactivated_dir() -> Format {
    Format {
        symbol: Some("".to_string()),
        style: Some(Style::new().fg(Color::White)),
    }
}

fn default_added() -> Format {
    Format {
        symbol: Some("  ".to_string()),
        style: Some(Style::new().bold().fg(Color::Green)),
    }
}

fn default_removed() -> Format {
    Format {
        symbol: Some("  ".to_string()),
        style: Some(Style::new().bold().fg(Color::Red)),
    }
}

fn default_changed() -> Format {
    Format {
        symbol: Some("  ".to_string()),
        style: Some(Style::new().bold().fg(Color::Fixed(208))),
    }
}

#[derive(Debug, Deserialize)]
struct Format {
    symbol: Option<String>,
    #[serde(deserialize_with = "deserialize_style")]
    style: Option<Style>,
}

impl Format {
    fn paint(&self, s: &str) -> String {
        match &self.style {
            Some(style) => format!("{}{}", style.paint(self.symbol.as_deref().unwrap_or("")), style.paint(s)),
            None => format!("{}{}", self.symbol.as_deref().unwrap_or(""), s),
        }
    }
}

fn deserialize_style<'de, D>(deserializer: D) -> Result<Option<Style>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => {
            let mut style = Style::new();
            let mut s = s.split_whitespace();
            let color = match s.next() {
                Some("black") => Color::Black,
                Some("darkgray") => Color::DarkGray,
                Some("red") => Color::Red,
                Some("lightred") => Color::LightRed,
                Some("green") => Color::Green,
                Some("lightgreen") => Color::LightGreen,
                Some("yellow") => Color::Yellow,
                Some("lightyellow") => Color::LightYellow,
                Some("blue") => Color::Blue,
                Some("lightblue") => Color::LightBlue,
                Some("purple") => Color::Purple,
                Some("lightpurple") => Color::LightPurple,
                Some("magenta") => Color::Magenta,
                Some("lightmagenta") => Color::LightMagenta,
                Some("cyan") => Color::Cyan,
                Some("lightcyan") => Color::LightCyan,
                Some("white") => Color::White,
                Some("lightgray") => Color::LightGray,
                Some("default") => Color::Default,
                Some(bits) => {
                    match bits.parse() {
                        Ok(fixed) => Color::Fixed(fixed),
                        Err(_) => {
                            let mut rgb = bits.split(',');
                            let r = rgb.next().unwrap().parse().unwrap();
                            let g = rgb.next().unwrap().parse().unwrap();
                            let b = rgb.next().unwrap().parse().unwrap();
                            Color::Rgb(r, g, b)
                        },
                    }
                }
                _ => return Ok(None),
            };
            style = style.fg(color);
            for elem in s {
                style = match elem {
                    "bold" => style.bold(),
                    "dimmed" => style.dimmed(),
                    "italic" => style.italic(),
                    "underline" => style.underline(),
                    "blink" => style.blink(),
                    "reverse" => style.reverse(),
                    "hidden" => style.hidden(),
                    "strikethrough" => style.strikethrough(),
                    _ => style,
                }
            }
            Ok(Some(style))
        },
        None => Ok(None),
    }
}

fn main() {

    let cli = Cli::parse();

    match &cli.command {
        Commands::Set(args) => {
            let metadata_file = dirs::data_dir().unwrap().join("rv").join("metadata.json");
            let metadata_str = std::fs::read_to_string(&metadata_file).unwrap();
            let mut metadata: Metadata = serde_json::from_str(&metadata_str).unwrap();
            let current_dir = env::current_dir().unwrap().join("rv.toml");
            metadata
                .profiles
                .entry(current_dir)
                .and_modify(|profile| {
                    profile.name = args.profile.to_string();
                })
                .or_insert(Profile {
                    name: args.profile.to_string(),
                    variables: None,
                });
            std::fs::write(&metadata_file, serde_json::to_string(&metadata).unwrap()).unwrap();
        },
        Commands::Chpwd => {
            println!("export RV_CHECK=1");
        },
        Commands::Precmd => {
            let config = Config::load();

            let previous_dir = env::var("OLDPWD").unwrap();
            let current_dir = env::current_dir().unwrap();
            let check = env::var("RV_CHECK").ok();

            let metadata_file = dirs::data_dir().unwrap().join("rv").join("metadata.json");
            let metadata_str = std::fs::read_to_string(&metadata_file).unwrap();
            let mut metadata: Metadata = serde_json::from_str(&metadata_str).unwrap();

            let mut cmd = String::new();

            let rv_path = PathBuf::from(&previous_dir).join("rv.toml");
            let mut unset = String::new();
            let mut unset_changed = false;

            if check.is_some() {
                // Directory changed
                if let Some(previous_profile) = metadata
                    .profiles
                    .get(&rv_path) {

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
            if rv_path.exists() {
                if let Some(current_profile) = metadata
                    .profiles
                    .get_mut(&rv_path) {

                    let profile_name = current_profile.name.clone();
                
                    let rv_file = std::fs::read_to_string(rv_path.to_str().unwrap()).unwrap();
                    current_profile.variables = Some(Vec::new());

                    let mut rv: Value = toml::from_str(&rv_file).unwrap();
                    for (key, value) in rv.as_table().unwrap() {
                        if let Value::String(value) = value {
                            current_profile.variables.as_mut().unwrap().push(key.clone());
                            if let Ok(val) = env::var(key) {
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
                    for value in profile_name.split('.') {
                        rv = rv.get(value).unwrap().clone();
                    }

                    parse_rv(None, &mut rv, current_profile, &mut export_changed, &mut cmd, &mut export, &config);
                }
                std::fs::write(&metadata_file, serde_json::to_string(&metadata).unwrap()).unwrap();
            }

            let home_dir = dirs::home_dir().unwrap();
            let home_dir = home_dir.to_str().unwrap();
            let previous_dir = previous_dir.replace(home_dir, "~");
            let current_dir = current_dir.to_str().unwrap().replace(home_dir, "~");
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
        },
        Commands::Show => {
            let config = Config::load();

            let metadata_file = dirs::data_dir().unwrap().join("rv").join("metadata.json");
            let metadata_str = std::fs::read_to_string(metadata_file).unwrap();
            let metadata: Metadata = serde_json::from_str(&metadata_str).unwrap();
            let current_dir = env::current_dir().unwrap();
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
        },
        Commands::List(args) => {
            let metadata_file = dirs::data_dir().unwrap().join("rv").join("metadata.json");
            let metadata_str = std::fs::read_to_string(metadata_file).unwrap();
            let metadata: Metadata = serde_json::from_str(&metadata_str).unwrap();
            let current_dir = env::current_dir().unwrap();
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
                    if args.json {
                        list = serde_json::to_string_pretty(&result).unwrap();
                    } else if args.toml {
                        list = toml::to_string(&result).unwrap();
                    } else if args.env {
                        list = result
                            .iter()
                            .map(|(k, v)| format!("{}={}", k, v))
                            .collect::<Vec<String>>()
                            .join("\n");
                    } else if args.envrc {
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
        },
        Commands::Get(args) => {
            let metadata_file = dirs::data_dir().unwrap().join("rv").join("metadata.json");
            let metadata_str = std::fs::read_to_string(metadata_file).unwrap();
            let metadata: Metadata = serde_json::from_str(&metadata_str).unwrap();
            let current_dir = env::current_dir().unwrap();
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

                    println!("{}", result.get(&args.key).unwrap_or(&String::from("null")));
                }
            }
        },
        Commands::Clear => {
            let config = Config::load();

            let metadata_file = dirs::data_dir().unwrap().join("rv").join("metadata.json");
            let metadata_str = std::fs::read_to_string(&metadata_file).unwrap();
            let mut metadata: Metadata = serde_json::from_str(&metadata_str).unwrap();
            let current_dir = env::current_dir().unwrap();
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
            std::fs::write(&metadata_file, serde_json::to_string(&metadata).unwrap()).unwrap();
        },
    }
}

fn rv_to_map(
    key: Option<&String>,
    value: &mut Value,
    map: &mut HashMap<String, String>,
) {
    match value {
        Value::Table(table) => {
            for (key, value) in table {
                rv_to_map(Some(key), value, map);
            }
        },
        value => {
            let value = value.as_str().unwrap();
            let key = key.unwrap();
            map.insert(key.clone(), value.to_string());
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
            if let Ok(val) = env::var(key) {
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
