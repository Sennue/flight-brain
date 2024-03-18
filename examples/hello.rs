// examples/hello.rs

// This Rust file, `hello.rs`, is an example implementation in the Flight Brain project.
// It demonstrates a basic structure and functionality of a system within this framework.
// The primary features of this example include:
// - No standard library (`#![no_std]`): The example is written without the Rust standard library,
//   making it suitable for embedded or resource-constrained environments.
// - Message-driven architecture: Uses a message queue (`MessageQueue`) and a simple enum (`Message`)
//   to facilitate communication between different components (systems) in the program.
// - Program state management (`ProgramState`): Manages the overall state of the program.
// - `HelloSystem`: A minimalistic system that exemplifies a component within the Flight Brain
//   framework. It responds to initialization (`Init`) and shutdown (`Shutdown`) messages and 
//   demonstrates basic logging functionality with a "Hello, World!" message.
// - System update loop: The core of the program's execution, showing how systems are updated
//   based on messages and current program state.
// - Global allocator setup (`LibcAlloc`): Illustrates the use of a global allocator in a `no_std`
//   context, which is critical for memory management in such environments.
// - Custom panic handler and language items: Includes necessary components for `no_std`
//   compatibility, such as a panic handler and the `eh_personality` function.

// The example serves as an educational tool for understanding the basics of the Flight Brain
// project's structure and operational logic. It's designed to be simple yet illustrative of
// the fundamental concepts needed to build more complex systems within this framework.


#![no_std]
#![no_main]
#![allow(internal_features)]
#![feature(lang_items)]

extern crate alloc;
extern crate flight_brain;

use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use flight_brain::{message_queue::MessageQueue, run::run, system::System};
use libc_print::std_name::println;

use libc_alloc::LibcAlloc;

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

// State of the program. A production system will be more complex.
pub struct ProgramState {
    pub done: bool,
}

impl Default for ProgramState {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgramState {
    pub fn new() -> Self {
        ProgramState { done: false }
    }
}

// An enum that defines the messages the systems exchange.
// Messages can be simple signals, like Init and Shutdown, or contain data, like
// Log.
enum Message {
    Init,
    Log(String),
    Shutdown,
}

// The HelloSystem is a basic example of a system that prints "Hello, World!"
// and manages program flow. While simplistic for demonstration purposes, it
// represents a fundamental structure of a system with control flow. Typically,
// more complex production systems would include internal state variables and
// more elaborate logic. Init => Log("Hello, World!") => Shutdown
pub struct HelloSystem {}

impl Default for HelloSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl HelloSystem {
    pub fn new() -> Self {
        HelloSystem {}
    }
}

impl System<ProgramState, Message> for HelloSystem {
    // Called every system tick to process messages.
    fn update(
        &mut self,
        program_state: &mut ProgramState,
        message_queue: &mut MessageQueue<Message>,
    ) {
        let mut init: bool = false;
        for message in message_queue.iter_mut() {
            match message {
                Message::Init => {
                    init = true;
                }
                Message::Log(text) => {
                    println!("{}", text);
                }
                Message::Shutdown => {
                    program_state.done = true;
                }
            }
        }

        // On initialization, send the message to log.
        if init {
            message_queue.push(Message::Log("Hello, World!".to_string()));
        }
        // If there are no messages to process, initiate shutdown.
        else if 0 == message_queue.iter().count() {
            message_queue.push(Message::Shutdown);
        }
    }
}

#[no_mangle]
pub extern "C" fn main() {
    let program_state = ProgramState::new(); // Initialize the program state
    let message_queue = MessageQueue::new(); // Initialize the message queue

    // The run loop orchestrates the program's execution. It continuously updates
    // systems based on the current state and messages. Each iteration of the
    // loop represents a 'system tick,' where systems can react to messages and
    // modify the program state.
    let update_func = |program_state: &mut ProgramState,
                       message_queue: &mut MessageQueue<Message>,
                       systems: Vec<Box<dyn System<ProgramState, Message>>>| {
        if program_state.done {
            // Exit when the program is done.
            Vec::new()
        } else if systems.is_empty() {
            // Push startup messages.
            message_queue.push(Message::Init);
            // Initialize systems.
            vec![Box::new(HelloSystem::new()) as Box<dyn System<ProgramState, Message>>]
        } else {
            // This example does not dynamically prioritize systems, so the list is static.
            systems
        }
    };

    // Run the main loop of the program.
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
