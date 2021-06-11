mod moonread;
mod package;
// mod utils;

use moonread::read;
use package::{Package, find_version_from_index};

use dirs;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::collections::HashMap;
use std::path::PathBuf;

// This function parses the arguments given by the user
#[pyfunction]
fn find_path(
    package: String,
    version: String
) -> PyResult<String> {
    let mut moon_path = match dirs::home_dir() {
        Some(n) => n.display().to_string(),
        None => String::new(),
    };

    moon_path.push_str("/.moon/Repos/");

    let repos = read::find_repos(moon_path)?;

    let mut path: PathBuf = PathBuf::new();

    for repo in repos {
        path = repo;
    }
    let (prefix, index) = read::find_file_in_dir(path, &package)?;
    let s = find_version_from_index((prefix, index), version);
    Ok(match s {
        Ok(v) => v,
        Err(_) => String::from("404 file not found"),
    })
}

// moon module.
#[pymodule]
fn moon(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(find_path, m)?).unwrap();
    m.add_class::<Package>()?;
    Ok(())
}

