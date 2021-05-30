// mod repository;
// mod moonread;
// mod package;
// mod utils;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::collections::HashMap;

// This function parses the arguments given by the user
#[pyfunction]
fn parse_args(args: HashMap<String, bool>, package: String) -> PyResult<bool> {
    let key = String::from("ask");
    Ok(args[&key])
    
    
}

// moon module.
#[pymodule]
fn moon(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_args, m)?).unwrap();
    Ok(())

}





