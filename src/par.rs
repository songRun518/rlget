// use std::path::PathBuf;

// use color_eyre::owo_colors::OwoColorize;
// use indicatif::{ProgressBar, ProgressStyle};
// use tokio::{fs::File, io::AsyncWriteExt};

// pub struct Config {
//     pub url: String,
//     pub output_file: Option<PathBuf>,
//     pub output_dir: Option<PathBuf>,
//     pub nblocks: Option<usize>,
// }

// pub async fn execute(config: Config) -> crate::Result<()> {
//     let url = config.url;
//     let output_file = config.output_file;
//     let output_dir = config.output_dir;
//     let nblocks = config.nblocks;

//     let filename = crate::utils::filename(&url)?;
//     let filepath = output_file.unwrap_or_else(|| {
//         output_dir
//             .map(|dir| dir.join(filename))
//             .unwrap_or_else(|| PathBuf::from(filename))
//     });

//     let mut file = File::options()
//         .create(true)
//         .truncate(true)
//         .write(true)
//         .open(&filepath)
//         .await?;

//     let client = reqwest::Client::new();
//     let mut resp = client.get(url).send().await?;

//     let total_size = resp.content_length().unwrap_or(0);
//     let pb = ProgressBar::new(total_size);
//     pb.set_style(
//         ProgressStyle::default_bar()
//             .template("{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes}")?
//             .progress_chars("#>-"),
//     );

//     while let Some(chunk) = resp.chunk().await? {
//         pb.inc(chunk.len() as u64);
//         file.write_all(&chunk).await?;
//     }

//     pb.finish();
//     println!("Saved to {}", filepath.display().purple());

//     Ok(())
// }
