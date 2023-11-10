use crate::filesystem::file::{File, FileContent};
use crate::filesystem::FileSystem;
use crate::Player;

pub(crate) enum Command {
    Cd { path: String },         // change "directories"
    Ls,                          // list contents of current "directory"
    Pwd,                         // display the current "directory"
    Clear,                       // clear the screen
    Exit,                        // exit the program
    Cat { path: String },        // display the contents of a file
    Run { args: String },        // "run" a "program"
    Help,                        // display help
    Debug { args: Vec<String> }, // modify variables during runtime
    Invalid,                     // invalid command
}

impl Command {
    pub fn parse<S: Into<String>>(raw: S, allow_debug: bool) -> Self {
        let input = &raw.into();
        let mut tokens: Vec<&str> = input.split_whitespace().collect();

        if tokens.is_empty() {
            return Command::Invalid;
        }

        let cmd = tokens.remove(0);

        match cmd {
            "cd" if !tokens.is_empty() => {
                Command::Cd {
                    path: tokens.join(" "),
                }
            }
            "ls" if tokens.is_empty() => Command::Ls,
            "pwd" if tokens.is_empty() => Command::Pwd,
            "clear" if tokens.is_empty() => Command::Clear,
            "exit" if tokens.is_empty() => Command::Exit,
            "cat" if !tokens.is_empty() => {
                Command::Cat {
                    path: tokens.join(" "),
                }
            }
            "help" => {
                Command::Help
            }
            "debug" if allow_debug => {
                Command::Debug {
                    args: tokens.iter().map(|s| s.to_string()).collect(),
                }
            }
            _ if cmd.starts_with("./") => {
                let mut executable_name = cmd.to_string().replace("./", "");

                for arg in tokens.iter() {
                    executable_name.push(' ');
                    executable_name.push_str(arg);
                }

                Command::Run {
                    args: executable_name
                }
            }
            _ => Command::Invalid,
        }
    }

    pub fn execute(&self, fs: &mut FileSystem, ps: &mut Player) -> Result<bool, String> {
        match self {
            Command::Help => {
                println!("Commands:\
                \n  cd <path>                    - change directories\
                \n  ls                           - list contents of current directory\
                \n  pwd                          - display the current directory\
                \n  clear                        - clear the screen\
                \n  cat <path>                   - display the contents of a file\
                \n  exit                         - exit the program\
                \n  help                         - display this help message\
                \nTo run an EXEC file, type ./<program name>")
            }
            Command::Cd { path } => {
                let (parsed, remainder) = FileSystem::parse_path_out_of_string(path)?;
                if remainder.is_some() {
                    println!("Invalid path.");
                    return Ok(false);
                }
                fs.cd(parsed)?
            }
            Command::Ls => {
                fs.ls()
            }
            Command::Pwd => {
                println!("{}", fs.get_pwd());
            }
            Command::Clear => {
                println!("This command is not currently implemented.");
            }
            Command::Cat { path } => {
                let (parsed, remainder) = FileSystem::parse_path_out_of_string(path)?;
                if remainder.is_some() {
                    println!("Invalid path.");
                    return Ok(false);
                }
                let (name, text) = fs.cat(parsed)?;
                println!("{}:\n{}", name, text)
            }
            Command::Run { args } => {
                let (parsed, remainder) = FileSystem::parse_path_out_of_string(args)?;
                let mut args = Vec::new();
                if let Some(remainder) = remainder {
                    args = remainder.split(' ').map(|s| s.to_string()).collect();
                }
                fs.run(parsed, ps, args)?;
            }
            Command::Exit => {
                return Ok(true);
            }
            Command::Debug { args } => {
                if args.is_empty() {
                    println!("Not enough arguments.");
                    return Ok(false);
                }

                let subcmd = args[0].as_str();
                match subcmd {
                    "help" | "?" => {
                        println!("Debug commands:\
                        \n  ps <var> <value>             - edit player stats\
                        \n  edit <file path> <new text>  - edit a text file's content\
                        \n  touch <file path> <text?>    - create a new text file with optional text\
                        \n  rm <file, dir> <path>        - remove a file or directory");
                    }
                    "ps" => { // format: debug ps <var> <value>
                        if args.len() != 3 {
                            println!("Not enough arguments.");
                            return Ok(false);
                        }

                        let var = args[1].as_str();
                        let value = args[2].as_str();
                        // parse value into a u32
                        let value = match value.parse::<u32>() {
                            Ok(v) => v,
                            Err(_) => {
                                println!("Invalid value.");
                                return Ok(false);
                            }
                        };
                        match var {
                            "health" => {
                                ps.health = value;
                            }
                            "score" => {
                                ps.score = value;
                            }
                            "bytes" => {
                                ps.bytes = value;
                            }
                            _ => {
                                println!("Unknown variable.");
                                return Ok(false);
                            }
                        }
                    }
                    "mkdir" => {
                        if args.len() < 2 {
                            println!("Not enough arguments.");
                            return Ok(false);
                        }
                        let (parsed, remainder) = FileSystem::parse_path_out_of_string(&args[1..].join(" "))?;
                        if remainder.is_some() {
                            println!("Invalid path.");
                            return Ok(false);
                        }
                        fs.mkdir(parsed.as_str())?
                    }
                    "edit" => { // format: debug edit <file path> <new text>
                        if args.len() < 3 {
                            println!("Not enough arguments.");
                            return Ok(false);
                        }

                        // parse file path from args[1..], allowing for spaces if surrounded by ''
                        let arguments = args[1..].join(" ");
                        let (path, remainder) = FileSystem::parse_path_out_of_string(&arguments)?;
                        let Some(text) = remainder else {
                            println!("Invalid path.");
                            return Ok(false);
                        };
                        let path = fs.current_directory.clone() + path.as_str();

                        fs.edit_file(path, FileContent::Text(text))?;
                    }
                    "touch" => { // format: debug touch <file path> <text?>
                        if args.len() < 2 {
                            println!("Not enough arguments.");
                            return Ok(false);
                        }

                        let (path, remainder) = FileSystem::parse_path_out_of_string(&args[1..].join(" "))?;
                        let text = remainder.unwrap_or(String::default());

                        let Some((path, file)) = FileSystem::separate_file_and_parent(path) else {
                            println!("Invalid path.");
                            return Ok(false);
                        };
                        let path = fs.current_directory.clone() + path.as_str();

                        fs.touch(path, File {
                            name: file,
                            content: FileContent::Text(text),
                        });

                    }
                    "rm" => { // format: debug rm <file, dir> <path>
                        if args.len() < 3 {
                            println!("Not enough arguments.");
                            return Ok(false);
                        }

                        let subcmd = args[1].as_str();
                        let (path, remainder) = FileSystem::parse_path_out_of_string(&args[2..].join(" "))?;

                        let path = fs.current_directory.clone() + path.as_str();

                        if remainder.is_some() {
                            println!("Invalid arguments: {}", remainder.unwrap());
                            return Ok(false);
                        }

                        match subcmd {
                            "file" => {
                                fs.rm_file(path)?;
                            }
                            "dir" => {
                                fs.rm_dir(path)?;
                            }
                            _ => {
                                println!("Unknown subcommand for rm.");
                                return Ok(false);
                            }
                        }
                    }
                    _ => {
                        println!("Unknown subcommand.");
                        return Ok(false);
                    }
                }
            }
            Command::Invalid => {
                println!("Unknown command.");
            }
        }

        Ok(false)
    }
}