use serde::Deserialize;
use nu_ansi_term::{Color, Style};

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_activated")]
    pub activated: Format,
    #[serde(default = "default_activated_dir")]
    pub activated_dir: Format,
    #[serde(default = "default_deactivated")]
    pub deactivated: Format,
    #[serde(default = "default_deactivated_dir")]
    pub deactivated_dir: Format,
    #[serde(default = "default_added")]
    pub added: Format,
    #[serde(default = "default_removed")]
    pub removed: Format,
    #[serde(default = "default_changed")]
    pub changed: Format,
}

impl Config {
    pub fn load() -> Config {
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
pub struct Format {
    pub symbol: Option<String>,
    #[serde(deserialize_with = "deserialize_style")]
    pub style: Option<Style>,
}

impl Format {
    pub fn paint(&self, s: &str) -> String {
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

