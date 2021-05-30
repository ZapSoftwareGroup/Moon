
pub mod read {
    
    use std::fs::File;
    use std::io::prelude::*;
    use std::io;
    use serde_json::{Result as jsResult, Value};
    

    fn index_to_serde() -> jsResult<Value> {
        let INDEX_PATH: String = String::from("/home/abhinav/Code/moon/moonRepository/moonIndex.json");
        let file = read_to_string(&INDEX_PATH);

        match file {
            Ok(v) => Ok(serde_json::from_str(&v[..])?),
            _ => panic!("File not found.")
        }
    }

    fn read_to_string(path: &String) -> std::io::Result<String> {
        let mut file = File::open(path as &str)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents);
        Ok(contents)
    }
    
    fn find_category(package: &String) -> (&str, &str) {
        let bytes = package.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b'/' {
                return (&package[0..i], &package[i..]);
            }
        }
    }

    fn find_package_file(package: String) -> Result<String, io::Error> {
        let index = index_to_serde()?;

    
}
