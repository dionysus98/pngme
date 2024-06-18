#[derive(Debug, Clone, Copy)]
pub enum Command {
    Encode,
    Decode,
    Remove,
    Print,
}

impl TryFrom<String> for Command {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "encode" => Ok(Self::Encode),
            "decode" => Ok(Self::Decode),
            "remove" => Ok(Self::Remove),
            "print" => Ok(Self::Print),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Args {
    pub command: Command,
    pub path: String,
    pub chunk: Option<String>,
    pub message: Option<String>,
}

pub fn parse_args(args: Vec<String>) -> Args {
    let args = &args[1..];

    if args.len() < 2 || args.len() > 4 {
        panic!("Invalid args");
    }

    let args = if let Ok(command) = Command::try_from(args[0].clone()) {
        match command {
            Command::Encode => {
                if args.len() >= 3 {
                    Args {
                        command,
                        path: args[1].clone(),
                        chunk: Some(args[2].clone()),
                        message: if args.len() == 4 {
                            Some(args[3].clone())
                        } else {
                            None
                        },
                    }
                } else {
                    panic!(
                        "\nEncode command needs path and chunk type, and a optional message can be passed\n"
                    )
                }
            }
            Command::Decode | Command::Remove => {
                if args.len() == 3 {
                    Args {
                        command,
                        path: args[1].clone(),
                        chunk: Some(args[2].clone()),
                        message: None,
                    }
                } else {
                    panic!("\nDeocde command needs path and chunk type\n")
                }
            }
            Command::Print => {
                if args.len() == 2 {
                    Args {
                        command,
                        path: args[1].clone(),
                        chunk: None,
                        message: None,
                    }
                } else {
                    panic!("\nPrint command needs path and chunk type\n")
                }
            }
        }
    } else {
        panic!("\nInvalid command\n ")
    };

    args
}
