use crate::highlight::highlight_rust;
use crate::util::escape_html;

fn find_close(chars: &[char], start: usize, target: char) -> Option<usize> {
    let mut i = start;
    while i < chars.len() {
        if chars[i] == target {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn find_close_seq(chars: &[char], start: usize, seq: &[char]) -> Option<usize> {
    let mut i = start;
    while i + seq.len() <= chars.len() {
        if &chars[i..i + seq.len()] == seq {
            return Some(i);
        }
        i += 1;
    }
    None
}

/// Inline-level formatting: `code`, **bold**, *italic*, [text](url)
fn inline(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut i = 0;
    let mut out = String::new();

    while i < n {
        let c = chars[i];

        if c == '`' {
            if let Some(end) = find_close(&chars, i + 1, '`') {
                let code: String = chars[i + 1..end].iter().collect();
                out.push_str(&format!("<code>{}</code>", escape_html(&code)));
                i = end + 1;
                continue;
            }
        }

        if c == '*' && i + 1 < n && chars[i + 1] == '*' {
            if let Some(end) = find_close_seq(&chars, i + 2, &['*', '*']) {
                let text: String = chars[i + 2..end].iter().collect();
                out.push_str(&format!("<strong>{}</strong>", inline(&text)));
                i = end + 2;
                continue;
            }
        }

        if c == '*' {
            if let Some(end) = find_close(&chars, i + 1, '*') {
                let text: String = chars[i + 1..end].iter().collect();
                out.push_str(&format!("<em>{}</em>", inline(&text)));
                i = end + 1;
                continue;
            }
        }

        if c == '[' {
            if let Some(close_bracket) = find_close(&chars, i + 1, ']') {
                if close_bracket + 1 < n && chars[close_bracket + 1] == '(' {
                    if let Some(close_paren) = find_close(&chars, close_bracket + 2, ')') {
                        let text: String = chars[i + 1..close_bracket].iter().collect();
                        let url: String =
                            chars[close_bracket + 2..close_paren].iter().collect();
                        out.push_str(&format!(
                            "<a href=\"{}\">{}</a>",
                            escape_html(&url),
                            escape_html(&text)
                        ));
                        i = close_paren + 1;
                        continue;
                    }
                }
            }
        }

        out.push_str(&escape_html(&c.to_string()));
        i += 1;
    }

    out
}

/// Convert a block of authored Markdown into HTML.
/// Supports: #/##/### headings, fenced code blocks, blockquotes (>),
/// unordered lists (-), horizontal rules (---), and inline formatting.
pub fn markdown_to_html(src: &str) -> String {
    let lines: Vec<&str> = src.lines().collect();
    let mut out = String::new();
    let mut i = 0;
    let mut para_buf: Vec<&str> = Vec::new();

    macro_rules! flush_para {
        () => {
            if !para_buf.is_empty() {
                let joined = para_buf.join(" ");
                out.push_str(&format!("<p>{}</p>\n", inline(&joined)));
                para_buf.clear();
            }
        };
    }

    while i < lines.len() {
        let line = lines[i];

        if line.trim().is_empty() {
            flush_para!();
            i += 1;
            continue;
        }

        if let Some(rest) = line.strip_prefix("```") {
            flush_para!();
            let lang = rest.trim().to_string();
            i += 1;
            let start = i;
            while i < lines.len() && !lines[i].starts_with("```") {
                i += 1;
            }
            let code = lines[start..i].join("\n");
            if i < lines.len() {
                i += 1; // skip closing fence
            }
            let highlighted = if lang.is_empty() || lang == "rust" {
                highlight_rust(&code)
            } else {
                escape_html(&code)
            };
            let label = if lang.is_empty() { "rust" } else { &lang };
            out.push_str(&format!(
                "<div class=\"code-block\"><div class=\"code-bar\"><span class=\"code-dot\"></span><span class=\"code-lang\">{}</span></div><pre><code>{}</code></pre></div>\n",
                escape_html(label),
                highlighted
            ));
            continue;
        }

        if let Some(rest) = line.strip_prefix("### ") {
            flush_para!();
            out.push_str(&format!("<h3>{}</h3>\n", inline(rest)));
            i += 1;
            continue;
        }
        if let Some(rest) = line.strip_prefix("## ") {
            flush_para!();
            out.push_str(&format!("<h2>{}</h2>\n", inline(rest)));
            i += 1;
            continue;
        }
        if let Some(rest) = line.strip_prefix("# ") {
            flush_para!();
            out.push_str(&format!("<h2>{}</h2>\n", inline(rest)));
            i += 1;
            continue;
        }

        if line.starts_with("> ") {
            flush_para!();
            let mut buf = Vec::new();
            while i < lines.len() && lines[i].starts_with("> ") {
                buf.push(&lines[i][2..]);
                i += 1;
            }
            out.push_str(&format!(
                "<blockquote><p>{}</p></blockquote>\n",
                inline(&buf.join(" "))
            ));
            continue;
        }

        if line.starts_with("- ") {
            flush_para!();
            out.push_str("<ul>\n");
            while i < lines.len() && lines[i].starts_with("- ") {
                out.push_str(&format!("<li>{}</li>\n", inline(&lines[i][2..])));
                i += 1;
            }
            out.push_str("</ul>\n");
            continue;
        }

        if line.trim() == "---" {
            flush_para!();
            out.push_str("<hr/>\n");
            i += 1;
            continue;
        }

        para_buf.push(line);
        i += 1;
    }
    flush_para!();

    out
}
