mod moonread;
mod package;
// mod utils;

use moonread::read;
use package::find_version_from_index;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::collections::HashMap;
use std::path::PathBuf;
use dirs;


// This function parses the arguments given by the user
#[pyfunction]
fn parse_args(bool_args: HashMap<String, bool>, string_args: HashMap<String, String>, package: String) -> PyResult<String> {
    let mut moon_path = match dirs::home_dir() {
        Some(n) => n.display().to_string(),
        None => String::new()
    
    };

    moon_path.push_str("/.moon/Repos/");

    let key = String::from("version");
    let repos = read::find_repos(moon_path)?;

    let mut path: PathBuf = PathBuf::new();



    for repo in repos {
        path = repo;
    }
    let (prefix, index) = read::find_file_in_dir(path, &package)?;
    Ok(find_version_from_index((prefix, index), string_args[&key].to_string()).unwrap())
    
    
}

// moon module.
#[pymodule]
fn moon(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_args, m)?).unwrap();
    Ok(())

}





