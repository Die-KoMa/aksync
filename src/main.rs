// © 2025 Maximilian Marx
// SPDX-FileContributor: Maximilian Marx
//
// SPDX-License-Identifier: EUPL-1.2

mod aktool;
mod cli;
mod komapedia;
mod model;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use env_logger::Env;

use aktool::AKToolApi;
use komapedia::wikipage;

pub(crate) const AKSYNC_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    "https://github.com/die-koma/aksync/"
);
pub(crate) const AKTOOL_ENDPOINT: &str = "https://ak.kif.rocks/KIFKoMa25/api";

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let _args = Cli::parse();

    let aktool_api = AKToolApi::new(AKTOOL_ENDPOINT.to_string()).expect("should succeed");
    let events = aktool_api.events().await?;

    for (id, ref event) in events {
        log::info!("processing event {id:?}");

        log::debug!("{}", event.wikitext());

        log::info!("{}", wikipage(id)?);
    }

    Ok(())
}
