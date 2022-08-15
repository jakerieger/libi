#![allow(non_camel_case_types)]
#![allow(dead_code)]

use crate::config;
use colored::Colorize;
use platform_dirs::AppDirs;
use std::fs;
use std::io::Error;
use std::path::PathBuf;

pub mod build_lib;
pub mod usage;

pub fn get_libi_data_path() -> PathBuf {
    let app_dir = AppDirs::new(Some("libi"), true).unwrap();
    return app_dir.data_dir;
}

pub fn create_libi_data_dirs(libi_dir: &PathBuf) {
    let _: Result<(), Error> = fs::create_dir(libi_dir);

    // <libi_dir>/bin
    let mut bin_dir = libi_dir.clone();
    bin_dir.push("bin");
    let _: Result<(), Error> = fs::create_dir(&bin_dir);

    // <libi_dir>/include
    let mut inc_dir = libi_dir.clone();
    inc_dir.push("include");
    let _: Result<(), Error> = fs::create_dir(inc_dir);

    // <libi_dir>/cache
    let mut cache_dir = libi_dir.clone();
    cache_dir.push("cache");
    let _: Result<(), Error> = fs::create_dir(cache_dir);

    // <libi_dir>/bin/debug
    let mut bin_dir_dbg = bin_dir.clone();
    bin_dir_dbg.push("debug");
    let _: Result<(), Error> = fs::create_dir(&bin_dir_dbg);

    // <libi_dir>/bin/debug/dynamic
    let mut bin_dir_dbg_dyn = bin_dir_dbg.clone();
    bin_dir_dbg_dyn.push("dynamic");
    let _: Result<(), Error> = fs::create_dir(&bin_dir_dbg_dyn);

    // <libi_dir>/bin/debug/static
    let mut bin_dir_dbg_static = bin_dir_dbg.clone();
    bin_dir_dbg_static.push("static");
    let _: Result<(), Error> = fs::create_dir(&bin_dir_dbg_static);

    // <libi_dir>/bin/release
    let mut bin_dir_rel = bin_dir.clone();
    bin_dir_rel.push("release");
    let _: Result<(), Error> = fs::create_dir(&bin_dir_rel);

    // <libi_dir>/bin/release/dynamic
    let mut bin_dir_rel_dyn = bin_dir_rel.clone();
    bin_dir_rel_dyn.push("dynamic");
    let _: Result<(), Error> = fs::create_dir(&bin_dir_rel_dyn);

    // <libi_dir>/bin/release/static
    let mut bin_dir_rel_static = bin_dir_rel.clone();
    bin_dir_rel_static.push("static");
    let _: Result<(), Error> = fs::create_dir(&bin_dir_rel_static);
}

pub fn get_cache_dir() -> PathBuf {
    let mut cache_dir = get_libi_data_path().clone();
    cache_dir.push("cache");
    return cache_dir;
}

pub fn get_config_path() -> PathBuf {
    let config_path = AppDirs::new(Some("libi"), true).unwrap();
    let config_file_path = config_path.config_dir.join("libi.json");
    return config_file_path;
}

pub fn get_config() -> config::LibiConfig {
    let config_path = AppDirs::new(Some("libi"), true).unwrap();
    let config_file_path = config_path.config_dir.join("libi.json");
    let config_str = fs::read_to_string(config_file_path).unwrap();
    let config = serde_json::from_str::<config::LibiConfig>(&config_str).unwrap();
    return config;
}

pub enum ErrorLevel {
    ErrorLevel_Info,
    ErrorLevel_Warn,
    ErrorLevel_Error,
    ErrorLevel_Fatal,
}

pub fn print_usage() {
    let usage_str = r#"
Libi - The common sense package manager for modern C++ projects.

Usage:
    libi [command] [options]

Available Commands:
    add       Install a package to the current project directory and build with the following type
    config    Set global configuration options for Libi
    freeze    Export dependencies list to .libs file
    init      Create a new Libi-configured C++ project
    help      Show help for Libi
    new       Create a new C++ CMake project
    rebuild   Rebuild all installed packages in project
    remove    Remove a package from the current project
    version   Print the currently installed version of Libi

Use "libi [command] --help" for more information about a command.
    "#;

    println!("{}", usage_str);
}

pub fn print_command_add_usage() {
    let usage_str = r#"
Usage:
    libi add <repo>

Example:
    libi add https://github.com/glfw/glfw.git static

Parameters:
    repo         URL of the git repo for the library to add

    "#;

    println!("{}", usage_str);
}

pub fn print_error(msg: &str, level: ErrorLevel, print_help: bool) {
    match level {
        ErrorLevel::ErrorLevel_Info => {
            println!(
                "{}  - {}",
                format!("{}", "[INFO]").bold().bright_blue(),
                msg
            );
        }
        ErrorLevel::ErrorLevel_Warn => {
            println!("{}  - {}", format!("{}", "[WARN]").bold().yellow(), msg);
        }
        ErrorLevel::ErrorLevel_Error => {
            println!("{} - {}", format!("{}", "[ERROR]").bold().red(), msg);
        }
        ErrorLevel::ErrorLevel_Fatal => {
            println!("{} - {}", format!("{}", "[FATAL]").bold().bright_red(), msg);
        }
    }

    if print_help {
        print_usage();
    }
}

pub fn print_status(msg: &str) {
    println!("{}  - {}", format!("{}", "[Libi]").bold().cyan(), msg);
}
