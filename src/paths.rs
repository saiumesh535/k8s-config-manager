use crate::errors::Error;
use std::{fs::read_dir, path::PathBuf};

pub fn get_all_paths(dir_path: &PathBuf, paths: &mut Vec<PathBuf>) -> Result<(), Error> {
    let dir = read_dir(dir_path)?;
    for curr_path in dir {
        let dir_entry = curr_path?;
        if dir_entry.path().is_dir() {
            get_all_paths(&dir_entry.path(), paths)?;
        } else {
            paths.push(dir_entry.path());
        }
    }
    Ok(())
}
