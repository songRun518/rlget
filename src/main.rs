mod download;

use clap::Parser;
use colored::Colorize;

#[derive(Debug, clap::Parser)]
/// RLget
struct Cli {
    /// Number of download threads
    #[arg(short, long, default_value_t = 2020)]
    threads: u64,

    /// The amount of memory for each thread to chunk request by in KB
    #[arg(long = "mem", default_value_t = 256)]
    memory: u64,

    #[arg(long)]
    /// The output file name [default: value at end of url after /]
    filename: Option<String>,

    url: String,
}

fn main() {
    let cli = Cli::parse();
    let filename = cli.filename.unwrap_or(filename(&cli.url));

    println!("threads: {}", cli.threads);
    println!("url: {}", cli.url);
    println!("memory: {}", cli.memory);
    println!("filename: {}\n", filename);

    let download = download::Download {
        threads: cli.threads,
        url: cli.url,
        memory: cli.memory,
        filename,

        ..Default::default()
    };

    download.get();
}

fn filename(url: &str) -> String {
    url.rfind('/')
        .and_then(|idx| {
            let filename = url[idx + 1..].to_string();

            if filename.is_empty() {
                None
            } else {
                Some(filename)
            }
        })
        .unwrap_or_else(|| panic!("{}", "Failed to parse filename from url".red().bold()))
}

#[cfg(test)]
mod tests {
    use super::filename;

    #[test]
    fn test_simple_filename() {
        assert_eq!(filename("https://example.com/file.txt"), "file.txt");
    }

    #[test]
    fn test_relative_path() {
        assert_eq!(filename("/docs/readme.md"), "readme.md");
    }

    #[test]
    fn test_root_only() {
        // "/": after '/' is empty → panic
        let result = std::panic::catch_unwind(|| filename("/"));
        assert!(result.is_err());
    }

    #[test]
    fn test_url_ending_with_slash() {
        // "https://example.com/": after last '/' is empty → panic
        let result = std::panic::catch_unwind(|| filename("https://example.com/"));
        assert!(result.is_err());
    }

    #[test]
    fn test_no_slash_at_all() {
        // No '/' → rfind returns None → panic
        let result = std::panic::catch_unwind(|| filename("no-slash"));
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_string() {
        let result = std::panic::catch_unwind(|| filename(""));
        assert!(result.is_err());
    }

    #[test]
    fn test_trailing_slash_after_file() {
        // This case doesn't exist — trailing slash means no filename
        // e.g., "/a/b/" → ""
        let result = std::panic::catch_unwind(|| filename("/a/b/"));
        assert!(result.is_err());
    }

    #[test]
    fn test_filename_is_only_slash_in_middle() {
        assert_eq!(filename("a/b"), "b");
    }

    #[test]
    fn test_single_char_filename() {
        assert_eq!(filename("/x"), "x");
    }
}
