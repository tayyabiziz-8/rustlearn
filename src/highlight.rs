// A small, dependency-free Rust syntax highlighter.
//
// This is intentionally not a full lexer for the whole language grammar —
// it's tuned for the kind of teaching snippets used on this site, and it
// errs toward "looks right for valid Rust" rather than handling every edge
// case (raw strings, byte strings, nested block comments, etc. are treated
// as plain text rather than mis-highlighted).

const KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "dyn", "else", "enum",
    "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop",
    "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self",
    "static", "struct", "super", "trait", "true", "type", "unsafe", "use",
    "where", "while", "async", "await", "yield", "union",
];

const PRIMITIVE_TYPES: &[&str] = &[
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64",
    "u128", "usize", "f32", "f64", "bool", "char", "str",
];

fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}
fn is_ident_continue(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

use crate::util::escape_html;

fn span(class: &str, text: &str) -> String {
    if class.is_empty() {
        escape_html(text)
    } else {
        format!("<span class=\"tk-{}\">{}</span>", class, escape_html(text))
    }
}

/// Highlight a block of Rust source code and return an HTML string
/// (no surrounding <pre>/<code> tags — caller wraps it).
pub fn highlight_rust(code: &str) -> String {
    let chars: Vec<char> = code.chars().collect();
    let n = chars.len();
    let mut i = 0;
    let mut out = String::new();

    while i < n {
        let c = chars[i];

        // Line comment
        if c == '/' && i + 1 < n && chars[i + 1] == '/' {
            let start = i;
            while i < n && chars[i] != '\n' {
                i += 1;
            }
            out.push_str(&span("cm", &chars[start..i].iter().collect::<String>()));
            continue;
        }

        // Block comment (non-nested)
        if c == '/' && i + 1 < n && chars[i + 1] == '*' {
            let start = i;
            i += 2;
            while i + 1 < n && !(chars[i] == '*' && chars[i + 1] == '/') {
                i += 1;
            }
            i = (i + 2).min(n);
            out.push_str(&span("cm", &chars[start..i].iter().collect::<String>()));
            continue;
        }

        // Attribute: #[...] or #![...]
        if c == '#' && i + 1 < n && (chars[i + 1] == '[' || chars[i + 1] == '!') {
            let start = i;
            let mut j = i + 1;
            if j < n && chars[j] == '!' {
                j += 1;
            }
            if j < n && chars[j] == '[' {
                let mut depth = 0i32;
                while j < n {
                    if chars[j] == '[' {
                        depth += 1;
                    } else if chars[j] == ']' {
                        depth -= 1;
                        if depth == 0 {
                            j += 1;
                            break;
                        }
                    }
                    j += 1;
                }
                out.push_str(&span("at", &chars[start..j].iter().collect::<String>()));
                i = j;
                continue;
            }
        }

        // String literal "..."
        if c == '"' {
            let start = i;
            i += 1;
            while i < n && chars[i] != '"' {
                if chars[i] == '\\' && i + 1 < n {
                    i += 2;
                } else {
                    i += 1;
                }
            }
            i = (i + 1).min(n);
            out.push_str(&span("st", &chars[start..i].iter().collect::<String>()));
            continue;
        }

        // Char literal 'x' / '\n' vs lifetime 'a
        if c == '\'' {
            // Try to match a char literal: 'c' or '\x'
            let mut j = i + 1;
            if j < n && chars[j] == '\\' {
                j += 1;
                if j < n {
                    j += 1;
                }
                if j < n && chars[j] == '\'' {
                    j += 1;
                    out.push_str(&span("st", &chars[i..j].iter().collect::<String>()));
                    i = j;
                    continue;
                }
            } else if j < n && j + 1 < n && chars[j + 1] == '\'' && chars[j] != '\'' {
                j += 2;
                out.push_str(&span("st", &chars[i..j].iter().collect::<String>()));
                i = j;
                continue;
            }
            // Otherwise: lifetime, e.g. 'a, 'static
            let start = i;
            i += 1;
            while i < n && is_ident_continue(chars[i]) {
                i += 1;
            }
            out.push_str(&span("lt", &chars[start..i].iter().collect::<String>()));
            continue;
        }

        // Number literal
        if c.is_ascii_digit() {
            let start = i;
            while i < n
                && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '.')
            {
                // avoid swallowing a method call like 5.clone() — stop at '.' followed by alpha that starts a method
                if chars[i] == '.' {
                    let next_is_digit = i + 1 < n && chars[i + 1].is_ascii_digit();
                    if !next_is_digit {
                        break;
                    }
                }
                i += 1;
            }
            out.push_str(&span("nm", &chars[start..i].iter().collect::<String>()));
            continue;
        }

        // Identifier / keyword / type / macro / function-call
        if is_ident_start(c) {
            let start = i;
            while i < n && is_ident_continue(chars[i]) {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();

            // macro!
            if i < n && chars[i] == '!' && word != "self" {
                i += 1;
                out.push_str(&span("mc", &format!("{}!", word)));
                continue;
            }

            if KEYWORDS.contains(&word.as_str()) {
                out.push_str(&span("kw", &word));
                continue;
            }
            if PRIMITIVE_TYPES.contains(&word.as_str()) {
                out.push_str(&span("ty", &word));
                continue;
            }
            // Heuristic: PascalCase / single-uppercase-letter identifiers are types/generics
            let first = word.chars().next().unwrap();
            if first.is_uppercase() {
                out.push_str(&span("ty", &word));
                continue;
            }
            // function/method call: identifier directly followed by '('
            let mut k = i;
            while k < n && chars[k] == ' ' {
                k += 1;
            }
            if k < n && chars[k] == '(' {
                out.push_str(&span("fn", &word));
                continue;
            }
            out.push_str(&escape_html(&word));
            continue;
        }

        // Operators / punctuation worth calling out
        let two: String = if i + 1 < n {
            [c, chars[i + 1]].iter().collect()
        } else {
            String::new()
        };
        let three: String = if i + 2 < n {
            [c, chars[i + 1], chars[i + 2]].iter().collect()
        } else {
            String::new()
        };
        if three == "..=" {
            out.push_str(&span("op", &three));
            i += 3;
            continue;
        }
        if ["->", "=>", "::", "&&", "||", "==", "!=", "<=", ">=", ".."].contains(&two.as_str()) {
            out.push_str(&span("op", &two));
            i += 2;
            continue;
        }
        if c == '&' || c == '*' {
            out.push_str(&span("op", &c.to_string()));
            i += 1;
            continue;
        }

        // default: pass through
        out.push_str(&escape_html(&c.to_string()));
        i += 1;
    }

    out
}
