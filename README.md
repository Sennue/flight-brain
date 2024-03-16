# Flight Brain - Flight Computer Engine

THIS README IS A WORK IN PROGRESS!

## Overview
Flight Brain is a flight computer engine.  Its architecture is centered around a powerful message queue system, enabling efficient communication and processing within various user-defined systems and messages.  This design approach offers exceptional flexibility and scalability.

## Key Features
- **Message Queue Architecture**: Utilizes a message queue system for managing communications between different systems within a tick-based processing framework.
- **User-Defined Systems and Messages**: Offers the flexibility to define custom systems and messages, catering to specific operational needs and scenarios.
- **Tick-Based Processing**: Each system processes messages relevant to the current tick and can enqueue messages for subsequent ticks, ensuring organized and timely data handling.
- **`no_std` Rust Environment**: Built in a `no_std` Rust environment, perfect for embedded systems.
- **Robust and Adaptable**: Designed to be resilient and adaptable for demanding mission requirements.

## Getting Started
To begin using Flight Brain, you should have the Rust toolchain installed on your system. You can download Rust from [the official Rust website](https://www.rust-lang.org/).

## Including Flight Brain as a Dependency

To use Flight Brain in your own Rust project, you can include it as a dependency from its GitHub repository. Follow these steps to set it up:

1. Open your Rust project's `Cargo.toml` file.

2. Under the `[dependencies]` section, add the following line:
   ```toml
   flight_brain = { git = "https://github.com/Sennue/flight_brain.git" }

## Usage
After including the dependency, your systems can be integrated into Flight Brain.  Refer to the documentation for guidance on creating your systems and messages, and how to utilize the message queue for efficient data processing.

## Contributing
Contributions to Flight Brain are highly encouraged.  Please read our contributing guidelines for information on submitting pull requests or issues.

### Installation for Contributors
1. Clone the repository:
   ```bash
   git clone https://github.com/Sennue/flight_brain.git
   ```
2. Navigate to the project directory:
   ```bash
   cd flight_brain
   ```
3. Build the project:
   ```bash
   cargo build
   ```

## License
Flight Brain is licensed under [BSD 2-Clause License](LICENSE).  For more information, see the `LICENSE` file in the repository.

## Contact
For more information, support, or to contribute, please contact Brendan Sechter at [sgeos@hotmail.com].

## Acknowledgements
We extend our gratitude to all the contributors and supporters who have played a pivotal role in the development of this project.

