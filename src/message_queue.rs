// src/message_queue.rs

// The `message_queue.rs` module in the Flight Brain project defines a core utility for managing
// inter-system communication via message passing. Key characteristics and features of this module
// include:

// - No Standard Library: As with other parts of the Flight Brain project, this module is designed
//   to work in a `no_std` environment, making it suitable for embedded or low-resource systems.

// - Double-buffered Queue: The `MessageQueue` structure uses two `VecDeque` (double-ended queues)
//   instances, `current_tick_queue` and `next_tick_queue`. This double-buffering approach separates
//   message processing for the current system tick from message queuing for the next tick, ensuring
//   clear separation of immediate and future actions.

// - Generic Implementation: The `MessageQueue<T>` is generic, allowing it to handle various message
//   types, making the queue flexible and adaptable to different system requirements.

// - Iterators: The module provides both immutable and mutable iterators over the current tick's
//   messages, enabling systems to process messages effectively during their update cycle.

// - Queue Management: Messages for the next system tick are queued using the `push` method, and
//   moving messages to the current tick's queue is handled by the `next_tick` method. This setup
//   facilitates clear transitions between system ticks and simplifies message lifecycle management.

// - Testing: The included tests demonstrate the functionality of the message queue, such as message
//   pushing, tick transition handling, and behavior with empty queues. These tests ensure the
//   reliability and correctness of the `MessageQueue`'s implementation.

// The `MessageQueue` plays a pivotal role in the Flight Brain framework, enabling asynchronous and
// decoupled communication between different components (systems) of an application. Its design
// emphasizes efficiency, flexibility, and clarity, making it a fundamental tool for developers
// working with this framework.

extern crate alloc;
use alloc::collections::VecDeque;
use core::mem;

pub struct MessageQueue<T> {
    current_tick_queue: VecDeque<T>,
    next_tick_queue: VecDeque<T>,
}

impl<T> Default for MessageQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> MessageQueue<T> {
    pub fn new() -> Self {
        MessageQueue {
            current_tick_queue: VecDeque::new(),
            next_tick_queue: VecDeque::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.current_tick_queue.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.current_tick_queue.iter_mut()
    }

    pub fn push(&mut self, message: T) {
        self.next_tick_queue.push_back(message);
    }

    pub fn next_tick(&mut self) {
        mem::swap(&mut self.current_tick_queue, &mut self.next_tick_queue);
        self.next_tick_queue.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_iter() {
        let mut queue: MessageQueue<i32> = MessageQueue::new();
        queue.push(1);
        queue.push(2);

        assert_eq!(queue.iter().next(), None);
        queue.next_tick();

        let mut iter = queue.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_next_tick_clears_next_tick_queue() {
        let mut queue: MessageQueue<i32> = MessageQueue::new();
        queue.push(1);
        queue.push(2);

        assert_eq!(queue.current_tick_queue.len(), 0);
        assert_eq!(queue.next_tick_queue.len(), 2);
        queue.next_tick();

        queue.push(3);
        assert_eq!(queue.current_tick_queue.len(), 2);
        assert_eq!(queue.next_tick_queue.len(), 1);

        queue.next_tick();

        let mut iter = queue.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_empty_queue() {
        let mut queue: MessageQueue<i32> = MessageQueue::new();
        assert!(queue.iter().next().is_none());
        queue.next_tick();
        assert!(queue.iter().next().is_none());
    }
}
