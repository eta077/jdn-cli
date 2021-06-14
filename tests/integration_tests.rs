use std::collections::HashSet;
use std::io::{stdout, BufReader, Read, Write};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use jdn_cli::manager::{CliManager, INVALID_COMMAND, PROMPT};
use jdn_cli::CliError;
use jdn_cli::CliHandler;

#[test]
fn test_manager_empty() -> std::io::Result<()> {
    // represents stdin
    let mut in_stream = TestBuffer::default();
    // represents stdout
    let out_stream = TestBuffer::default();

    // start manager
    let reader = in_stream.clone();
    let writer = out_stream.clone();
    let manager_handle = thread::Builder::new()
        .name(String::from("JdnCli-Manager"))
        .spawn(move || {
            let mut manager = CliManager::with_reader_writer(BufReader::new(reader), writer);
            manager.start();
        })
        .expect("failed to spawn thread");
    thread::sleep(Duration::from_millis(250));
    let mut out_buf = BufReader::new(out_stream);

    // expect prompt
    let mut prompt_buf = [0; PROMPT.len()];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(PROMPT.to_owned()))?;

    // send help command
    let help_command = b"help\n";
    in_stream.write(help_command)?;
    print(help_command.to_vec(), None)?;

    // expect prompt
    let mut prompt_buf = [0; PROMPT.len()];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(PROMPT.to_owned()))?;

    // send invalid command
    let invalid_command = b"invalid\n";
    in_stream.write(invalid_command)?;
    print(invalid_command.to_vec(), None)?;

    // expect invalid command response
    let invalid_response = INVALID_COMMAND.to_owned() + "\n";
    let mut prompt_buf = [0; INVALID_COMMAND.len() + 1];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(invalid_response))?;

    // expect prompt
    let mut prompt_buf = [0; PROMPT.len()];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(PROMPT.to_owned()))?;

    // send exit command
    let exit_command = b"exit\n";
    in_stream.write(exit_command)?;
    print(exit_command.to_vec(), None)?;

    // expect manager to stop
    manager_handle.join().unwrap();

    Ok(())
}

#[test]
fn test_one_handler() -> std::io::Result<()> {
    // represents stdin
    let mut in_stream = TestBuffer::default();
    // represents stdout
    let out_stream = TestBuffer::default();

    // start manager
    let reader = in_stream.clone();
    let writer = out_stream.clone();
    let manager_handle = thread::Builder::new()
        .name(String::from("JdnCli-Manager"))
        .spawn(move || {
            let mut manager = CliManager::with_reader_writer(BufReader::new(reader), writer);
            let handler = TestHandler::new();
            manager.add_handler(Arc::new(handler));
            manager.start();
        })
        .expect("failed to spawn thread");
    thread::sleep(Duration::from_millis(250));
    let mut out_buf = BufReader::new(out_stream);

    // expect prompt
    let mut prompt_buf = [0; PROMPT.len()];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(PROMPT.to_owned()))?;

    // send help command
    let help_command = b"help\n";
    in_stream.write(help_command)?;
    print(help_command.to_vec(), None)?;

    // expect help response
    let help_response = TestHandler::IS_RUNNING_COMMAND.to_owned()
        + "\n"
        + TestHandler::START_COMMAND
        + "\n"
        + TestHandler::STOP_COMMAND
        + "\n";
    let mut prompt_buf = [0; TestHandler::IS_RUNNING_COMMAND.len()
        + TestHandler::START_COMMAND.len()
        + TestHandler::STOP_COMMAND.len()
        + 3];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(help_response))?;

    // expect prompt
    let mut prompt_buf = [0; PROMPT.len()];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(PROMPT.to_owned()))?;

    // send start command
    let start_command = b"start\n";
    in_stream.write(start_command)?;
    print(start_command.to_vec(), None)?;

    // expect start response
    let started_response = String::from("started\n");
    let mut prompt_buf = [0; 8];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(started_response))?;

    // expect prompt
    let mut prompt_buf = [0; PROMPT.len()];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(PROMPT.to_owned()))?;

    // send exit command
    let exit_command = b"exit\n";
    in_stream.write(exit_command)?;
    print(exit_command.to_vec(), None)?;

    // expect manager to stop
    manager_handle.join().unwrap();

    Ok(())
}

