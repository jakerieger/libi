use std::process::Command;
use std::path::PathBuf;
use colored::Colorize;

use crate::utils::*;

pub fn parse_add(args: &Vec<String>, cache_dir: &mut PathBuf) {
    let repo = args.get(1).unwrap();
    let repo_name = repo.split('/').last().unwrap().rsplit_once('.').unwrap().0;

    let mut repo_dir = PathBuf::new();
    repo_dir.push(&cache_dir);
    repo_dir.push(&repo_name);

    print_status(format!("Cloning repo: {}", &repo).as_str());
    Command::new("git")
        .arg("-C")
        .arg(&cache_dir.as_os_str().to_str().unwrap())
        .arg("clone")
        .arg(&repo)
        .output()
        .expect("Failed to clone git repo");

    // Build type specified
    if args.len() == 3 {
        let build_type = args.get(2).unwrap().as_str();

        match build_type {
            "static" => {
                print_status(
                    format!("Using build type: {}", "static").bold().to_string().as_str()
                );

                build_lib::build(build_lib::BuildType::BuildType_Static, &repo_dir);
            },
            "dynamic" => {
                print_status(
                    format!("Using build type: {}", "dynamic").bold().to_string().as_str()
                );

                build_lib::build(build_lib::BuildType::BuildType_Dynamic, &repo_dir);
            },
            "header" => {
                print_status(
                    format!("Using build type: {}", "header").bold().to_string().as_str()
                );

                build_lib::build(build_lib::BuildType::BuildType_HeaderOnly, &repo_dir);
            },
            _ => {
                print_error(
                    format!("Invalid build type specifier: '{}'", &build_type).as_str(), 
                    ErrorLevel::ErrorLevel_Fatal, 
                    false
                );
                return
            }
        }
    } else { // Use default 'static'
        print_error(
            format!("No build type provided. Using default: {}", "static").bold().to_string().as_str(), 
            ErrorLevel::ErrorLevel_Warn, 
            false
        );
    }
}