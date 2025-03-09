use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

const SHUX_CONFIG_ENV: &str = "SHUX_CONFIG";
const SHUX_KERNEL_ENV: &str = "SHUX_KERNEL";
const SHUX_CONFIG: &str = ".shux.toml";

/// Configuration for running shux
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub kernel: Option<PathBuf>,
}

/// Set the location of the shux config file
pub fn set_env(path: &PathBuf) {
    env::set_var(SHUX_CONFIG_ENV, path);
}

fn find_project_root() -> PathBuf {
    let current_dir = env::current_dir().expect("Failed to get current directory");

    let mut current_path = current_dir.as_path();
    while let Some(parent) = current_path.parent() {
        let toml_path = parent.join(SHUX_CONFIG);
        if toml_path.exists() {
            return parent.to_path_buf();
        }
        current_path = parent;
    }

    current_dir
}

/// Return the location to use for the shux config file
///
/// Determines whether the environment variable SHUX_CONFIG is set, and if so,
/// returns the value of the variable as a PathBuf. Otherwise, check for .shux.toml at project root
fn get_config_path() -> PathBuf {
    env::var(SHUX_CONFIG_ENV)
        .ok()
        // Use environment variable if set
        .and_then(|path| Some(PathBuf::from(path)))
        // Otherwise determine project root and use .shux.toml
        .or_else(|| Some(find_project_root().join(SHUX_CONFIG)))
        .expect("Should always have a config path")
}

impl Config {
    /// Save shux config to the project root, or the current directory if no project root is found
    pub fn save(&self) {
        let config_path = get_config_path();
        let toml_string = toml::to_string(self).expect("Failed to serialize config");
        fs::write(config_path, toml_string).expect("Failed to write config file");
    }
    /// Load shux config from the project root, or the current directory if no project root is found
    pub fn load() -> Self {
        let project_root = find_project_root();
        let config_path = project_root.join(SHUX_CONFIG);
        let mut conf = if config_path.exists() {
            let toml_string = fs::read_to_string(config_path).expect("Failed to read config file");
            toml::from_str(&toml_string).expect("Failed to parse config file")
        } else {
            Config { kernel: None }
        };

        // If the kernel is not set in the config file, check the environment variable
        conf.kernel = conf
            .kernel
            .ok_or_else(|| {
                env::var(SHUX_KERNEL_ENV)
                    .ok()
                    .and_then(|path| Some(PathBuf::from(path)))
            })
            .ok();

        conf
    }
}
