extern crate undermoon;
extern crate tokio;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate config;

use std::env;
use undermoon::proxy::executor::SharedForwardHandler;
use undermoon::proxy::service::{ServerProxyService, ServerProxyConfig};

fn gen_conf() -> ServerProxyConfig {
    let conf_file_path = env::args().skip(1).next().unwrap_or("server-proxy.toml".to_string());

    let mut s = config::Config::new();
    s.merge(config::File::with_name(&conf_file_path))
        .map(|_| ())
        .unwrap_or_else(|e| warn!("failed to read config file: {:?}", e));
    // e.g. UNDERMOON_ADDRESS='127.0.0.1:5299'
    s.merge(config::Environment::with_prefix("undermoon"))
        .map(|_| ())
        .unwrap_or_else(|e| warn!("failed to read address from env vars {:?}", e));

    ServerProxyConfig{
        address: s.get::<String>("address").unwrap_or("127.0.0.1:5299".to_string())
    }
}

fn main() {
    env_logger::init();

    let config = gen_conf();
    let forward_handler = SharedForwardHandler::new(config.address.clone());
    let server = ServerProxyService::new(config, forward_handler);

    tokio::run(server.run());
}