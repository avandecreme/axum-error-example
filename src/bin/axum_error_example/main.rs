use anyhow::Context;
use env_logger::Env;
use log::info;

use crate::business_logic::job_runner;

mod api;
pub(crate) mod business_logic;
pub(crate) mod db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    tokio::spawn(job_runner());

    let app = api::router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .context("Could not start listening on port 3000")?;

    info!("Starting listening on port 3000");

    axum::serve(listener, app).await?;

    Ok(())
}
