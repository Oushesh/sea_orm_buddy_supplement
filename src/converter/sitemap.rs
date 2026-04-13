use ammonia::Builder;
use html2md::parse_html;
use std::collections::HashSet;

pub fn to_markdown(raw_html: &str) -> String {
    // 1. Define "Junk Tags" to remove before conversion
    // This saves tokens and removes non-content distraction
    let mut clean_list = HashSet::new();
    clean_list.insert("script");
    clean_list.insert("style");
    clean_list.insert("nav");
    clean_list.insert("footer");
    clean_list.insert("header");
    clean_list.insert("aside");
    clean_list.insert("iframe");
    clean_list.insert("noscript");

    // 2. Clean the HTML (Sanitization)
    // We want to keep the structure (h1, p, ul) but strip the logic (js)
    let cleaner = Builder::default()
        .rm_tags(clean_list)
        .clean(raw_html)
        .to_string();

    // 3. Convert the cleaned HTML to Markdown
    let markdown = parse_html(&cleaner);

    // 4. Post-processing: Basic cleanup of excessive whitespace
    markdown.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}