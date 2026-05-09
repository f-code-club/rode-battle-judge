use serde::Deserialize;

fn default_amqp_url() -> String {
    "amqp://127.0.0.1:5672/%2f".to_string()
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_amqp_url")]
    pub amqp_url: String,
}
