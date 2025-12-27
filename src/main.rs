mod error;
mod par;
mod single;
mod utils;

use std::{path::PathBuf, time::Duration};

use clap::Parser;
use color_eyre::owo_colors::OwoColorize;
use isahc::{
    Request, RequestExt,
    config::Configurable,
    http::{StatusCode, header},
};

use error::{Error, Result};
/// Parallel downloader
#[derive(Debug, clap::Parser)]
#[command(about)]
struct Cli {
    url: String,

    /// Output file path
    #[arg(short = 'O', long)]
    output_file: Option<PathBuf>,

    /// Output directory path
    #[arg(short = 'D', long)]
    output_dir: Option<PathBuf>,

    /// Force single-threaded download
    #[arg(short, long, default_value_t = false)]
    single: bool,

    /// Amount of blocks
    #[arg(short, long)]
    nblocks: Option<usize>,
}

#[compio::main]
async fn main() -> crate::Result<()> {
    let cli = Cli::parse();

    color_eyre::install()?;

    if cli.single || !accept_ranges(&cli.url).await? {
        println!("Download in {} mode", "single-thread".purple());

        single::execute(cli.url, cli.output_file, cli.output_dir).await?;
    } else {
        println!("Download in {} mode", "parallel".purple());

        todo!();
    }

    Ok(())
}

const BUFFER_SIZE: usize = 65536;
const TIMEOUT: Duration = Duration::from_secs(5);

async fn accept_ranges(url: &str) -> crate::Result<bool> {
    let resp = Request::head(url)
        .timeout(TIMEOUT)
        .body(())?
        .send_async()
        .await?;

    if let Some(val) = resp.headers().get(header::ACCEPT_RANGES)
        && let Ok(val_str) = val.to_str()
    {
        Ok(val_str == "bytes")
    } else {
        let resp = Request::get(url)
            .timeout(TIMEOUT)
            .header(header::RANGE, "bytes=0-0")
            .body(())?
            .send_async()
            .await?;

        Ok(resp.status() == StatusCode::PARTIAL_CONTENT)
    }
}
