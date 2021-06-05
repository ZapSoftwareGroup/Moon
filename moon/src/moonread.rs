pub mod read {

    use std::{
        fs, io,
        path::{Path, PathBuf},
    };
    use walkdir::WalkDir;

    pub fn find_repos<T>(path: T) -> io::Result<Vec<PathBuf>>
    where
        T: AsRef<Path>,
    {
        fs::read_dir(path)?
            .into_iter()
            .map(|x| x.map(|entry| entry.path()))
            .collect()
    }

    pub fn find_file_in_dir(path: PathBuf, package: &str) -> Result<(String, String), io::Error> {
        for entry in WalkDir::new(path) {
            let entry = entry?;
            let mut test_package: String = String::new();
            test_package.push_str("/");
            test_package.push_str(package);
            test_package.push_str(".json");
            if (entry.path().display().to_string().contains(&test_package))
                && (entry.path().is_file())
            {
                let orig = entry.path().to_str().unwrap();
                let upper_bounds = orig.find(&package).unwrap_or(0);
                let lower_bounds = 0;
                let mut prefix_path = orig[lower_bounds..upper_bounds].to_string();
                prefix_path.push_str(package);
                prefix_path.push_str("/");
                return Ok((prefix_path, entry.path().display().to_string()));
            }
        }
        Ok((String::new(), String::new()))
    }
}
