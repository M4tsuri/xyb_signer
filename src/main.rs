use chrono::Duration;
use error::SignerError;
use reqwest::Client;
use wizard::setup_wizard;

mod signer;
mod login;
mod error;
mod secret;
mod utils;
mod wizard;
mod api;

const CONFIG_PATH: &'static str = "./config.json";

lazy_static::lazy_static! {
    pub static ref CLIENT: Client = reqwest::Client::new();
}

#[tokio::main]
async fn main() {
    match main_loop().await {
        Ok(()) => {},
        Err(e) => println!("[ERROR] {}", e)
    }
}

async fn main_loop() -> Result<(), SignerError> {
    let times = usize::from_str_radix(
        &std::env::args().nth(1).get_or_insert("1".into()),
        10
    ).or(Err(SignerError::ConfigError("arg must be an integer")))?;

    let mut interval_timer = tokio::time::interval(
        Duration::days(1).to_std().unwrap()
    );

    for _ in 0..times {
        interval_timer.tick().await;
        let ctx = setup_wizard(CONFIG_PATH).await?;
        signer::signer(ctx).await?;
        println!("");
    };

    Ok(())
}
