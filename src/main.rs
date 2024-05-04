mod config;
use clap::{Parser, ValueEnum};
use anyhow::{Context, Result};
use log::{info, warn, error, debug, LevelFilter};
use env_logger::{Env, Builder};

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

fn main() -> Result<()> {
    let args = Cli::parse();

    // Initialize logger
    Builder::from_env(Env::default().default_filter_or(args.verbosity.to_string()))
        .init();

    // Depending on verbosity level set, adjust the log level
    log::set_max_level(match args.verbosity {
        LogLevel::Error => LevelFilter::Error,
        LogLevel::Warn => LevelFilter::Warn,
        LogLevel::Info => LevelFilter::Info,
        LogLevel::Debug => LevelFilter::Debug,
    });

    info!("Starting application");

    // Load configuration
    let config = config::load_config(args.config_file.to_str().unwrap())
        .with_context(|| format!("Failed to load config from {:?}", args.config_file))?;

    info!("Configuration loaded successfully: {:?}", config);

    // If no_detach is false, consider logic to detach (if applicable)
    if !args.no_detach {
        info!("Running in detached mode");
        // Add logic here if you need to detach the process or handle it like a daemon/service
    }


    // Vector to hold our connection tasks
    let mut connections = Vec::new();

    for device in config.serial_devices {
        let device_config = device.clone(); // Clone the device config if necessary
        let connection_task = tokio::spawn(async move {
            manage_device(device_config).await
        });
        connections.push(connection_task);
    }

    // Await all connection tasks (this will run indefinitely if you design it that way)
    for connection in connections {
        connection.await??;
    }


    Ok(())
}

async fn manage_device(device_config: SerialDeviceConfig) -> Result<()> {
    let mut connection = AsyncSerialConnection::new(&device_config.port, device_config.baud_rate);
    
    loop {
        match connection.ensure_port_open().await {
            Ok(_) => {
                // Process data or handle communication
                let data = connection.read_data().await?;
                println!("Received data from {}: {:?}", device_config.device_name, data);
                // Process data or handle it as necessary
            },
            Err(e) => {
                eprintln!("Error handling device {}: {}", device_config.device_name, e);
                // Reconnect logic or error handling
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await; // Wait before retrying
            }
        }
    }
}