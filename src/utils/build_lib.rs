use crate::utils::print_status;
use std::fs;
use std::io::Error;
use std::path::PathBuf;

use crate::parser::cmake::CMakeConfig;
use crate::utils::config::LibiConfig;

pub fn build(repo_dir: &PathBuf, config: &LibiConfig) -> Result<(), Error> {
    print_status("Building library...");

    let mut repo_dir_mut = repo_dir.clone();
    repo_dir_mut.push("build");
    let _: Result<(), Error> = fs::create_dir(&repo_dir_mut);

    let cmake_config = CMakeConfig::new_from_libi_conf(
        config,
        repo_dir_mut,
        repo_dir.to_owned()
    );

    match cmake_config.config() {
        Ok(_) => {
            cmake_config.build();
            Ok(())
        },
        Err(e) => Err(e)
    }
}
