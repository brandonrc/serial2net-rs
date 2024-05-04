mod config;
use clap::{Parser, ValueEnum};
use anyhow::{Context, Result};
use log::{info, warn, error, debug, LevelFilter};
use env_logger::{Env, Builder};
use crate::config::SerialDeviceConfig;
use crate::serial::AsyncSerialConnection;
use tokio::join;

mod serial;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Verbosity level (e.g., error, warn, info, debug)
    #[clap(short, long, value_enum, default_value_t = LogLevel::Info)]
    verbosity: LogLevel,

    /// Flag to keep the process attached
    #[clap(long)]
    no_detach: bool,

    /// The path to the configuration file
    config_file: std::path::PathBuf,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

impl LogLevel {
    fn to_string(&self) -> &'static str {
        match *self {
            LogLevel::Error => "error",
            LogLevel::Warn => "warn",
            LogLevel::Info => "info",
            LogLevel::Debug => "debug",
        }
    }
}

/// Initializes and runs the main application logic.
///
/// This function parses command-line arguments to configure the application's verbosity level,
/// whether to run in detached mode, and the path to the configuration file. It initializes the logger
/// based on the specified verbosity level, loads the configuration from the given file, and manages
/// serial device connections as specified in the configuration.
///
/// # Errors
/// Returns an error if the configuration file path is invalid, the configuration file cannot be loaded,
/// or if there is an issue managing any of the serial device connections.
///
/// # Examples
/// This function is intended to be run as the entry point of the application and does not return any value.
/// It is called automatically when the program starts if compiled with the `#[tokio::main]` attribute.
#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    // Initialize logger
    Builder::from_env(Env::default().default_filter_or(args.verbosity.to_string()))
        .init();

    // Adjust log level
    log::set_max_level(match args.verbosity {
        LogLevel::Error => LevelFilter::Error,
        LogLevel::Warn => LevelFilter::Warn,
        LogLevel::Info => LevelFilter::Info,
        LogLevel::Debug => LevelFilter::Debug,
    });

    info!("Starting application");

    // Load configuration
    let config_path = args.config_file.to_str().ok_or_else(|| anyhow::anyhow!("Invalid path"))?;
    let config = config::load_config(config_path)
        .with_context(|| format!("Failed to load config from {:?}", args.config_file))?;

    info!("Configuration loaded successfully: {:?}", config);

    if !args.no_detach {
        info!("Running in detached mode");
    }

    let mut connections = Vec::new();

    for device in config.serial_devices {
        let device_config = device.clone();
        let connection_task = tokio::spawn(async move {
            manage_device(device_config).await
        });
        connections.push(connection_task);
    }

    // Run network service and serial device connections concurrently
    let serial_results: Vec<_> = futures::future::join_all(connections).await;

    for result in serial_results {
        result??; // Handle or log errors as needed
    }
    // for connection in connections {
    //     connection.await??;
    // }

    Ok(())
}


/// Manages a single serial device connection.
///
/// This asynchronous function takes a `SerialDeviceConfig` as input, which contains the configuration
/// for a serial device (e.g., port and baud rate). It attempts to maintain an open connection to the
/// device, reading data as it becomes available and handling any errors that occur.
///
/// # Parameters
/// - `device_config`: The configuration for the serial device to be managed.
///
/// # Errors
/// Returns an error if there is an issue opening the port, reading data from the device, or handling
/// the device based on its configuration.
///
/// # Examples
/// This function is not intended to be called directly in application code but is spawned as an
/// asynchronous task for each device in the application's configuration.
async fn manage_device(device_config: SerialDeviceConfig) -> Result<()> {
    let mut connection = AsyncSerialConnection::new(&device_config.port, device_config.baud_rate);

    loop {
        match connection.ensure_port_open().await {
            Ok(_) => {
                // Process data or handle communication
                let data = connection.read_data().await?;
                println!("Received data from {}: {:?}", device_config.device_name, data);
                // Forward data to network service based on the protocol
                match device_config.network_protocol.as_str() {
                    "tcp" => {
                        // Establish TCP connection and send data
                        // Example: send_data_over_tcp(&device_config, &data).await?;
                    },
                    "udp" => {
                        // Send data over UDP
                        // Example: broadcast_data_over_udp(&device_config, &data).await?;
                    },
                    _ => eprintln!("Unsupported network protocol for device {}: {}", device_config.device_name, device_config.network_protocol),
                }
            },
            Err(e) => {
                eprintln!("Error handling device {}: {}", device_config.device_name, e);
                // Reconnect logic or error handling
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await; // Wait before retrying
            }
        }
    }
}