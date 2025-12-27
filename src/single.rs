use std::path::PathBuf;

use color_eyre::{eyre::eyre, owo_colors::OwoColorize};
use indicatif::{ProgressBar, ProgressStyle};
use tokio::{fs::File, io::AsyncWriteExt};

pub struct SingleConfig {
    pub url: String,
    pub output_file: Option<PathBuf>,
    pub output_dir: Option<PathBuf>,
}

pub async fn execute(config: SingleConfig) -> crate::Result<()> {
    let url = &config.url;
    let output_file = config.output_file;
    let output_dir = config.output_dir;

    let filename = filename(url)?;
    let filepath = output_file.unwrap_or_else(|| {
        output_dir
            .map(|dir| dir.join(filename))
            .unwrap_or_else(|| PathBuf::from(filename))
    });

    let mut file = File::options()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&filepath)
        .await?;

    let client = reqwest::Client::new();
    let mut resp = client.get(url).send().await?;

    let total_size = resp.content_length().unwrap_or(0);
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes}")?
            .progress_chars("#>-"),
    );

    while let Some(chunk) = resp.chunk().await? {
        pb.inc(chunk.len() as u64);
        file.write_all(&chunk).await?;
    }

    pb.finish();
    println!("Saved to {}", filepath.display().purple());

    Ok(())
}

fn filename(url: &str) -> crate::Result<&str> {
    let err_msg = || eyre!("Failed to parse filename from {url}");

    let pos = url.rfind('/').ok_or_else(err_msg)?;
    let pat = &url[pos + 1..];

    if pat.is_empty() {
        Err(err_msg().into())
    } else {
        Ok(pat)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filename_success() -> crate::Result<()> {
        assert_eq!(
            filename("https://example.com/assets/image.png")?,
            "image.png"
        );
        assert_eq!(filename("dir/file.txt")?, "file.txt");
        assert_eq!(filename("/usr/local/bin/rustc")?, "rustc");
        Ok(())
    }

    #[test]
    fn test_filename_unicode() -> crate::Result<()> {
        assert_eq!(
            filename("https://oss/bucket/我的文档.docx")?,
            "我的文档.docx"
        );
        assert_eq!(filename("path/to/🦀_rust.rs")?, "🦀_rust.rs");
        Ok(())
    }

    #[test]
    fn test_filename_no_slash() {
        let input = "filename_only.txt";
        let res = filename(input);
        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains(input));
    }

    #[test]
    fn test_filename_trailing_slash() {
        let input = "https://example.com/folder/";
        let res = filename(input);
        assert!(res.is_err());
    }

    #[test]
    fn test_filename_with_extra_symbols() -> crate::Result<()> {
        let url = "https://example.com/data.tar.gz?version=1#top";
        assert_eq!(filename(url)?, "data.tar.gz?version=1#top");
        Ok(())
    }

    #[test]
    fn test_filename_dot_files() -> crate::Result<()> {
        assert_eq!(filename("/home/user/.gitignore")?, ".gitignore");
        Ok(())
    }
}
