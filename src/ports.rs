use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use tokio::net::TcpListener;

const TO_TRY: [SocketAddr; 2] = [
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3390),
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3389),
];

pub async fn discover_free_port() -> Option<u16> {
    match TcpListener::bind(&TO_TRY[..]).await {
        Ok(l) => Some(l.local_addr().unwrap().port()),
        Err(_) => None 
    }
}
