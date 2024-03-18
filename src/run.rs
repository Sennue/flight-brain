// src/run.rs

// The `run` module in the Flight Brain framework is pivotal for managing the execution flow of the entire program.
// It employs a combination of a main loop and a message queue system, designed to offer both flexibility and efficiency in handling various computational tasks.

// Key Components:
// 1. ProgramState and Message Generics:
//    The function within the module is generic over `ProgramState` and `Message`. This design allows for broad applicability
//    across different types of programs, making the module highly reusable and adaptable. `ProgramState` encapsulates the global state
//    of the program, while `Message` represents the communication units between different systems within the framework.

// 2. MessageQueue:
//    Central to the module's operation is the `MessageQueue`. This queue handles messages that are either to be processed in the
//    current tick (an iteration of the main loop) or queued for the next. This facilitates asynchronous and non-blocking communication
//    between different systems, enhancing the responsiveness and scalability of applications.

// 3. System Management:
//    The core of this module is the management of various systems. Each system is an entity that performs specific tasks and operates on the
//    ProgramState and the messages it receives. These systems can be anything from simple loggers to complex data processors.

// 4. Dynamic Update Function (UpdateFunc):
//    A crucial feature of this module is the dynamic update function, defined as a closure. This function is responsible for updating the list
//    of active systems at each tick of the main loop. It takes three parameters: a mutable reference to `ProgramState`, a mutable reference to
//    `MessageQueue`, and a vector of `System` objects. The flexibility of this closure allows for dynamic prioritization and alteration of systems
//    based on the current program state and messages, enabling real-time adaptation to changing conditions.

// 5. Main Loop:
//    The execution loop in the `run` module drives the entire program. It repeatedly checks for active systems, processes messages for the current
//    tick, and updates systems based on the dynamic update function. This loop continues as long as there are systems to process, effectively
//    making it the heartbeat of the application.

// Significance:
// This module's architecture underscores the Flight Brain framework's emphasis on modularity, flexibility, and efficiency.
// By leveraging generics, a message queue, and a dynamic system update mechanism, the `run` module offers a robust foundation
// for building complex, responsive, and scalable applications.

// In summary, the `run` module is a testament to the Flight Brain framework's capabilities in handling intricate program flows and
// system interactions, making it a valuable tool for developers looking to build advanced and dynamic applications.

use crate::{message_queue::MessageQueue, system::System};
use alloc::{boxed::Box, vec, vec::Vec};

pub fn run<ProgramState, Message, UpdateFunc>(
    mut program_state: ProgramState,
    mut message_queue: MessageQueue<Message>,
    mut update: UpdateFunc,
) where
    UpdateFunc: FnMut(
        &mut ProgramState,
        &mut MessageQueue<Message>,
        Vec<Box<dyn System<ProgramState, Message>>>,
    ) -> Vec<Box<dyn System<ProgramState, Message>>>,
{
    let mut systems = update(&mut program_state, &mut message_queue, vec![]);

    while !systems.is_empty() {
        message_queue.next_tick();
        for system in systems.iter_mut() {
            system.update(&mut program_state, &mut message_queue);
        }
        systems = update(&mut program_state, &mut message_queue, systems);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestProgramState {
        done: bool,
        sum: i32,
    }

    struct TestSystem;

    impl System<TestProgramState, i32> for TestSystem {
        fn update(
            &mut self,
            program_state: &mut TestProgramState,
            message_queue: &mut MessageQueue<i32>,
        ) {
            for message_value in message_queue.iter() {
                program_state.sum += message_value;
            }
            message_queue.push(program_state.sum);
            if 10 < program_state.sum {
                program_state.done = true;
            }
        }
    }

    #[test]
    fn test_run() {
        let program_state = TestProgramState {
            done: false,
            sum: 0,
        };
        let message_queue = MessageQueue::new();
        let update_func =
            |program_state: &mut TestProgramState,
             message_queue: &mut MessageQueue<i32>,
             systems: Vec<Box<dyn System<TestProgramState, i32>>>| {
                if program_state.done {
                    // return empty vec to exit
                    Vec::new()
                } else if systems.is_empty() {
                    // push startup messages
                    message_queue.push(1);
                    // initialize systems vec
                    vec![Box::new(TestSystem) as Box<dyn System<TestProgramState, i32>>]
                } else {
                    systems
                }
            };

        run(program_state, message_queue, update_func);
    }
}
