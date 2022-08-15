use console::{style, Emoji};
use std::path::PathBuf;
use std::process::Command;

use crate::config::LibiConfig;
use crate::utils::{*, self};

pub mod cmake;

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
static CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
static PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

pub fn parse_add(args: &Vec<String>, cache_dir: &mut PathBuf, config: &LibiConfig) {
    let repo = args.get(1).unwrap();
    let repo_name = repo.split('/').last().unwrap().rsplit_once('.').unwrap().0;

    let mut repo_dir = PathBuf::new();
    repo_dir.push(&cache_dir);
    repo_dir.push(&repo_name);

    println!(
        "{} {}Resolving repository...",
        style("[1/4]").bold().dim(),
        LOOKING_GLASS
    );

    println!(
        "{} {}Cloning repository...",
        style("[2/4]").bold().dim(),
        TRUCK
    );

    Command::new("git")
        .arg("-C")
        .arg(&cache_dir.as_os_str().to_str().unwrap())
        .arg("clone")
        .arg(&repo)
        .output()
        .expect("Failed to clone git repo");

    println!(
        "{} {}Building library...",
        style("[3/4]").bold().dim(),
        PAPER
    );

    match utils::build_lib::build(&repo_dir, config) {
        Ok(_) => {
            println!("{} {}Linking library...", style("[4/4]").bold().dim(), CLIP);
        },
        Err(e) => {
            panic!("{}", e.to_string());
        },
    }
}
