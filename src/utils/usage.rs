pub struct CmdParameter {
    pub name: String,
    pub desc: String,
}

pub struct CmdUsage {
    pub usage: String,
    pub example: String,
    pub parameters: Vec<CmdParameter>,
}

pub enum Cmd {
    add,
    config,
    freeze,
    init,
    help,
    new,
    rebuild,
    remove,
    version,
}

impl CmdUsage {
    pub fn get_cmd_usage(cmd: Cmd) -> CmdUsage {
        match cmd {
            Cmd::add => {
                let repo_param = CmdParameter {
                    name: String::from("repo"),
                    desc: String::from("URL of the git repo for the library to add"),
                };

                let cmd_usage = CmdUsage {
                    usage: String::from("libi add <repo> <build_type>"),
                    example: String::from("libi add https://github.com/glfw/glfw.git static"),
                    parameters: vec![repo_param],
                };

                return cmd_usage;
            }
            Cmd::config => {
                let cmd_usage = CmdUsage {
                    usage: String::from("libi add <repo> <build_type>"),
                    example: String::from("libi add https://github.com/glfw/glfw.git static"),
                    parameters: vec![],
                };

                return cmd_usage;
            }
            Cmd::freeze => {
                let cmd_usage = CmdUsage {
                    usage: String::from("libi add <repo> <build_type>"),
                    example: String::from("libi add https://github.com/glfw/glfw.git static"),
                    parameters: vec![],
                };

                return cmd_usage;
            }
            Cmd::help => {
                let cmd_usage = CmdUsage {
                    usage: String::from("libi add <repo> <build_type>"),
                    example: String::from("libi add https://github.com/glfw/glfw.git static"),
                    parameters: vec![],
                };

                return cmd_usage;
            }
            Cmd::init => {
                let cmd_usage = CmdUsage {
                    usage: String::from("libi add <repo> <build_type>"),
                    example: String::from("libi add https://github.com/glfw/glfw.git static"),
                    parameters: vec![],
                };

                return cmd_usage;
            }
            Cmd::new => {
                let cmd_usage = CmdUsage {
                    usage: String::from("libi add <repo> <build_type>"),
                    example: String::from("libi add https://github.com/glfw/glfw.git static"),
                    parameters: vec![],
                };

                return cmd_usage;
            }
            Cmd::rebuild => {
                let cmd_usage = CmdUsage {
                    usage: String::from("libi add <repo> <build_type>"),
                    example: String::from("libi add https://github.com/glfw/glfw.git static"),
                    parameters: vec![],
                };

                return cmd_usage;
            }
            Cmd::remove => {
                let cmd_usage = CmdUsage {
                    usage: String::from("libi add <repo> <build_type>"),
                    example: String::from("libi add https://github.com/glfw/glfw.git static"),
                    parameters: vec![],
                };

                return cmd_usage;
            }
            Cmd::version => {
                let cmd_usage = CmdUsage {
                    usage: String::from("libi add <repo> <build_type>"),
                    example: String::from("libi add https://github.com/glfw/glfw.git static"),
                    parameters: vec![],
                };

                return cmd_usage;
            }
        }
    }
}

const PARAM_SPACING: u16 = 12;

pub fn print_cmd_usage(cmd: Cmd) {
    let usage = CmdUsage::get_cmd_usage(cmd);

    let mut output_str = String::from("Usage:");
    output_str.push_str(format!("\n    {}", usage.usage).as_str());
    output_str.push_str("\n\nExample:");
    output_str.push_str(format!("\n    {}", usage.example).as_str());
    output_str.push_str("\n\nParameters:");
    for param in usage.parameters {
        output_str.push_str("\n    ");
        output_str.push_str(&param.name);
        let spaces_to_add: u16 = PARAM_SPACING - (param.name.len() as u16);
        for _ in 0..spaces_to_add {
            output_str.push(' ');
        }
        output_str.push_str(&param.desc);
    }

    println!("\n{}\n", output_str);
}
