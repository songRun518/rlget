mod error;
mod par;
mod single;

use clap::Parser;

use error::{Error, Result};

/// Parallel downloader
#[derive(Debug, clap::Parser)]
#[command(about)]
struct Cli {
    url: String,

    /// Amount of blocks
    #[arg(short, long)]
    nblocks: Option<usize>,

    /// Force single-threaded download
    #[arg(short, long, default_value_t = false)]
    single: bool,
}

fn main() -> crate::Result<()> {
    let cli = Cli::parse();

    color_eyre::install()?;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async_main(cli))?;

    Ok(())
}

async fn async_main(cli: Cli) -> crate::Result<()> {
    if accept_ranges(&cli.url).await? {
        todo!("parallel download")
    } else {
        single::execute(&cli.url).await?;
    }

    Ok(())
}

async fn accept_ranges(url: &str) -> crate::Result<bool> {
    let client = reqwest::Client::new();

    let resp = client.head(url).send().await?;

    if let Some(val) = resp.headers().get(reqwest::header::ACCEPT_RANGES)
        && let Ok(val) = val.to_str()
    {
        Ok(val == "bytes")
    } else {
        let resp = client
            .get(url)
            .header(reqwest::header::RANGE, "bytes=0-0")
            .send()
            .await?;

        Ok(resp.status() == reqwest::StatusCode::PARTIAL_CONTENT)
    }
}
