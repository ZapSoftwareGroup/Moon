use serde_json::{Value};
use std::{fs::{File, metadata}, io::{self, Read}, path::PathBuf};
use pyo3::prelude::*;

#[pyclass]
pub struct Package {
    name: String,
    source: String,
    file: String,
    path: String,
    format: String,
    version: String,
    binaries: Vec<String>,
    dependencies: Vec<String>,
    steps: Vec<String>
}

pub fn find_version_from_index(
    (prefix, path): (String, String),
    version: String,
) -> Result<String, io::Error> {
    // load index file to heap
    let mut package = File::open(&path)?;
    let mut file_string = String::new();
    package.read_to_string(&mut file_string).unwrap();
    // Use Serde to find the real version number.
    let v: Value =
        serde_json::from_str(file_string.as_str()).expect("Couldn't load file into serde.");
    let latest = v["version"][0].as_str().unwrap();
    if version.as_str() == "latest" {
        let mut prefix = prefix;
        prefix.push_str(latest);
        Ok(prefix)
    } else {
        let mut prefix = prefix;
        prefix.push_str(version.as_str());
        prefix.push_str(".json");
        let md = metadata(prefix.as_str()).expect("I couldn't find that specific version anywhere. Can you check if it exists? Cheers, moon");
        if md.is_file() {
            Ok(prefix)
        } else {
            panic!("No such version exists in the repo");
        }
    }
}
