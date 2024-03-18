// src/system.rs

// The `system.rs` module in the Flight Brain project introduces the `System` trait, a fundamental
// component of the project's architecture. This module's core functionalities and design principles include:

// - Trait-Based Design: The `System` trait defines a common interface for all systems in the Flight Brain framework. 
//   This approach facilitates polymorphism, allowing various systems with different functionalities to be treated uniformly.

// - Generic Parameters: The trait is generic over `ProgramState` and `Message`, enabling systems to work with a 
//   wide range of program states and message types. This flexibility allows the `System` trait to be adaptable to 
//   different applications and use cases within the framework.

// - Update Method: The primary method of the trait, `update`, takes mutable references to the `ProgramState` and 
//   a `MessageQueue<Message>`. This design emphasizes the role of systems in actively modifying the program state 
//   and processing messages during each update cycle.

// - Decoupling: By relying on message passing (via `MessageQueue`) for communication, the `System` implementations 
//   are decoupled from each other. This decoupling promotes modularity and maintainability in complex software architectures.

// - Testing: The module includes tests demonstrating the usage of the `System` trait with a test implementation, `TestSystem`, 
//   and a sample `TestProgramState`. These tests validate the functionality of the `System` trait and provide a reference 
//   implementation for testing custom systems.

// Overall, the `system.rs` module plays a crucial role in the Flight Brain framework by defining a standardized interface for 
// systems. Its design supports a scalable, modular approach to building complex software systems, particularly in resource-constrained 
// or embedded environments where the Flight Brain project is typically deployed.

use crate::message_queue::MessageQueue;

pub trait System<ProgramState, Message> {
    fn update(&mut self, program_state: &mut ProgramState, messages: &mut MessageQueue<Message>);
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
            messages: &mut MessageQueue<i32>,
        ) {
            for message_value in messages.iter() {
                program_state.sum += message_value;
            }
            program_state.done = true;
            messages.push(program_state.sum);
        }
    }

    #[test]
    fn test_system_update() {
        let mut program_state = TestProgramState {
            done: false,
            sum: 0,
        };
        let mut message_queue = MessageQueue::new();
        message_queue.push(10);
        message_queue.push(20);

        assert_eq!(message_queue.iter().count(), 0);
        assert_eq!(message_queue.iter().next(), None);

        message_queue.next_tick(); // Move messages to current tick

        assert_eq!(program_state.sum, 0);
        assert_eq!(program_state.done, false);
        assert_eq!(message_queue.iter().count(), 2);
        assert_eq!(message_queue.iter().next(), Some(&10));
        assert_eq!(message_queue.iter().skip(1).next(), Some(&20));

        let mut test_system = TestSystem;
        test_system.update(&mut program_state, &mut message_queue);

        assert_eq!(program_state.sum, 30); // 10 + 20
        assert_eq!(program_state.done, true);
        assert_eq!(message_queue.iter().count(), 2);
        assert_eq!(message_queue.iter().next(), Some(&10));
        assert_eq!(message_queue.iter().skip(1).next(), Some(&20));

        message_queue.next_tick(); // Move messages to current tick

        assert_eq!(message_queue.iter().count(), 1);
        assert_eq!(message_queue.iter().next(), Some(&30));
    }
}
