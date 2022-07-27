#![allow(dead_code)]

use std::env::args;
use std::path::Path;

use colored::Colorize;

mod utils;
mod parser;
mod config;

struct Flags {
    compiler: String,
    clean_cache_on_install: u16
}

fn main() {
    let data_path = utils::get_libi_data_path();
    if !Path::exists(&data_path.as_path()) {
        utils::create_libi_data_dirs(&data_path);
    }

    if !Path::exists(&utils::get_config_path().as_path()) {
        utils::create_config_file().unwrap();
    }

    let config = utils::get_config();

    let mut argv: Vec<String> = args().collect();
    argv.remove(0); // Remove first arg since it's just the executable name

    if argv.len() < 1 {
        utils::print_usage();
        return
    }

    if argv.len() > 1 {
        // Check if current working directory is a C++ project
        // and abort if not
        if !Path::new("./CMakeLists.txt").exists() {
            utils::print_error(
                "Current directory is not a C++ project",
                utils::ErrorLevel::ErrorLevel_Fatal,
                false
            );
            return
        }
    }

    let cmd = argv.get(0).unwrap().as_str();
    match cmd {
        "add" => {
            if argv.len() < 2 {
                utils::print_error(
                    "No package repo provided", 
                    utils::ErrorLevel::ErrorLevel_Fatal, 
                    false
                );

                utils::print_command_add_usage();

                return
            }

            // Validate that a URL was passed
            if !&argv.get(1).unwrap().contains(".git") {
                utils::print_error(
                    "The provided URL is not recognized as a git repository", 
                    utils::ErrorLevel::ErrorLevel_Error, 
                    true
                );
                return
            }

            parser::parse_add(&argv, &mut utils::get_cache_dir());
        },
        "config" => {},
        "help" => {},
        "init" => {},
        "remove" => {},
        "freeze" => {},
        "rebuild" => {},
        "version" => {},
        _ => {
            utils::print_error(
                format!("Invalid command provided: '{}'", cmd).as_str(), 
                utils::ErrorLevel::ErrorLevel_Fatal, 
                true
            );

            return
        }
    }
}
