use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LibiConfig {
    pub generator: String,
    pub c_compiler: String,
    pub cxx_compiler: String,
    pub clear_cache_on_build: bool,
}

impl LibiConfig {
    pub fn read_from_project() -> LibiConfig {
        let conf_str = fs::read_to_string(Path::new("./libi.conf.json")).unwrap();
        return serde_json::from_str::<LibiConfig>(&conf_str).unwrap();
    }
}