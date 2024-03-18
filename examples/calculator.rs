// examples/calculator.rs

//! # Flight Brain - Calculator Example
//!
//! This is an example program demonstrating a simple calculator application, designed
//! to showcase the architecture and capabilities of the `flight_brain` framework. The
//! example is tailored to potentially suit aeronautical applications, emphasizing 
//! modularity, message-driven architecture, and system integration. 
//!
//! ## Overview
//!
//! - **Modularity**: The program is structured into distinct systems (`InputSystem`, 
//!   `CalculatorSystem`, `OutputSystem`), each responsible for handling specific 
//!   aspects of the application's logic. This modular approach enhances readability, 
//!   maintainability, and testability.
//!
//! - **Message-Driven Design**: Communication between systems is managed through a 
//!   message queue, using an enumeration to define various message and command types. 
//!   This decouples the systems and allows for clear, manageable data and command flow.
//!
//! - **Program State Management**: Centralized state management is done via the 
//!   `ProgramState` struct, handling variables, accumulator values, and operational 
//!   modes (e.g., batch mode).
//!
//! - **Input Handling**: The `InputSystem` parses user input, translating it into 
//!   commands for the calculator. It supports both interactive and batch processing 
//!   modes, adapting the program's behavior accordingly.
//!
//! - **Calculator Logic**: The `CalculatorSystem` executes core calculator operations, 
//!   including arithmetic functions and variable management. It handles the computational 
//!   logic of the application.
//!
//! - **Output Management**: The `OutputSystem` is responsible for displaying results, 
//!   error messages, and help text, managing the user interface aspect of the application.
//!
//! - **Non-Blocking Input for Batch Mode**: The application can read inputs in a non-blocking 
//!   manner when in batch mode, allowing for flexible interaction models.
//!
//! - **No-Std Compatibility and Memory Management**: The application is compatible with 
//!   `no_std` environments, making it suitable for systems with limited resources or 
//!   those that do not support the Rust standard library.
//!
//! - **Error Handling and Panic Management**: Custom panic handling and error management 
//!   are included to ensure robust operation, particularly important in aeronautical contexts.
//!
//! ## Usage
//!
//! This calculator can be used in both interactive and batch modes. It supports basic arithmetic 
//! operations and variable management, providing a simple yet powerful tool for calculations.
//! It can be extended or modified to fit specific aeronautical computational needs.
//!
//! ## Notes
//!
//! - The code is designed with clarity and simplicity in mind, prioritizing ease of understanding 
//!   and modification.
//! - While this example is tailored for demonstration purposes, it serves as a solid foundation 
//!   for more complex aeronautical applications.
//! - It's crucial to ensure rigorous testing and validation if this code is intended for critical 
//!   aeronautical applications, given the high standards for safety and reliability in the field.
//!
//! Enjoy exploring and extending this `flight_brain` calculator example!

#![no_std]
#![no_main]
#![allow(internal_features)]
#![feature(lang_items)]

extern crate alloc;
extern crate flight_brain;

