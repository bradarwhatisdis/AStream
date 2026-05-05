use tokio::net::UdpSocket;
use tokio::time::{sleep, Duration};

pub struct DeviceInfo {
    pub name: String,
    pub port: u16,
}

impl Default for DeviceInfo {
    fn default() -> Self {
        Self {
            name: "AStream PC".to_string(),
            port: 8080,
        }
    }
}

pub async fn start_discovery_server(info: DeviceInfo) -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:5353").await?;
    socket.set_broadcast(true)?;
    
    println!("Device discovery started on port 5353");
    
    let mut buf = [0u8; 1024];
    
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        let msg = String::from_utf8_lossy(&buf[..len]);
        
        if msg.trim() == "ASTREAM_DISCOVER" {
            let response = format!("ASTREAM_RESPONSE {} {}", info.name, info.port);
            socket.send_to(response.as_bytes(), addr).await?;
            println!("Responded to discovery request from {}", addr);
        }
    }
}

pub async fn discover_devices() -> Vec<(String, String, u16)> {
    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    socket.set_broadcast(true).unwrap();
    
    let broadcast_addr = "255.255.255.255:5353";
    let discover_msg = "ASTREAM_DISCOVER";
    
    socket.send_to(discover_msg.as_bytes(), broadcast_addr).await.unwrap();
    println!("Sent discovery broadcast");
    
    let mut buf = [0u8; 1024];
    let mut devices = Vec::new();
    
    // Wait for responses (with timeout)
    tokio::select! {
        result = socket.recv_from(&mut buf) => {
            if let Ok((len, addr)) = result {
                let msg = String::from_utf8_lossy(&buf[..len]);
                if msg.starts_with("ASTREAM_RESPONSE") {
                    let parts: Vec<&str> = msg.split_whitespace().collect();
                    if parts.len() >= 3 {
                        let name = parts[1].to_string();
                        let port: u16 = parts[2].parse().unwrap_or(8080);
                        devices.push((addr.ip().to_string(), name, port));
                    }
                }
            }
        }
        _ = sleep(Duration::from_secs(2)) => {
            println!("Discovery timeout");
        }
    }
    
    devices
}
