use std::env;
use std::net::IpAddr;
use tracing::info;
use tracing_subscriber::fmt::format;

pub fn get_address() -> (IpAddr, u16) {
    let env_host = env::var_os("HOST").expect("HOST is undefined.");
    let ip_addr = env_host
        .into_string()
        .expect("HOST is invalid.")
        .parse::<IpAddr>()
        .expect("HOST is invalid.");

    let env_port = env::var_os("PORT").expect("PORT is undefined.");
    let port = env_port
        .into_string()
        .expect("PORT is invalid.")
        .parse::<u16>()
        .expect("PORT is invalid.");
    let url_message = format!("executed: initializing server url = {}:{}", &ip_addr, &port);
    info!(url_message);
    (ip_addr, port)
}