#[test]
fn test_two_handlers() -> std::io::Result<()> {
    // represents stdin
    let mut in_stream = TestBuffer::default();
    // represents stdout
    let out_stream = TestBuffer::default();

    // start manager
    let reader = in_stream.clone();
    let writer = out_stream.clone();
    let manager_handle = thread::Builder::new()
        .name(String::from("JdnCli-Manager"))
        .spawn(move || {
            let mut manager = CliManager::with_reader_writer(BufReader::new(reader), writer);
            manager.add_handler(Arc::new(TestHandler::new()));
            manager.add_handler(Arc::new(TestHandler2::new()));
            manager.start();
        })
        .expect("failed to spawn thread");
    thread::sleep(Duration::from_millis(250));
    let mut out_buf = BufReader::new(out_stream);

    // expect prompt
    let mut prompt_buf = [0; PROMPT.len()];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(PROMPT.to_owned()))?;

    // send help command
    let help_command = b"help\n";
    in_stream.write(help_command)?;
    print(help_command.to_vec(), None)?;

    // expect help response
    let help_response = TestHandler2::END_COMMAND.to_owned()
        + "\n"
        + TestHandler::IS_RUNNING_COMMAND
        + "\n"
        + TestHandler::START_COMMAND
        + "\n"
        + TestHandler::STOP_COMMAND
        + "\n";
    let mut prompt_buf = [0; TestHandler2::END_COMMAND.len()
        + TestHandler::IS_RUNNING_COMMAND.len()
        + TestHandler::START_COMMAND.len()
        + TestHandler::STOP_COMMAND.len()
        + 4];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(help_response))?;

    // expect prompt
    let mut prompt_buf = [0; PROMPT.len()];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(PROMPT.to_owned()))?;

    // send stop command
    let stop_command = b"stop\n";
    in_stream.write(stop_command)?;
    print(stop_command.to_vec(), None)?;

    // expect stop response
    let stopped_response = String::from("stopped\n");
    let mut prompt_buf = [0; 8];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(stopped_response))?;

    // expect prompt
    let mut prompt_buf = [0; PROMPT.len()];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(PROMPT.to_owned()))?;

    // send start command
    let start_command = b"start\n";
    in_stream.write(start_command)?;
    print(start_command.to_vec(), None)?;

    // expect start response from handler 2
    let started_response = String::from("begun\n");
    let mut prompt_buf = [0; 6];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(started_response))?;

    // expect prompt
    let mut prompt_buf = [0; PROMPT.len()];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(PROMPT.to_owned()))?;

    // send exit command
    let exit_command = b"exit\n";
    in_stream.write(exit_command)?;
    print(exit_command.to_vec(), None)?;

    // expect manager to stop
    manager_handle.join().unwrap();

    Ok(())
}

