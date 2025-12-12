use rlget::filename;

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
