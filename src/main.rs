use config::{Config, ConfigFile};

mod config;
mod utils;

fn main() {
    let config: ConfigFile = Config::des(config);
}
