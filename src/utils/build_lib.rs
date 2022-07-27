use std::io::Error;
use std::path::PathBuf;
use std::process::Command;
use std::fs;
use crate::utils::print_status;

pub enum BuildType {
    BuildType_Static,
    BuildType_Dynamic,
    BuildType_HeaderOnly,
}

pub fn build(build_type: BuildType, repo_dir: &PathBuf) {
    print_status("Building library...");

    let mut repo_dir_mut = repo_dir.clone();
    repo_dir_mut.push("build");
    let _ : Result<(), Error> = fs::create_dir(&repo_dir_mut);

    match build_type {
        BuildType::BuildType_Static => {
            Command::new("cmake")
                .arg("-B")
                .arg(&repo_dir_mut.to_str().unwrap())
                .arg("-S")
                .arg(repo_dir.to_str().unwrap())
                .output()
                .expect("CMake project configuration failed");

            Command::new("cmake")
                .arg(&repo_dir_mut.to_str().unwrap())
                .arg("--build")
                .output()
                .expect("CMake build command failed");
        },
        BuildType::BuildType_Dynamic => {},
        BuildType::BuildType_HeaderOnly => {},
    }
}