#[test]
fn test_complex_commands() -> std::io::Result<()> {
    // represents stdin
    let mut in_stream = TestBuffer::default();
    // represents stdout
    let out_stream = TestBuffer::default();

    // start manager
    let reader = in_stream.clone();
    let writer = out_stream.clone();
    let manager_handle = thread::Builder::new()
        .name(String::from("JdnCli-Manager"))
        .spawn(move || {
            let mut manager = CliManager::with_reader_writer(BufReader::new(reader), writer);
            let handler = TestHandler::new();
            manager.add_handler(Arc::new(handler));
            manager.start();
        })
        .expect("failed to spawn thread");
    thread::sleep(Duration::from_millis(250));
    let mut out_buf = BufReader::new(out_stream);

    // expect prompt
    let mut prompt_buf = [0; PROMPT.len()];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(PROMPT.to_owned()))?;

    // send start command
    let start_command = b"start\n";
    in_stream.write(start_command)?;
    print(start_command.to_vec(), None)?;

    // expect start response
    let started_response = String::from("started\n");
    let mut prompt_buf = [0; 8];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(started_response))?;

    // expect prompt
    let mut prompt_buf = [0; PROMPT.len()];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(PROMPT.to_owned()))?;

    // execute calculate command
    let calculate_command = b"calculate \"complex calculation\" 1 + 1\n";
    in_stream.write(calculate_command)?;
    print(calculate_command.to_vec(), None)?;

    // expect calculate response
    let calculate_response = String::from("complex calculation is 2\n");
    let mut prompt_buf = [0; 25];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(calculate_response))?;

    // expect prompt
    let mut prompt_buf = [0; PROMPT.len()];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(PROMPT.to_owned()))?;

    // execute calculate command
    let calculate_command = b"calculate \"complex calculation\" 1 +\n";
    in_stream.write(calculate_command)?;
    print(calculate_command.to_vec(), None)?;

    // expect invalid number of arguments response
    let calculate_response = String::from("Invalid number of arguments: expected 4, received 3.\n");
    let mut prompt_buf = [0; 53];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(calculate_response))?;

    // expect prompt
    let mut prompt_buf = [0; PROMPT.len()];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(PROMPT.to_owned()))?;

    // execute calculate command
    let calculate_command = b"calculate \"complex calculation\" 1 plus 1\n";
    in_stream.write(calculate_command)?;
    print(calculate_command.to_vec(), None)?;

    // expect invalid number of arguments response
    let calculate_response = String::from("Argument parse failure: too many characters in string\n");
    let mut prompt_buf = [0; 54];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(calculate_response))?;

    // expect prompt
    let mut prompt_buf = [0; PROMPT.len()];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(PROMPT.to_owned()))?;

    // send stop command
    let stop_command = b"stop\n";
    in_stream.write(stop_command)?;
    print(stop_command.to_vec(), None)?;

    // expect stop response
    let stop_response = String::from("stopped\n");
    let mut prompt_buf = [0; 8];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(stop_response))?;

    // expect prompt
    let mut prompt_buf = [0; PROMPT.len()];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(PROMPT.to_owned()))?;

    // execute calculate command
    let calculate_command = b"calculate \"complex calculation\" 1 + 1\n";
    in_stream.write(calculate_command)?;
    print(calculate_command.to_vec(), None)?;

    // expect execution error response
    let calculate_response = String::from("Execution error: TestHandler not started.\n");
    let mut prompt_buf = [0; 42];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(calculate_response))?;

    // expect prompt
    let mut prompt_buf = [0; PROMPT.len()];
    out_buf.read(&mut prompt_buf)?;
    print(prompt_buf.to_vec(), Some(PROMPT.to_owned()))?;

    // send exit command
    let exit_command = b"exit\n";
    in_stream.write(exit_command)?;
    print(exit_command.to_vec(), None)?;

    // expect manager to stop
    manager_handle.join().unwrap();

    Ok(())
}

fn print(buf: Vec<u8>, expect: Option<String>) -> std::io::Result<()> {
    let msg = String::from_utf8(buf).unwrap();
    if let Some(expect) = expect {
        assert_eq!(msg, expect);
    }
    print!("{}", msg);
    stdout().flush()?;
    Ok(())
}

#[derive(Default)]
struct TestBuffer {
    buffer: Arc<Mutex<Vec<u8>>>,
}

impl Clone for TestBuffer {
    fn clone(&self) -> Self {
        TestBuffer {
            buffer: Arc::clone(&self.buffer),
        }
    }
}

impl Read for TestBuffer {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        loop {
            let mut contents = self
                .buffer
                .lock()
                .expect("Unable to lock read buffer")
                .drain(..)
                .collect::<Vec<u8>>();
            let len = contents.len();
            if len > 0 {
                buf[..len].copy_from_slice(&mut contents);
                return Ok(len);
            } else {
                thread::sleep(Duration::from_millis(50));
            }
        }
    }
}

impl Write for TestBuffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut contents = self.buffer.lock().expect("Unable to lock write buffer");
        contents.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub struct TestHandler {
    on: Mutex<Option<bool>>,
}

impl TestHandler {
    pub const START_COMMAND: &'static str = "start";
    pub const CALCULATE_COMMAND: &'static str = "calculate";
    pub const STOP_COMMAND: &'static str = "stop";
    pub const IS_RUNNING_COMMAND: &'static str = "is-running";
    pub const COMMANDS: [&'static str; 4] = [
        Self::START_COMMAND,
        Self::CALCULATE_COMMAND,
        Self::STOP_COMMAND,
        Self::IS_RUNNING_COMMAND,
    ];

