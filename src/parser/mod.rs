use std::process::Command;
use std::path::PathBuf;

use crate::utils::print_status;

pub fn parse_add(args: &Vec<String>, cache_dir: &mut PathBuf) {
    let repo = args.get(1).unwrap();
    let repo_name = repo.as_str().rsplit(pat);

    print_status(format!("Cloning repo: {}", &repo).as_str());
    Command::new("git")
        .arg("clone")
        .arg(&repo)
        .arg(&cache_dir.as_os_str().to_str().unwrap())
        .output()
        .expect("Failed to clone git repo");
}