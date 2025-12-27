use std::path::PathBuf;

use color_eyre::eyre::eyre;
use compio::{fs::OpenOptions, io::AsyncWriteAtExt};
use futures_util::AsyncReadExt;
use indicatif::{ProgressBar, ProgressStyle};
use isahc::{Request, RequestExt, config::Configurable};

use crate::{BUFFER_SIZE, TIMEOUT};

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

    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&filepath)
        .await?;

    let mut resp = Request::get(&url)
        .timeout(TIMEOUT)
        .body(())?
        .send_async()
        .await?;

    let resp_status = resp.status();
    if !resp_status.is_success() {
        let reason = resp_status
            .canonical_reason()
            .unwrap_or("No canonical_reason");
        return Err(eyre!("Failed to get {url}: {reason}").into());
    }

    let body = resp.body_mut();

    let pb = ProgressBar::new(body.len().unwrap_or(0));
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes}")?
            .progress_chars("#>-"),
    );

    let mut buf = vec![0u8; BUFFER_SIZE];
    let mut pos = 0;

    loop {
        let len = body.read(&mut buf).await?;

        if len == 0 {
            break;
        }

        buf.truncate(len);
        let res = file.write_all_at(buf, pos).await;
        res.0?;
        buf = res.1;

        pos += len as u64;
        pb.inc(len as u64);
    }

    Ok(())
}
