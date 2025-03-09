use std::path::PathBuf;

use crate::conf::Config;

pub fn select_kernel(kernel: &Option<PathBuf>) {
    let mut config = Config::load();
    match kernel {
        Some(path) => {
            config.kernel = Some(path.clone());
            config.save();
        }
        None => {
            todo!("Implement kernel selection logic");
        }
    }
}
