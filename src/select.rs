use std::path::PathBuf;

use crate::conf::Config;

pub fn select_kernel(config: &mut Config, kernel: &Option<PathBuf>) {
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
