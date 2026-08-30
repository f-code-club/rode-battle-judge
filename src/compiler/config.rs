use serde::Deserialize;

const fn default_port() -> u16 {
    3002
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,
}

impl Config {
    pub fn new() -> color_eyre::Result<Self> {
        let cfg = ::config::Config::builder()
            .add_source(
                ::config::Environment::default()
                    .try_parsing(true)
                    .separator("__"),
            )
            .build()?
            .try_deserialize()?;

        Ok(cfg)
    }
}
