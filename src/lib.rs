// src/lib.rs

// Flight Brain Framework
// 
// Overview:
// This crate forms the core of the Flight Brain Framework, a robust system designed for efficient and flexible 
// message-based communication between various components in a program. It is particularly well-suited for 
// applications that require modularity and scalability, such as embedded systems or complex application logic.
// 
// Modules:
// - message_queue: Implements a message queue system that handles the asynchronous exchange of messages
//   between different components of the application. This module is crucial for the non-blocking communication
//   pattern that the framework facilitates.
// - system: Defines the `System` trait, a fundamental concept in the framework that represents a modular unit
//   of functionality. Each system can interact with others through the message queue and can alter the program's
//   state.
// - run: Contains the primary runtime loop that drives the application. It coordinates the execution of different
//   systems based on the program state and messages in the queue.
//
// Design Philosophy:
// The Flight Brain Framework emphasizes a decoupled and event-driven architecture, allowing for highly modular 
// and testable code. It is designed to operate in no_std environments, making it suitable for systems with 
// limited resources or specific runtime constraints.
//
// Usage:
// This crate can be used as a foundation for building complex applications where modularity, efficiency, and 
// responsiveness are key requirements. It is especially useful in scenarios where the business logic needs to 
// be cleanly separated from application state and message handling.
//
// Note:
// The crate is designed to be extensible, allowing developers to implement custom systems and integrate 
// them seamlessly into the framework's message-driven architecture.

#![no_std]

extern crate alloc;

pub mod message_queue;
pub mod run;
pub mod system;
