mod moonread;
mod package;
// mod utils;

use moonread::read;
use package::find_version_from_index;

use dirs;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::collections::HashMap;
use std::path::PathBuf;

// This function parses the arguments given by the user
#[pyfunction]
fn parse_args(
    bool_args: HashMap<String, bool>,
    string_args: HashMap<String, String>,
    package: String,
) -> PyResult<String> {
    let mut moon_path = match dirs::home_dir() {
        Some(n) => n.display().to_string(),
        None => String::new(),
    };

    moon_path.push_str("/.moon/Repos/");

    let key = String::from("version");
    let repos = read::find_repos(moon_path)?;

    let mut path: PathBuf = PathBuf::new();

    for repo in repos {
        path = repo;
    }
    let (prefix, index) = read::find_file_in_dir(path, &package)?;
    let s = find_version_from_index((prefix, index), string_args[&key].to_string());
    Ok(match s {
        Ok(v) => v,
        Err(e) => String::from("404 file not found"),
    })
}

// moon module.
#[pymodule]
fn moon(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_args, m)?).unwrap();
    Ok(())
}

fn hello(s: &str) {
    println!("Hello");
    for char in s.as_bytes() {
        println!("{}", char);
    }
}
