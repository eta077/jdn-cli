use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use std::sync::Arc;
use std::vec::Vec;

use crate::CliHandler;

const PROMPT: &str = "> ";
const HELP: &str = "help";
const EXIT: &str = "exit";

/// A manager responsible for handling command line input and output.
#[derive(Default)]
pub struct CliManager {
    handlers: HashMap<String, Arc<dyn CliHandler>>,
}

impl CliManager {
    /// Constructs a new CliManager.
    pub fn new() -> CliManager {
        CliManager::default()
    }

    /// Starts the command line interface. Note that this is a blocking operation; once this function returns, the
    /// user has submitted a request to stop the application.
    pub fn start(&self) {
        loop {
            print!("{}", PROMPT);
            stdout().flush().expect("Failed to print prompt.");
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed to read line");
            let input = String::from(input.trim());
            if input.is_empty() {
                continue;
            }
            let (command, args) = parse_input(input);
            if command.is_empty() {
                println!("Invalid command");
                continue;
            }

            if EXIT.eq_ignore_ascii_case(&command) {
                break;
            } else if HELP.eq_ignore_ascii_case(&command) {
                let mut cmds: Vec<String> = self.handlers.iter().map(|(cmd, _)| cmd.clone()).collect();
                cmds.sort();
                for cmd in cmds {
                    println!("{}", cmd);
                }
            } else {
                let handler = self.handlers.get(&command);
                match handler {
                    Some(ref value) => {
                        if let Err(msg) = value.handle_command(&command, args) {
                            println!("{}", msg);
                        }
                    }
                    None => println!("Invalid command"),
                };
            }
        }
    }

    /// Adds the given CliHandler. All commands returned by [CliHandler::get_commands()] will now be forwarded to
    /// this handler.
    ///
    /// # Arguments
    /// `handler` - A reference to the handler to add. This reference is cloned for retention by the manager.
    pub fn add_handler(&mut self, handler: Arc<dyn CliHandler>) {
        for cmd in handler.get_commands() {
            self.handlers.insert(cmd.to_string(), handler.clone());
        }
    }
}

fn parse_input(input: String) -> (String, Vec<String>) {
    let quoted_vars: Vec<&str> = input.split('\"').collect();
    let mut command = String::from("");
    let mut vars: Vec<String> = Vec::new();
    for element in quoted_vars.iter().enumerate() {
        if element.0 % 2 == 0 {
            let space_vars: Vec<&str> = element.1.split(' ').collect();
            for var in space_vars {
                let var = var.trim();
                if !var.is_empty() {
                    if command.is_empty() {
                        command.push_str(var);
                    } else {
                        vars.push(var.to_string());
                    }
                }
            }
        } else {
            vars.push(element.1.to_string());
        }
    }
    (command, vars)
}
