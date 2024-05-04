use serde::Deserialize;
use std::fs;
use std::io::Result as IoResult;

// Define structs to deserialize the TOML configuration
#[derive(Debug, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub serial_devices: Vec<SerialDeviceConfig>,
    pub network: NetworkConfig,
    pub discovery: DiscoveryConfig,
    pub security: SecurityConfig,
}

#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    pub log_level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SerialDeviceConfig {
    pub device_name: String,
    pub port: String,
    pub baud_rate: u32,
    pub data_bits: u8,
    pub stop_bits: u8,
    pub parity: String,
    pub network_port: u16,
    pub network_protocol: String,
}

#[derive(Debug, Deserialize)]
pub struct NetworkConfig {
    pub discovery_port: u16,
    pub broadcast_interval: u64,
    pub max_clients: usize,
}

#[derive(Debug, Deserialize)]
pub struct DiscoveryConfig {
    pub device_id: String,
    pub device_type: String,
    pub discovery_message_lifetime: u64,
}

#[derive(Debug, Deserialize)]
pub struct SecurityConfig {
    pub enable_encryption: bool,
    pub encryption_key: String,
}

// Function to load and parse the configuration file
pub fn load_config(file_path: &str) -> IoResult<Config> {
    // Read the configuration file
    let contents = fs::read_to_string(file_path)?;

    // Parse the TOML string into the Config struct
    let config: Config = toml::from_str(&contents)
        .expect("Failed to parse configuration file");

    Ok(config)
}
