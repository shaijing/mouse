use std::path::Path;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub vertical_reverse: bool,
    pub horizontal_reverse: bool,
    pub scroll_sensitivity: i64,
    pub mouse_reverse: bool,
    pub trackpad_reverse: bool,
}

impl Config{
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let config_content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&config_content)?;
        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            vertical_reverse: true,
            horizontal_reverse: false,
            scroll_sensitivity: 1,
            mouse_reverse: true,
            trackpad_reverse: false,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let config = Config::load_from_file("target/debug/mouse.toml").unwrap();
        assert_eq!(config.vertical_reverse, true);
    }
}