pub fn sanitize_filename(name: &str) -> String {
    // Remove characters that are invalid in file names
    let invalid_chars = ['\\', '/', ':', '*', '?', '"', '<', '>', '|'];
    let mut sanitized = name
        .chars()
        .filter(|c| !invalid_chars.contains(c) && !c.is_control())
        .collect::<String>();

    // Fallback if name becomes empty
    if sanitized.is_empty() {
        sanitized = "song".to_string();
    }

    sanitized
}