use alloc::{
    boxed::Box,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use flight_brain::{message_queue::MessageQueue, run::run, system::System};
use hashbrown::HashMap;
use libc::{c_void, fcntl, F_GETFL, F_SETFL, O_NONBLOCK, STDIN_FILENO};
use libc_alloc::LibcAlloc;
use libc_print::std_name::{print, println};

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

// Define Messages
#[derive(Debug)]
enum Message {
    Init,
    Command(Command),
    Result(f64),
    Log(String),
    Error(String),
    Help,
    PollInput,
    FlushOutput,
    Shutdown,
}

#[derive(Debug)]
enum Command {
    Add(f64),
    Subtract(f64),
    Multiply(f64),
    Divide(f64),
    SetValue(f64),
    TargetVariable(String),
    VariableAdd(String),
    VariableSubtract(String),
    VariableMultiply(String),
    VariableDivide(String),
    VariableSetValue(String),
    LoadVariable(String),
    StoreVariable(String),
    Clear,
}

pub struct ProgramState {
    tick: u32,
    variables: HashMap<String, f64>,
    accumulator: f64,
    batch_mode: bool,
    done: bool,
}

impl Default for ProgramState {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgramState {
    fn new() -> Self {
        Self {
            tick: 0,
            variables: HashMap::new(),
            accumulator: 0.0,
            batch_mode: false,
            done: false,
        }
    }
}

// Input System
pub struct InputSystem;

impl Default for InputSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl InputSystem {
    const BUFFER_SIZE: usize = 1024;

    fn new() -> Self {
        Self {}
    }

    // Parses the user input string into a Command
    fn parse_command(input: &str) -> Vec<Message> {
        let mut messages = Vec::new();
        let parts = input.split_whitespace().collect::<Vec<&str>>();

        if parts.is_empty() {
            messages.push(Message::Command(Command::Clear));
            return messages;
        }

        messages = Self::match_command(&parts, 0);
        if messages.is_empty() {
            messages = Self::match_command(&parts, 1);
        }
        messages
    }

    fn match_command(parts: &[&str], index: usize) -> Vec<Message> {
        let mut help = false;
        let parts_len = parts.len() - index;
        let mut messages = Vec::new();

        if parts_len < 1 {
            match index {
                0 => messages.push(Message::Command(Command::Clear)),
                1 => match parts[0].parse::<f64>() {
                    Ok(value) => messages.push(Message::Command(Command::SetValue(value))),
                    Err(_) => messages.push(Message::Command(Command::LoadVariable(
                        parts[0].to_string(),
                    ))),
                },
                _ => unreachable!(),
            }
            return messages;
        } else {
            match index {
                0 => (),
                1 => messages.push(Message::Command(Command::TargetVariable(
                    parts[0].to_string(),
                ))),
                _ => unreachable!(),
            }
        }

        match parts[index] {
            "exit" | "quit" => {
                messages.push(Message::Shutdown);
            }
            "help" => {
                help = true;
            }
            "clear" => {
                messages.push(Message::Command(Command::Clear));
            }
            "+" | "-" | "*" | "/" | "=" => {
                if 1 < parts_len {
                    let command = match parts[index] {
                        "+" => Self::parse_value_or_variable_command(
                            &parts[(index + 1)..],
                            Command::Add,
                            Command::VariableAdd,
                        ),
                        "-" => Self::parse_value_or_variable_command(
                            &parts[(index + 1)..],
                            Command::Subtract,
                            Command::VariableSubtract,
                        ),
                        "*" => Self::parse_value_or_variable_command(
                            &parts[(index + 1)..],
                            Command::Multiply,
                            Command::VariableMultiply,
                        ),
                        "/" => Self::parse_value_or_variable_command(
                            &parts[(index + 1)..],
                            Command::Divide,
                            Command::VariableDivide,
                        ),
                        "=" => Self::parse_value_or_variable_command(
                            &parts[(index + 1)..],
                            Command::SetValue,
                            Command::VariableSetValue,
                        ),
                        _ => unreachable!(),
                    };
                    if let Some(cmd) = command {
                        messages.push(Message::Command(cmd));
                    }
                } else {
                    help = true;
                }
            }
            "set" => {
                if 1 < parts_len {
                    messages.push(Message::Command(Command::StoreVariable(
                        parts[index + 1].to_string(),
                    )));
                } else {
                    help = true;
                }
            }
            _ => (),
        }
        if help {
            messages.push(Message::Help);
            messages.push(Message::FlushOutput);
        }

        messages
    }

    // Helper function to parse a command that requires a value or a variable
    fn parse_value_or_variable_command(
        parts: &[&str],
        value_constructor: fn(f64) -> Command,
        variable_constructor: fn(String) -> Command,
    ) -> Option<Command> {
        if parts.is_empty() {
            None
        } else {
            match parts[0].parse::<f64>() {
                Ok(value) => Some(value_constructor(value)), /* If it's a valid f64, create a */
                // value command
                Err(_) => Some(variable_constructor(parts[0].to_string())), /* Otherwise, create a variable command */
            }
        }
    }

    fn read_input() -> String {
        let mut buffer = [0u8; Self::BUFFER_SIZE]; // Create a buffer for input
        let mut total_bytes_read = 0usize;

        while total_bytes_read < Self::BUFFER_SIZE {
            let bytes_read = unsafe {
                // Read one character at a time
                libc::read(
                    libc::STDIN_FILENO,
                    buffer[total_bytes_read..].as_mut_ptr() as *mut c_void,
                    1,
                )
            };

            if bytes_read <= 0 {
                // In case of an error or end of file, return an empty string.
                return String::new();
            }

            // Check for newline character, which indicates the end of input
            if buffer[total_bytes_read] == b'\n' {
                break;
            }

            total_bytes_read += 1;
        }

        // Convert the buffer to a Rust String, trimming the newline character
        String::from_utf8_lossy(&buffer[..total_bytes_read])
            .trim_end_matches('\n')
            .to_string()
    }

    fn set_stdin_blocking(is_blocking: bool) {
        // Get the current flags of the STDIN file descriptor
        let flags = unsafe { fcntl(STDIN_FILENO, F_GETFL) };
        if flags < 0 {
            // Handle error if necessary
            panic!("Failed to get flags for STDIN");
        }

        // Modify flags based on the is_blocking argument
        let new_flags = if is_blocking {
            flags & !O_NONBLOCK // Clear O_NONBLOCK to set blocking mode
        } else {
            flags | O_NONBLOCK // Set O_NONBLOCK to set non-blocking mode
        };

        // Set the modified flags
        let result = unsafe { fcntl(STDIN_FILENO, F_SETFL, new_flags) };
        if result < 0 {
            // Handle error if necessary
            panic!("Failed to set STDIN blocking state");
        }
    }

    fn check_for_batch_mode(program_state: &mut ProgramState) {
        Self::set_stdin_blocking(false);
        // Read a small amount of data in non-blocking fashion.
        let mut buffer = [0u8; 6]; // Enough to read "batch\n"
        let bytes_read = unsafe {
            libc::read(
                libc::STDIN_FILENO,
                buffer.as_mut_ptr() as *mut c_void,
                buffer.len(),
            )
        };

        if 0 < bytes_read {
            let input = String::from_utf8_lossy(&buffer[..bytes_read as usize]);
            if input.trim().eq_ignore_ascii_case("batch") {
                program_state.batch_mode = true;
            }
        }
        Self::set_stdin_blocking(true);
    }
}

impl System<ProgramState, Message> for InputSystem {
    fn update(
        &mut self,
        program_state: &mut ProgramState,
        message_queue: &mut MessageQueue<Message>,
    ) {
        let mut do_poll_input = false;
        if program_state.done {
            return;
        }
        for message in message_queue.iter() {
            match message {
                Message::Init => {
                    Self::check_for_batch_mode(program_state);
                    return;
                }
                Message::Shutdown => {
                    return;
                }
                Message::PollInput => {
                    do_poll_input = true;
                }
                _ => (),
            }
        }
        if do_poll_input {
            let input = Self::read_input().trim().to_string();
            let is_eof = input.is_empty();
            let commands = if !is_eof {
                Self::parse_command(&input)
            } else {
                vec![Message::Shutdown]
            };

            for command in commands {
                message_queue.push(command);
            }
        }
    }
}

// Calculator System
pub struct CalculatorSystem;

impl Default for CalculatorSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl CalculatorSystem {
    fn new() -> Self {
        Self {}
    }
}

impl System<ProgramState, Message> for CalculatorSystem {
    fn update(
        &mut self,
        program_state: &mut ProgramState,
        message_queue: &mut MessageQueue<Message>,
    ) {
        let mut new_messages = Vec::new();
        let mut flush_output = false;
        let mut error = false;
        let mut is_variable_target = false;
        let mut variable_target_name = "".to_string();
        let mut accumulator = program_state.accumulator;
        for message in message_queue.iter() {
            match message {
                Message::Init => {
                    flush_output = true;
                }
                Message::Command(command) => match command {
                    Command::Add(value) => {
                        accumulator += value;
                        flush_output = true;
                    }
                    Command::Subtract(value) => {
                        accumulator -= value;
                        flush_output = true;
                    }
                    Command::Multiply(value) => {
                        accumulator *= value;
                        flush_output = true;
                    }
                    Command::Divide(value) => {
                        if *value != 0.0 {
                            accumulator /= value;
                            flush_output = true;
                        } else {
                            error = true;
                            new_messages.push(Message::Error("Division by zero".to_string()));
                        }
                    }
                    Command::SetValue(value) => {
                        accumulator = *value;
                        flush_output = true;
                    }
                    Command::VariableAdd(variable) => {
                        let value = *program_state
                            .variables
                            .entry(variable.to_string())
                            .or_insert(0.0);
                        accumulator += value;
                        flush_output = true;
                    }
                    Command::VariableSubtract(variable) => {
                        let value = *program_state
                            .variables
                            .entry(variable.to_string())
                            .or_insert(0.0);
                        accumulator -= value;
                        flush_output = true;
                    }
                    Command::VariableMultiply(variable) => {
                        let value = *program_state
                            .variables
                            .entry(variable.to_string())
                            .or_insert(0.0);
                        accumulator *= value;
                        flush_output = true;
                    }
                    Command::VariableDivide(variable) => {
                        let value = *program_state
                            .variables
                            .entry(variable.to_string())
                            .or_insert(0.0);
                        if value != 0.0 {
                            accumulator /= value;
                            flush_output = true;
                        } else {
                            error = true;
                            new_messages.push(Message::Error("Division by zero".to_string()));
                        }
                    }
                    Command::VariableSetValue(variable) => {
                        let value = *program_state
                            .variables
                            .entry(variable.to_string())
                            .or_insert(0.0);
                        accumulator = value;
                        flush_output = true;
                    }
                    // Add cases for other variable-related commands
                    Command::TargetVariable(variable) => {
                        is_variable_target = true;
                        variable_target_name = variable.to_string();
                        let value = *program_state
                            .variables
                            .entry(variable.to_string())
                            .or_insert(0.0);
                        accumulator = value;
                        flush_output = true;
                    }
                    Command::LoadVariable(variable) => {
                        let value = *program_state
                            .variables
                            .entry(variable.to_string())
                            .or_insert(0.0);
                        accumulator = value;
                        flush_output = true;
                    }
                    Command::StoreVariable(variable) => {
                        is_variable_target = true;
                        variable_target_name = variable.to_string();
                        let value = program_state.accumulator;
                        accumulator = value;
                        flush_output = true;
                    }
                    Command::Clear => {
                        accumulator = 0.0;
                        flush_output = true;
                    }
                },
                _ => {}
            }
        }
        if is_variable_target {
            program_state
                .variables
                .insert(variable_target_name, accumulator);
        } else {
            program_state.accumulator = accumulator;
        }
        if !error {
            new_messages.push(Message::Result(accumulator));
        }
        if flush_output {
            new_messages.push(Message::FlushOutput);
        }
        while !new_messages.is_empty() {
            let message = new_messages.pop();
            message_queue.push(message.expect("Message expected."));
        }
    }
}

// Output System
pub struct OutputSystem {
    target: String,
    value: f64,
    help: bool,
    log_messages: Vec<String>,
    error_messages: Vec<String>,
}

impl Default for OutputSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputSystem {
    fn new() -> Self {
        Self {
            target: "Value".to_string(),
            value: 0.0,
            help: false,
            log_messages: Vec::new(),
            error_messages: Vec::new(),
        }
    }

    fn flush_output(&mut self) {
        // Print stored log messages
        for log_message in &self.log_messages {
            println!("{}", log_message);
        }
        // Print stored error messages
        for error_message in &self.error_messages {
            println!("Error: {}", error_message);
        }
        if self.help {
            Self::print_help();
        } else {
            self.print_result();
        }

        // Display prompt for next command
        print!("> ");

        self.target = "Value".to_string();
        self.value = 0.0;
        self.help = false;
        self.log_messages.clear();
        self.error_messages.clear();
    }

    fn print_result(&self) {
        println!("{}: {}", self.target, self.value);
    }

    fn print_help() {
        println!("Commands:");
        println!("    exit | quit : Terminate the program");
        println!("    help : Print commands");
        println!("    clear : Set accumulator to zero");
        println!("    = <value> : Set accumulator to <value>");
        println!("    + <value> : Add value to accumulator");
        println!("    - <value> : Subtract value from accumulator");
        println!("    * <value> : Multiply accumulator by value");
        println!("    / <value> : Divide accumulator by value");
        println!("    set <variable> : Set variable to the accumulator");
    }
}

impl System<ProgramState, Message> for OutputSystem {
    fn update(
        &mut self,
        program_state: &mut ProgramState,
        message_queue: &mut MessageQueue<Message>,
    ) {
        let mut flush_output = false;
        if program_state.done {
            return;
        }
        for message in message_queue.iter_mut() {
            match message {
                Message::Init => {
                    return;
                }
                Message::Shutdown => {
                    if program_state.batch_mode {
                        println!("{}", program_state.accumulator);
                    }
                    return;
                }
                Message::Result(value) => {
                    self.value = *value;
                }
                Message::Error(error_message) => {
                    self.error_messages.push(error_message.clone());
                }
                Message::Log(log_message) => {
                    self.log_messages.push(log_message.clone());
                }
                Message::Help => {
                    self.help = true;
                }
                Message::Command(Command::TargetVariable(variable)) => {
                    self.target = variable.to_string();
                }
                Message::FlushOutput => {
                    flush_output = true;
                }
                _ => (),
            }
        }
        if flush_output {
            if !program_state.batch_mode {
                self.flush_output();
            }
            message_queue.push(Message::PollInput);
        }
    }
}

#[no_mangle]
pub extern "C" fn main() {
    let program_state = ProgramState::new(); // Initialize the program state
    let message_queue = MessageQueue::new(); // Initialize the message queue

    let update_func = |program_state: &mut ProgramState,
                       message_queue: &mut MessageQueue<Message>,
                       systems: Vec<Box<dyn System<ProgramState, Message>>>| {
        program_state.tick += 1;
        let result = if program_state.done {
            // Exit when the program is done.
            Vec::new()
        } else if systems.is_empty() {
            // Push startup messages.
            message_queue.push(Message::Init);
            // Initialize systems.
            vec![
                Box::new(CalculatorSystem::new()) as Box<dyn System<ProgramState, Message>>,
                Box::new(OutputSystem::new()) as Box<dyn System<ProgramState, Message>>,
                Box::new(InputSystem::new()) as Box<dyn System<ProgramState, Message>>,
            ]
        } else {
            // This example does not dynamically prioritize systems, so the list is static.
            systems
        };

        // Prepare the initial part of the log line with the tick number
        let mut log_line = format!("Tick {} : ", program_state.tick);

        // Collect the message descriptions in a vector
        let messages: Vec<String> = message_queue
            .iter()
            .filter(|message| !matches!(message, Message::Log(_)))
            .map(|message| format!("{:?}", message))
            .collect();

        // Join the messages with a comma and a space, then add to the log line
        if !messages.is_empty() {
            log_line.push_str(&messages.join(", "));
        }

        message_queue.push(Message::Log(log_line));

        program_state.done = message_queue
            .iter()
            .any(|message| matches!(message, Message::Shutdown));

        result
    };

    run(program_state, message_queue, update_func);
}

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {} // Panic handler loops indefinitely.
}

// Empty personality function for no_std compatibility.
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
