use glob::glob;
use log::trace;
use std::convert::Infallible;
use std::env::current_dir;
use std::fs;
use std::process::Command;
use std::str::FromStr;
use std::{env, path::PathBuf};

use dirs::home_dir;

pub(crate) fn jupyter_runtime_dir() -> PathBuf {
    if let Ok(p) = env::var("JUPYTER_RUNTIME_DIR") {
        PathBuf::from(p)
    } else {
        os_jupyter_runtime_dir()
    }
}

pub(crate) fn jupyter_data_dir() -> PathBuf {
    if let Ok(p) = env::var("JUPYTER_DATA_DIR") {
        PathBuf::from(p)
    } else {
        os_jupyter_data_dir()
    }
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn os_jupyter_runtime_dir() -> PathBuf {
    jupyter_data_dir().join("runtime")
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn os_jupyter_runtime_dir() -> PathBuf {
    if let Ok(p) = env::var("XDG_RUNTIME_DIR") {
        PathBuf::from(p).join("jupyter")
    } else {
        jupyter_data_dir().join("runtime")
    }
}

#[cfg(target_os = "macos")]
fn os_jupyter_data_dir() -> PathBuf {
    let home = home_dir().unwrap();
    home.join("Library").join("Jupyter")
}

#[cfg(target_os = "linux")]
fn os_jupyter_data_dir() -> PathBuf {
    if let Ok(p) = env::var("XDG_DATA_HOME") {
        PathBuf::from(p).join("jupyter")
    } else {
        let home = home_dir().unwrap();
        home.join(".local").join("share").join("jupyter")
    }
}

#[cfg(target_os = "windows")]
fn os_jupyter_data_dir() -> PathBuf {
    if let Ok(app_data) = env::var("APPDATA") {
        PathBuf::from(app_data).join("jupyter")
    } else {
        unimplemented!()
    }
}

fn find_connection_file<S>(glob_pattern: S, paths: Option<Vec<PathBuf>>) -> Option<PathBuf>
where
    S: Into<String>,
{
    let paths = paths.unwrap_or_else(|| {
        vec![
            current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            jupyter_runtime_dir(),
        ]
    });
    trace!("connection file paths to search: {:?}", paths);

    let glob_pattern = glob_pattern.into();

    for path in paths.into_iter() {
        let pattern = path.join(&glob_pattern);
        trace!("glob pattern: {:?}", pattern);
        let matches = glob(pattern.to_str().unwrap()).unwrap();
        let mut matches: Vec<PathBuf> = matches.map(|m| m.unwrap()).collect();
        trace!("matches: {:?}", matches);
        if !matches.is_empty() {
            matches.sort_by_key(|p| {
                let metadata = fs::metadata(p).unwrap();
                metadata.modified().unwrap()
            });
            trace!("sorted matches: {:#?}", matches);
            return Some(matches.last().unwrap().clone());
        }
    }
    None
}

pub fn get_jupyter_runtime_dir() -> Result<PathBuf, Infallible> {
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
