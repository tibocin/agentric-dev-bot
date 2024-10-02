use std::fs;

use log::info;
use serde::Deserialize;
use tokio::runtime::Runtime;

#[derive(Deserialize)]
struct Config {
    // Define your configuration fields here
    app_name: String,
    log_level: String,
}

fn load_config(config_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_content)?;
    Ok(config)
}

fn setup_logging(log_level: &str) {
    let log_level = match log_level {
        "debug" => log::LevelFilter::Debug,
        "info" => log::LevelFilter::Info,
        "warn" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        _ => log::LevelFilter::Info,
    };

    let _log_builder = env_logger::Builder::new().filter_level(log_level).init();
}

async fn main_event_loop() {
    // Your main event loop code here
    info!("Main event loop started");
}

fn main() {
    let config_path = "config/config.toml";
    let config = match load_config(config_path) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    setup_logging(&config.log_level);

    info!("Starting application: {}", config.app_name);

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        main_event_loop().await;
    });
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_load_config_success() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("config.toml");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "app_name = 'TestApp'\nlog_level = 'info'").unwrap();

        let config = load_config(file_path.to_str().unwrap()).unwrap();
        assert_eq!(config.app_name, "TestApp");
        assert_eq!(config.log_level, "info");
    }

    #[test]
    fn test_load_config_failure() {
        let result = load_config("non_existent_file.toml");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_main_event_loop() {
        // This test is a placeholder. You should add more specific assertions
        // based on what your main_event_loop function does.
        main_event_loop().await;
    }
}
