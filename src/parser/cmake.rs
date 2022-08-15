use std::{io::{Error, BufRead, BufReader}, process::{Command, Output, Stdio}, path::{Path, PathBuf}};

use crate::config::LibiConfig;

pub struct CMakeConfig {
    pub generator: String,
    pub c_compiler: String,
    pub cxx_compiler: String,
    pub build_dir: PathBuf,
    pub source_dir: PathBuf,
}

impl CMakeConfig {
    pub fn new_from_libi_conf(config: &LibiConfig, build_dir: PathBuf, source_dir: PathBuf) -> CMakeConfig {
        return CMakeConfig {
            generator: config.generator.clone(),
            c_compiler: config.c_compiler.clone(),
            cxx_compiler: config.cxx_compiler.clone(),
            build_dir,
            source_dir,
        }
    }

    pub fn config(&self) -> Result<Output, Error> {
        return Command::new("cmake")
            .arg("-B")
            .arg(self.build_dir.to_str().unwrap())
            .arg("-S")
            .arg(self.source_dir.to_str().unwrap())
            .arg("-G")
            .arg(&self.generator)
            .arg("-D")
            .arg(format!("CMAKE_C_COMPILER={}", self.c_compiler))
            .arg("-D")
            .arg(format!("CMAKE_CXX_COMPILER={}", self.cxx_compiler))
            .output();
    }

    pub fn build(&self) {
        let mut cmd = Command::new("cmake")
            .arg("--build")
            .arg(&self.build_dir)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        {
            let stdout = cmd.stdout.as_mut().unwrap();
            let stdout_reader = BufReader::new(stdout);
            let stdout_lines = stdout_reader.lines();

            for line in stdout_lines {
                println!("Read: {:?}", line);
            }
        }

        cmd.wait().unwrap();
    }
}