    pub fn new() -> Self {
        TestHandler { on: Mutex::new(None) }
    }
}

impl CliHandler for TestHandler {
    fn get_commands(&self) -> HashSet<&'static str> {
        Self::COMMANDS.iter().cloned().collect()
    }

    fn handle_command(&self, command: &str, args: Vec<String>, writer: &mut dyn Write) -> Result<(), CliError> {
        match command {
            Self::START_COMMAND => {
                *self.on.lock().expect("Unable to lock `on`") = Some(true);
                writeln!(writer, "started").expect("Failed to write start response");
                Ok(())
            }
            Self::CALCULATE_COMMAND => {
                if args.len() != 4 {
                    return Err(CliError::InvalidNumberOfArguments {
                        min: 4,
                        max: None,
                        given: args.len(),
                    });
                }
                if self.on.lock().expect("Unable to lock `on`").unwrap_or(false) {
                    let result_name = &args[0];
                    let num1 =
                        i32::from_str_radix(&args[1], 10).map_err(|e| CliError::ArgumentParseFailure(e.to_string()))?;
                    let operator =
                        char::from_str(&args[2]).map_err(|e| CliError::ArgumentParseFailure(e.to_string()))?;
                    let num2 =
                        i32::from_str_radix(&args[3], 10).map_err(|e| CliError::ArgumentParseFailure(e.to_string()))?;
                    match operator {
                        '+' => {
                            writeln!(writer, "{} is {}", result_name, num1 + num2)
                                .expect("Failed to write calculate response");
                        }
                        '-' => {
                            writeln!(writer, "{} is {}", result_name, num1 - num2)
                                .expect("Failed to write calculate response");
                        }
                        '/' => {
                            writeln!(writer, "{} is {}", result_name, num1 / num2)
                                .expect("Failed to write calculate response");
                        }
                        '*' | 'x' => {
                            writeln!(writer, "{} is {}", result_name, num1 * num2)
                                .expect("Failed to write calculate response");
                        }
                        _ => {
                            return Err(CliError::ArgumentParseFailure(format!(
                                "{} is not a valid operator.",
                                operator
                            )));
                        }
                    }
                    Ok(())
                } else {
                    Err(CliError::ExecutionError(String::from("TestHandler not started.")))
                }
            }
            Self::STOP_COMMAND => {
                *self.on.lock().expect("Unable to lock `on`") = Some(false);
                writeln!(writer, "stopped").expect("Failed to write stop response");
                Ok(())
            }
            Self::IS_RUNNING_COMMAND => {
                writeln!(writer, "{:?}", *self.on.lock().expect("Unable to lock `on`"))
                    .expect("Failed to write is-running response");
                Ok(())
            }
            _ => Err(CliError::ExecutionError(format!("Unknown command: {}", command))),
        }
    }
}

pub struct TestHandler2 {
    on: Mutex<Option<bool>>,
}

impl TestHandler2 {
    pub const START_COMMAND: &'static str = "start";
    pub const END_COMMAND: &'static str = "end";
    pub const COMMANDS: [&'static str; 2] = [Self::START_COMMAND, Self::END_COMMAND];

    pub fn new() -> Self {
        TestHandler2 { on: Mutex::new(None) }
    }
}

impl CliHandler for TestHandler2 {
    fn get_commands(&self) -> HashSet<&'static str> {
        Self::COMMANDS.iter().cloned().collect()
    }

    fn handle_command(&self, command: &str, _args: Vec<String>, writer: &mut dyn Write) -> Result<(), CliError> {
        match command {
            Self::START_COMMAND => {
                *self.on.lock().expect("Unable to lock `on`") = Some(true);
                writeln!(writer, "begun").expect("Failed to write start response");
                Ok(())
            }
            Self::END_COMMAND => {
                *self.on.lock().expect("Unable to lock `on`") = Some(false);
                writeln!(writer, "ended").expect("Failed to write end response");
                Ok(())
            }
            _ => Err(CliError::ExecutionError(format!("Unknown command: {}", command))),
        }
    }
}
