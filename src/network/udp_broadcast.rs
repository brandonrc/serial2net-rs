use tokio::net::UdpSocket;

async fn start_udp_broadcaster(addr: &str, broadcast_addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind(addr).await?;
    println!("UDP Broadcaster bound to {}", addr);
    socket.set_broadcast(true)?;

    let msg = b"Hello from UDP broadcaster!";
    loop {
        socket.send_to(msg, broadcast_addr).await?;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}