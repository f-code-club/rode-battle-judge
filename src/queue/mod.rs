mod config;

use lapin::{Connection, ConnectionProperties};

pub use config::*;

pub async fn connect() -> color_eyre::Result<Connection> {
    let config: Config = ::config::Config::builder()
        .add_source(
            ::config::Environment::default()
                .try_parsing(true)
                .separator("__"),
        )
        .build()?
        .try_deserialize()?;
    let connection = Connection::connect(
        &config.amqp_url,
        ConnectionProperties::default().enable_auto_recover(),
    )
    .await?;

    Ok(connection)
}
