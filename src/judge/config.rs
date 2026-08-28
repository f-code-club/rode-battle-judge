use serde::Deserialize;

fn default_amqp_url() -> String {
    "amqp://127.0.0.1:5672/%2f".to_string()
}

fn default_task_queue() -> String {
    "task".to_string()
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_amqp_url")]
    pub amqp_url: String,

    #[serde(default = "default_task_queue")]
    pub task_queue: String,

    pub database_url: String,
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
