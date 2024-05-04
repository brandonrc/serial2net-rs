# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
#### Added
- UDP Broadcaster (Tokio Async Library)
- TCP Handler (Tokio Async Library)
- Initial UML/Diagram drawio to illustrate application

## [0.2.0] - 2024-05-04

### Added
- Initial folder/repository structure for the project setup.
- Main Rust application file with basic setup for command-line parsing and configuration loading.
- Network module to handle network connections and operations, including TCP and UDP protocols.
- Serial module to manage connections from serial devices using `tokio-serial` for asynchronous operations.
- Device manager logic in the main file to manage serial device connections, including opening ports and reading data asynchronously.
- Configuration handling through `config.rs` and `default.toml` for application settings, including log level and serial device specifications.
- Logging setup using `env_logger` and `log` crates, with adjustable log levels based on command-line arguments.

### Changed
- Updated the serial module to utilize `tokio-serial` for improved asynchronous serial port communication.
- Enhanced the main application logic to use `tokio::main` macro for asynchronous runtime, facilitating non-blocking I/O operations.

## [0.1.0] - 2024-05-03
### Added
- Initial release of the `serial2net-rs` project, providing basic functionality for bridging serial devices to the network via TCP/UDP.