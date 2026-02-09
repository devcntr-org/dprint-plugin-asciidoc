use crate::configuration::Configuration;
use dprint_core::configuration::NewLineKind;

/// Format AsciiDoc text according to the configuration.
/// Returns `Some(formatted_text)` if formatting changed the text, `None` otherwise.
pub fn format_text(text: &str, config: &Configuration) -> anyhow::Result<Option<String>> {
    // First, normalize all line endings to LF for consistent processing
    let mut formatted = text.replace("\r\n", "\n").replace('\r', "\n");

    // Normalize trailing whitespace on each line
    formatted = remove_trailing_whitespace(&formatted);

    // Normalize multiple consecutive blank lines to single blank line
    formatted = normalize_blank_lines(&formatted);

    // Ensure single trailing newline
    formatted = ensure_trailing_newline(&formatted);

    // Convert to target line endings as the LAST step
    formatted = convert_line_endings(&formatted, config.new_line_kind.clone());

    // Only return Some if the text actually changed
    if formatted == text {
        Ok(None)
    } else {
        Ok(Some(formatted))
    }
}

/// Convert line endings to the configured style (assumes input is already normalized to LF)
fn convert_line_endings(text: &str, new_line_kind: NewLineKind) -> String {
    let newline = match new_line_kind {
        NewLineKind::LineFeed => "\n",
        NewLineKind::CarriageReturnLineFeed => "\r\n",
        NewLineKind::Auto => "\n", // Default to LF for auto
    };

    if newline == "\n" {
        text.to_string()
    } else {
        text.replace('\n', newline)
    }
}

/// Remove trailing whitespace from each line
fn remove_trailing_whitespace(text: &str) -> String {
    text.lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Ensure the file ends with exactly one newline
fn ensure_trailing_newline(text: &str) -> String {
    let mut result = text.trim_end().to_string();
    result.push('\n');
    result
}

/// Normalize multiple consecutive blank lines to at most two newlines (one blank line)
fn normalize_blank_lines(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut blank_count = 0;

    for line in text.lines() {
        let is_blank = line.trim().is_empty();

        if is_blank {
            blank_count += 1;
            // Allow at most 1 blank line between content
            if blank_count <= 1 {
                result.push_str(line);
                result.push('\n');
            }
        } else {
            blank_count = 0;
            result.push_str(line);
            result.push('\n');
        }
    }

    // Ensure single trailing newline
    while result.ends_with("\n\n") {
        result.pop();
    }
    if !result.ends_with('\n') {
        result.push('\n');
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use dprint_core::configuration::NewLineKind;

    fn default_config() -> Configuration {
        Configuration {
            line_width: 80,
            indent_width: 2,
            use_tabs: false,
            new_line_kind: NewLineKind::LineFeed,
        }
    }

    #[test]
    fn test_no_change_for_well_formatted() {
        let input = "= Title\n\nSome paragraph text.\n";
        let config = default_config();
        let result = format_text(input, &config).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_normalize_crlf_to_lf() {
        let input = "= Title\r\n\r\nSome text.\r\n";
        let config = default_config();
        let result = format_text(input, &config).unwrap();
        assert_eq!(result, Some("= Title\n\nSome text.\n".to_string()));
    }

    #[test]
    fn test_remove_trailing_whitespace() {
        let input = "= Title   \n\nSome text.  \n";
        let config = default_config();
        let result = format_text(input, &config).unwrap();
        assert_eq!(result, Some("= Title\n\nSome text.\n".to_string()));
    }

    #[test]
    fn test_add_trailing_newline() {
        let input = "= Title\n\nSome text.";
        let config = default_config();
        let result = format_text(input, &config).unwrap();
        assert_eq!(result, Some("= Title\n\nSome text.\n".to_string()));
    }

    #[test]
    fn test_normalize_excessive_blank_lines() {
        let input = "= Title\n\n\n\n\nSome text.\n";
        let config = default_config();
        let result = format_text(input, &config).unwrap();
        assert_eq!(result, Some("= Title\n\nSome text.\n".to_string()));
    }

    #[test]
    fn test_crlf_output() {
        let input = "= Title\n\nSome text.\n";
        let config = Configuration {
            new_line_kind: NewLineKind::CarriageReturnLineFeed,
            ..default_config()
        };
        let result = format_text(input, &config).unwrap();
        assert_eq!(result, Some("= Title\r\n\r\nSome text.\r\n".to_string()));
    }
}
