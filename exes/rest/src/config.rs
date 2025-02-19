use serde::Deserialize;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

fn default_listening_address() -> SocketAddr {
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 8090))
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub listening_adress: SocketAddr,
}
impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            listening_adress: default_listening_address(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Discord {
    pub token: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct ReverseProxy {
    pub server: ServerSettings,
    pub discord: Discord,
    pub ratelimiter_address: String,
    pub ratelimiter_port: u16,
}
