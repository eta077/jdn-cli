use std::collections::HashMap;
use std::io::{BufRead, BufReader, Stdin, Stdout, Write};
use std::sync::Arc;
use std::vec::Vec;

use crate::CliHandler;

/// The string used to represent the manager is waiting for input.
pub const PROMPT: &str = "> ";
/// The command used to print all available commands.
pub const HELP: &str = "help";
/// The command used to stop the manager.
pub const EXIT: &str = "exit";
/// The message displayed when an invalid command is received by the manager.
pub const INVALID_COMMAND: &str = "Invalid command";

/// A manager responsible for handling command line input and output.
pub struct CliManager<R: BufRead, W: Write> {
    reader: R,
    writer: W,
    handlers: HashMap<String, Arc<dyn CliHandler>>,
}

impl Default for CliManager<BufReader<Stdin>, Stdout> {
    fn default() -> Self {
        CliManager {
            reader: BufReader::new(std::io::stdin()),
            writer: std::io::stdout(),
            handlers: HashMap::default(),
        }
    }
}

impl CliManager<BufReader<Stdin>, Stdout> {
    /// Constructs a new CliManager.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<R: BufRead, W: 'static + Write> CliManager<R, W> {
    /// Constructs a new CliManager with the given read/write streams.
    pub fn with_reader_writer(reader: R, writer: W) -> CliManager<R, W> {
        CliManager {
            reader,
            writer,
            handlers: HashMap::new(),
        }
    }

    /// Starts the command line interface. Note that this is a blocking operation; once this function returns, the
    /// user has submitted a request to stop the application ([EXIT]).
    pub fn start(&mut self) {
        loop {
            write!(self.writer, "{}", PROMPT).expect("Failed to print prompt");
            self.writer.flush().expect("Failed to flush prompt");
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("Failed to read line");
            let input = input.trim().to_owned();
            if input.is_empty() {
                continue;
            }
            let (command, args) = parse_input(input);
            if command.is_empty() {
                writeln!(self.writer, "{}", INVALID_COMMAND).expect("Failed to print `Invalid command`");
                continue;
            }

            if EXIT.eq_ignore_ascii_case(&command) {
                break;
            } else if HELP.eq_ignore_ascii_case(&command) {
                let mut cmds: Vec<String> = self.handlers.iter().map(|(cmd, _)| cmd.clone()).collect();
                cmds.sort();
                for cmd in cmds {
                    writeln!(self.writer, "{}", cmd).expect("Failed to print help output");
                }
            } else {
                let handler = self.handlers.get(&command);
                match handler {
                    Some(ref value) => {
                        if let Err(msg) = value.handle_command(&command, args, &mut self.writer) {
                            writeln!(self.writer, "{}", msg).expect("Failed to print error message");
                        }
                    }
                    None => writeln!(self.writer, "{}", INVALID_COMMAND).expect("Failed to print `Invalid command`"),
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
            self.handlers.insert(cmd.to_string(), Arc::clone(&handler));
        }
    }
}

fn parse_input(input: String) -> (String, Vec<String>) {
    let quoted_vars: Vec<&str> = input.split('\"').collect();
    let mut command = String::new();
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
