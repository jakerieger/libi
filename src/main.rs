use std::env::args;
use std::path::Path;

mod utils;
mod parser;

fn main() {
    let data_path = utils::get_libi_data_path();
    if !Path::exists(&data_path.as_path()) {
        utils::create_libi_data_dirs(&data_path);
    }

    let mut argv: Vec<String> = args().collect();
    argv.remove(0); // Remove first arg since it's just the executable name

    let cmd = argv.get(0).unwrap().as_str();
    match cmd {
        "add" => {
            if argv.len() < 2 {
                utils::print_error(
                    "No package repo provided", 
                    utils::ErrorLevel::ErrorLevel_Fatal, 
                    true
                );

                return
            }

            parser::parse_add(&argv, &mut utils::get_cache_dir());
        },
        "config" => {},
        "help" => {},
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
