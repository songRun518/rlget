mod error;
mod par;
mod single;

use clap::Parser;

use error::Error;
use error::Result;

#[derive(Debug, clap::Parser)]
#[command(about)]
/// Parallel downloader
struct Cli {
    url: String,

    /// Number of blocks
    #[arg(short, long)]
    nblocks: Option<usize>,
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
        single::execute().await?;
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
