use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use tokio::net::TcpListener;

const TO_TRY: [SocketAddr; 2] = [
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3389),
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3390),
];

pub async fn discover_free_port() -> Option<u16> {
    match TcpListener::bind(&TO_TRY[..]).await {
        Ok(l) => Some(l.local_addr().unwrap().port()),
        Err(_) => None 
    }
}

pub async fn wait_port_freed(port: u16) -> bool {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    let mut retries = 5;
    loop {
        match TcpListener::bind(addr).await {
            Ok(_) => return true,
            Err(_) => { eprintln!("Port still occupied, waiting... "); }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        retries -= 1;
        if retries == 0 { return false }
    }
}
