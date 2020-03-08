use std::collections::HashMap;
use std::sync::RwLock;
use config::{Config, ConfigError};

lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new(Config::default());
}

fn handle_config_errors(res: Result<(), ConfigError>) {
    if res.is_err() {
        match res.err().unwrap() {
            ConfigError::NotFound(_) => {
                
            }
        }
    }
}

fn main() {
    handle_config_errors((SETTINGS.write()?.merge(config::File::with_name("/etc/yamori.conf")));

    println!("{:?}", settings.try_into::<HashMap<String, String>>().unwrap());
}
