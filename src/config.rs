use crate::errors::{self, Error};
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use serde_yaml::{from_str, to_string};
use std::fs::read_to_string;
use std::{
    fs::{self, create_dir_all, write},
    path::PathBuf,
};

pub(crate) const CLUSTER_DIR_NAME: &str = "clusters";
const DATA: &str = "data";
const BUILD_DIR_NAME: &str = "clusters";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub cluster_name: String,
    #[serde(rename(serialize = "namespace", deserialize = "namespace"))]
    pub name_space: String,
    pub config_file_name: String,
    pub key: String,
    pub value: String,
    pub base_path: Option<String>,
}

pub fn get_configfrom_file() -> Result<Vec<Config>, Error> {
    let content = fs::read_to_string("./config.json")?;
    let config: Vec<Config> = serde_json::from_str(&content)?;
    return Ok(config);
}

impl Config {
    pub fn is_paths_matches(&self, path: &PathBuf) -> bool {
        let config_path_buf = PathBuf::from(format!(
            "clusters/{}/{}/{}",
            self.cluster_name, self.name_space, self.config_file_name
        ));
        return config_path_buf.as_path() == path.as_path();
    }
    pub fn create_build_dir(&self, dir_name: &str) -> Result<(), errors::Error> {
        create_dir_all(
            format!("./{}/{}/{}", dir_name, self.cluster_name, self.name_space).as_str(),
        )?;
        Ok(())
    }
    pub fn write_build_dir(&self, dir_name: &str, content: String) -> Result<(), errors::Error> {
        write(
            format!(
                "./{}/{}/{}/{}",
                dir_name, self.cluster_name, self.name_space, self.config_file_name
            ),
            content,
        )?;
        Ok(())
    }
    pub fn update_yaml(&self, path: &PathBuf) -> Result<(), errors::Error> {
        let config_to_string = read_to_string(path)?;
        let mut config_yaml_string = from_str::<Value>(config_to_string.as_str())?;
        config_yaml_string[DATA][self.key.as_str()] = Value::String(self.value.to_string());
        let to_yaml = to_string(&config_yaml_string)?;
        self.create_build_dir(BUILD_DIR_NAME)?;
        self.write_build_dir(BUILD_DIR_NAME, to_yaml)?;
        Ok(())
    }
}
