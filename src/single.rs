use std::path::PathBuf;

use color_eyre::{eyre::Context, owo_colors::OwoColorize};
use indicatif::{ProgressBar, ProgressStyle};
use tokio::{fs::File, io::AsyncWriteExt, sync::mpsc};

pub async fn execute(
    url: String,
    output_file: Option<PathBuf>,
    output_dir: Option<PathBuf>,
) -> crate::Result<()> {
    let filename = crate::utils::filename(&url)?;
    let filepath = output_file.unwrap_or_else(|| {
        output_dir
            .map(|dir| dir.join(filename))
            .unwrap_or_else(|| PathBuf::from(filename))
    });

    let file = File::options()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&filepath)
        .await?;

    let client = reqwest::Client::new();
    let resp = client.get(url).send().await?;

    let total_size = resp.content_length().unwrap_or(0);
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes}")?
            .progress_chars("#>-"),
    );

    const CHANNEL_BUFFER: usize = 1000;
    let (sx, rx) = mpsc::channel::<Vec<u8>>(CHANNEL_BUFFER);

    let dh = tokio::spawn(downloader(resp, sx));
    let wh = tokio::spawn(writer(file, rx, pb));

    dh.await??;
    wh.await??;

    println!("Saved to {}", filepath.display().purple());

    Ok(())
}

async fn downloader(mut resp: reqwest::Response, sx: mpsc::Sender<Vec<u8>>) -> crate::Result<()> {
    while let Some(chunk) = resp.chunk().await? {
        sx.send(chunk.to_vec()).await.wrap_err("channel closed")?;
    }
    Ok(())
}

async fn writer(
    mut file: File,
    mut rx: mpsc::Receiver<Vec<u8>>,
    pb: ProgressBar,
) -> crate::Result<()> {
    while let Some(chunk) = rx.recv().await {
        pb.inc(chunk.len() as u64);
        file.write_all(&chunk).await?;
    }

    pb.finish();

    Ok(())
}
