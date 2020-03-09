use config::{ConfigError, Config, File, Environment};

#[derive(Debug, Deserialize)]
pub struct Settings {
    webhook_url: String,
    rpc_socket: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Read configurations
        s.merge(File::with_name("/etc/yamori.conf").required(false))?;
        s.merge(
            glob::glob("/etc/yamori.conf.d/*")
                .unwrap()
                .map(|path| File::from(path.unwrap()).required(false))
                .collect::<Vec<_>>())?;
        s.merge(Environment::with_prefix("YAMORI"))?;

        s.try_into()
    }
}
