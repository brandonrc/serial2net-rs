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

    Ok(())
}
