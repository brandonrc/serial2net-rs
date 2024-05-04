use tokio_serial::{SerialStream, SerialPortBuilderExt};
use tokio::io::AsyncReadExt;
use std::path::Path;

pub struct AsyncSerialConnection {
    device_path: String,
    baud_rate: u32,
    port: Option<SerialStream>,
}

impl AsyncSerialConnection {
    pub fn new(device_path: &str, baud_rate: u32) -> Self {
        AsyncSerialConnection {
            device_path: device_path.to_string(),
            baud_rate,
            port: None,
        }
    }

    async fn ensure_port_open(&mut self) -> Result<(), tokio_serial::Error> {
        if self.port.is_none() || self.port.as_ref().unwrap().bytes_to_read().await.is_err() {
            let port = tokio_serial::new(&self.device_path, self.baud_rate)
                .open_native_async()?;
            self.port = Some(port);
        }
        Ok(())
    }

    pub async fn read_data(&mut self) -> Result<Vec<u8>, tokio_serial::Error> {
        self.ensure_port_open().await?;

        let mut buffer = vec![0; 1024];
        self.port.as_mut().unwrap().read(&mut buffer).await?;
        Ok(buffer)
    }
}
