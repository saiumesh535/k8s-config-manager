use std::path::PathBuf;

use crate::config::{get_configfrom_file, CLUSTER_DIR_NAME};
use crate::errors::Error;
use crate::paths::get_all_paths;

pub fn generate_cmp() -> Result<(), Error> {
    let configs = get_configfrom_file()?;
    let mut paths: Vec<PathBuf> = Vec::new();
    let cluster_dir_path = &PathBuf::new().join(CLUSTER_DIR_NAME);
    get_all_paths(cluster_dir_path, &mut paths)?;
    for path in &paths {
        let config = configs
            .iter()
            .find(|config| config.is_paths_matches(path));
        if let Some(config) = config {
            let is_path_matched = config.is_paths_matches(&path);
            if !is_path_matched {
                return Ok(());
            }
            // now read the content and update file with new key and value
            config.update_yaml(&path)?;
        }
    }
    Ok(())
}
