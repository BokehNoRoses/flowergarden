use serde::{Deserialize, Serialize};
use std::env;
use std::fs::read_to_string;
use std::io::Result;
use std::path::PathBuf;
use toml;

/// TOML configuration file for Tauri program.
///
/// # Fields
/// `base`: `PathBuf` -> Entry point from which the Node tree will be built.
#[derive(Deserialize, Serialize)]
pub struct Config {
    pub base: PathBuf,
}

impl Config {
    /// Helper function that returns or creates the program configuration file.
    ///
    /// The program expects the `HOME` environment variable to be set. If it is not, the program will
    /// assume that the user's home directory is placed one level above the current working directory
    /// to look for its configuration file.
    ///
    /// # Examples
    /// On Linux: [~/.config/mpb/config.toml](file://~/.config/mpb/config.toml)
    /// Default: `../.config/mpb/config.toml`
    pub fn new() -> Result<Config> {
        let mut home = env::var_os("HOME").unwrap_or("..".into());
        home.push("/.config/mpb/config.toml");
        let path: PathBuf = PathBuf::from(&home);

        if path.try_exists()? {
            let config: Config = toml::from_str(read_to_string(path)?.as_str())
                .expect("An error occurred during deserialization of the config file.");
            return Ok(config);
        } else {
            home = env::var_os("HOME").unwrap_or("..".into());
            home.push("/Music/");
            return Ok(Config {
                base: PathBuf::from(home),
            });
        }
    }
}
