use std::convert::Infallible;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

use crate::conf::Config;

fn _jupyter_runtime_dir() -> Result<PathBuf, Infallible> {
    let jupyter_runtime_cmd = Command::new("jupyter")
        .args(["--runtime-dir"])
        .output()
        .expect("jupyter should be installed");
    let jupyter_runtime_output = String::from_utf8_lossy(&jupyter_runtime_cmd.stdout);
    PathBuf::from_str(
        jupyter_runtime_output
            .strip_suffix("\n")
            .expect("jupyter should report runtime directory"),
    )
}

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
