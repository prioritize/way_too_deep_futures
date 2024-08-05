use std::{future::Future, time::Duration};

use color_eyre::Report;
use reqwest::Client;
use tracing::info;
use tracing_subscriber::{filter::EnvFilter, util::SubscriberInitExt};

const URL_1: &str = "https://fasterthanli.me/articles/whats-in-the-box";
const URL_2: &str = "https://fasterthanli.me/series/advent-of-code-2020/part-13";

fn type_name_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
#[tokio::main]
async fn main() -> Result<(), Report> {
    setup()?;
    let client = Client::new();
    let fut1 = fetch_thing(&client, URL_1);
    tokio::spawn(fut1);
    let fut2 = fetch_thing(&client, URL_2);
    tokio::spawn(fut2);

    tokio::time::sleep(Duration::from_secs(1)).await;
    Ok(())
}
fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    Ok(())
}
async fn fetch_thing<'a>(
    client: &'a Client,
    url: &'a str,
) -> impl Future<Output = Result<(), Report>> + 'a {
    async move {
        let res = client.get(url).send().await?.error_for_status()?;
        info!(%url, content_type = ?res.headers().get("content-type"), "Got a response!");
        Ok(())
    }
}
