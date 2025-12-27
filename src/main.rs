mod error;
mod par;
mod single;
mod utils;

use std::path::PathBuf;

use clap::Parser;
use color_eyre::owo_colors::OwoColorize;

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

fn main() -> crate::Result<()> {
    let cli = Cli::parse();

    color_eyre::install()?;

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    if cli.single || !accept_ranges(&cli.url)? {
        println!("Download in {} mode", "single-thread".purple());

        rt.block_on(single::execute(cli.url, cli.output_file, cli.output_dir))?;
    } else {
        println!("Download in {} mode", "parallel".purple());

        todo!();

        // let config = par::Config {
        //     url: cli.url,
        //     output_file: cli.output_file,
        //     output_dir: cli.output_dir,
        //     nblocks: cli.nblocks,
        // };
        // rt.block_on(par::execute(config))?;
    }

    Ok(())
}

fn accept_ranges(url: &str) -> crate::Result<bool> {
    let client = reqwest::blocking::Client::new();

    let resp = client.head(url).send()?;

    if let Some(val) = resp.headers().get(reqwest::header::ACCEPT_RANGES)
        && let Ok(val) = val.to_str()
    {
        Ok(val == "bytes")
    } else {
        let resp = client
            .get(url)
            .header(reqwest::header::RANGE, "bytes=0-0")
            .send()?;

        Ok(resp.status() == reqwest::StatusCode::PARTIAL_CONTENT)
    }
}
