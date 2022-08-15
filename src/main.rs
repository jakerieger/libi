#![allow(dead_code)]

use regex::Regex;
use std::path::Path;
use std::{env::args, fs};

mod config;
mod parser;
mod utils;

use utils::{usage, usage::Cmd};

fn main() {
    let data_path = utils::get_libi_data_path();
    // Create directories used by Libi if missing
    if !Path::exists(&data_path.as_path()) {
        utils::create_libi_data_dirs(&data_path);
    }
    // --------------------------------------------

    let config = config::LibiConfig::read_from_project();

    // Check for libi.conf.json in project root
    if !Path::new("./libi.conf.json").exists() {
        utils::print_error(
            "This project has no Libi configuration. Create one now? (Y|n)",
            utils::ErrorLevel::ErrorLevel_Info,
            false,
        );
    }

    let mut argv: Vec<String> = args().collect();
    argv.remove(0); // Remove first arg since it's just the executable name

    if argv.len() < 1 {
        utils::print_usage();
        return;
    }

    if argv.len() > 1 {
        // Check if current working directory is a C++ project
        // and abort if not
        if !Path::new("./CMakeLists.txt").exists() {
            utils::print_error(
                "Current directory is not a CMake C++ project",
                utils::ErrorLevel::ErrorLevel_Fatal,
                false,
            );
            return;
        } else {
            // Update CMakeLists.txt if missing Libi config vars
            let mut made_changes = false;
            let cmake_lists_path = Path::new("./CMakeLists.txt");
            let cmake_lists_content = fs::read_to_string(cmake_lists_path).unwrap();
            let mut lines: Vec<&str> = cmake_lists_content.split("\n").into_iter().collect();

            let libi_root = format!(
                "\nset(LIBI_ROOT \"{}\")",
                utils::get_libi_data_path().as_path().to_str().unwrap()
            );
            let libi_root_str = libi_root.clone();

            let re = Regex::new(r"(?m)^set\(LIBI_ROOT").unwrap();
            if !re.is_match(cmake_lists_content.as_str()) {
                lines.insert(1, libi_root_str.as_str());
            }

            let re = Regex::new(r"(?m)^include_directories").unwrap();
            if !re.is_match(cmake_lists_content.as_str()) {
                let mut index = 0;
                for line in lines.clone() {
                    if line.starts_with("project(") {
                        lines.insert(
                            index + 1,
                            r#"
include_directories(
    ${LIBI_ROOT}/include
)"#,
                        );
                        made_changes = true;
                        break;
                    }
                    index += 1;
                }
                let re = Regex::new(r"(?m)^ *\$\{LIBI_ROOT\}/include").unwrap();
                if !re.is_match(cmake_lists_content.as_str()) {
                    let mut insert_index = 0usize;
                    let mut found_include_dirs_loc = false;
                    for line in lines.clone() {
                        if line.starts_with("include_directories(") {
                            found_include_dirs_loc = true;
                        }

                        if line.starts_with(")") && found_include_dirs_loc {
                            break;
                        }

                        insert_index += 1;
                    }

                    lines.insert(insert_index, "    ${LIBI_ROOT}/include");
                    made_changes = true;
                }
            }

            // Write changes back to CMakeLists.txt
            if made_changes {
                let output = lines.join("\n");
                let _ = fs::write(cmake_lists_path, output);
            }
        }
    }

    let cmd = argv.get(0).unwrap().as_str();
    match cmd {
        "add" => {
            if argv.len() < 2 {
                utils::print_error(
                    "No package repo provided",
                    utils::ErrorLevel::ErrorLevel_Fatal,
                    false,
                );

                usage::print_cmd_usage(Cmd::add);

                return;
            }

            // Validate that a URL was passed
            if !&argv.get(1).unwrap().contains(".git") {
                utils::print_error(
                    "The provided URL is not recognized as a git repository",
                    utils::ErrorLevel::ErrorLevel_Error,
                    true,
                );
                return;
            }

            parser::parse_add(&argv, &mut utils::get_cache_dir(), &config);
        }
        "config" => {}
        "help" => {}
        "init" => {}
        "remove" => {}
        "freeze" => {}
        "rebuild" => {}
        "version" => {}
        _ => {
            utils::print_error(
                format!("Invalid command provided: '{}'", cmd).as_str(),
                utils::ErrorLevel::ErrorLevel_Fatal,
                true,
            );

            return;
        }
    }
}
