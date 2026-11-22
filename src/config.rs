use std::fs;

pub struct Config {
    pub history_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self { history_size: 1000 }
    }
}

pub fn load_config() -> Config {
    let mut default_config = Config { history_size: 1000 };

    if let Some(mut path) = dirs::config_dir() {
        path.push("jodsh");
        path.push("config.toml");

        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(parsed) = content.parse::<toml::Value>() {
                if let Some(hist) = parsed.get("history_size").and_then(|v| v.as_integer()) {
                    default_config.history_size = hist as usize;
                }
            } else {
                eprintln!(
                    "jodsh: warning: failed to parse config file {}",
                    path.display()
                );
            }
        }
    }

    default_config
}
