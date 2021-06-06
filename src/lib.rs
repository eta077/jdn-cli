#![deny(missing_docs)]
//! A service used to provide a command line user interface.

/// Defines a manager responsible for handling command line input and output.
pub mod manager;

use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::vec::Vec;

/// An enumeration of errors that can occur while executing a CLI command.
#[derive(Debug)]
pub enum CliError {
    /// Indicates an invalid number of arguments was given to the handler.
    InvalidNumberOfArguments {
        /// The minimum or only number of arguments expected.
        min: usize,
        /// The maximum number of arguments expected.
        max: Option<usize>,
        /// The number of arguments that was given to the handler.
        given: usize,
    },
    /// Indicates an argument was unable to be coerced from a String.
    /// The internal attribute contains a description of the error that occurred.
    ArgumentParseFailure(String),
    /// Indicates an error occurred while executing the command.
    /// The internal attribute contains a description of the error that occurred.
    ExecutionError(String),
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            CliError::InvalidNumberOfArguments { min, max, given } => {
                let expected_max_string = match max {
                    Some(val) => format!("-{}", val),
                    None => String::new(),
                };
                write!(
                    f,
                    "Invalid number of arguments: expected {}{}, received {}.",
                    min, expected_max_string, given
                )
            }
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Error for CliError {}

/// The trait that allows commands received from the command line interface to be translated and executed.
pub trait CliHandler {
    /// Gets the commands that the handler is able to translate and execute. Returns the commands for which the handler is responsible.
    /// Note that the contents of the Set must not change over the lifetime of the handler.
    fn get_commands(&self) -> HashSet<&'static str>;

    /// Parses the given arguments and executes the given command.
    ///
    /// # Errors
    /// A Result indicating if an error occurred while executing the command, or if the command could not be executed.
    fn handle_command(&self, command: &str, args: Vec<String>) -> Result<(), CliError>;
}
