use std::{
    fs,
    net::{IpAddr, SocketAddr},
};

use serde::Deserialize;

pub struct ServerConfig {
    pub socket_addr: SocketAddr,
}

impl ServerConfig {
    pub fn get() -> Self {
        let config_toml = fs::read_to_string("elfmsg.toml").unwrap_or("".to_string());

        let config: Config = toml::from_str(&config_toml).unwrap_or_else(|err| {
            eprintln!("ConfigError: {}", err);
            eprintln!("Fallback to default config");
            Default::default()
        });

        Self::new(config)
    }

    fn new(config: Config) -> Self {
        Self {
            socket_addr: { SocketAddr::new(config.server.addr, config.server.port) },
        }
    }
}

#[derive(Deserialize, Default)]
struct Config {
    #[serde(default)]
    server: ConfigServer,
}

#[derive(Deserialize)]
struct ConfigServer {
    addr: IpAddr,
    port: u16,
}

impl Default for ConfigServer {
    fn default() -> Self {
        Self {
            addr: "127.0.0.1".parse().unwrap(),
            port: 13131,
        }
    }
}
