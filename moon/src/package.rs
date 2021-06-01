use std::{fs::File, io::{Read}};
use serde_json::{Result as SResult, Value};
use std::fs::metadata;

pub struct Software {
    pub name: String,
    pub binaries: Vec<String>,
    pub url: String,
    commands: Vec<String>
}

pub fn find_version_from_index((prefix, path): (String, String), version: String) -> SResult<String> {
    // load index file to heap
    let mut package = File::open(&path).expect("Couldn't find file 404 beep boop");
    let mut file_string = String::new();
    package.read_to_string(&mut file_string).unwrap();
    // Use Serde to find the real version number.
    let v: Value = serde_json::from_str(file_string.as_str()).expect("Couldn't load file into serde.");
    let latest = v["version"][0].as_str().unwrap();
    if version.as_str() == "latest" {
        let mut prefix = prefix.clone();
        prefix.push_str(latest);
        return Ok(prefix);
    } else {
        let mut prefix = prefix.clone();
        prefix.push_str(version.as_str());
        prefix.push_str(".json");
        let md = metadata(prefix.as_str()).expect("I couldn't find that specific version anywhere. Can you check if it exists? Cheers, moon");
        if md.is_file() {
            return Ok(prefix);
        } else {
            panic!("No such version exists in the repo");
        }
    }
}

// impl Software {
    
//    pub fn from_file(path: &str) -> Software {

        // Import file into a String
 //       let mut package = File::open(path).expect("File not found 404 error beep boop beep boop");
 //       let mut file_string = String::new();
 //       package.read_to_string(&mut file_string);


 //   }
//}
