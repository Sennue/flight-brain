// tests/integration_test.rs

extern crate flight_brain;

use flight_brain::{message_queue::MessageQueue, system::System};

#[test]
fn test_ok() {
    assert!(true, "test_ok() failed");
}

#[test]
fn test_run_ok() {
    let program_state = true;
    let message_queue = MessageQueue::new();
    let update_func = |_program_state: &mut bool,
                       _message_queue: &mut MessageQueue<bool>,
                       _systems: Vec<Box<dyn System<bool, bool>>>| {
        Vec::new() // Return empty vector to exit
    };
    flight_brain::run::run(program_state, message_queue, update_func);
}
