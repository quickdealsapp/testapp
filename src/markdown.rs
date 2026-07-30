use pulldown_cmark::{html, Options, Parser};

/// Render markdown source into an HTML fragment.
pub fn to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let parser = Parser::new_ext(markdown, options);
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_headings_and_inline_code() {
        let html = to_html("# Title\n\nsome `code` and ~~strike~~");
        assert!(html.contains("<h1>Title</h1>"));
        assert!(html.contains("<code>code</code>"));
        assert!(html.contains("<del>strike</del>"));
    }

    #[test]
    fn renders_tables() {
        let html = to_html("| a | b |\n| - | - |\n| 1 | 2 |");
        assert!(html.contains("<table>"));
    }
}
