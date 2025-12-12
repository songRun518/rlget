pub mod download;

use colored::Colorize;

pub fn filename(url: &str) -> String {
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
