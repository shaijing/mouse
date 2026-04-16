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
    fn test_default_config() {
        let config = Config::default();
        assert!(config.vertical_reverse);
        assert!(!config.horizontal_reverse);
        assert_eq!(config.scroll_sensitivity, 1);
        assert!(config.mouse_reverse);
        assert!(!config.trackpad_reverse);
    }

    #[test]
    fn test_parse_config() {
        let config_content = r#"
vertical_reverse = false
horizontal_reverse = true
scroll_sensitivity = 2
mouse_reverse = false
trackpad_reverse = true
"#;
        let config: Config = toml::from_str(config_content).unwrap();
        assert!(!config.vertical_reverse);
        assert!(config.horizontal_reverse);
        assert_eq!(config.scroll_sensitivity, 2);
        assert!(!config.mouse_reverse);
        assert!(config.trackpad_reverse);
    }
}