mod highlight;
mod markdown;
mod util;

use markdown::markdown_to_html;
use std::fs;
use std::path::Path;

const STYLE: &str = include_str!("../static/style.css");

/// (group title, slugs in that group, in display order)
const MANIFEST: &[(&str, &[&str])] = &[
    (
        "Foundations",
        &["01-variables", "02-flow-control", "03-functions"],
    ),
    (
        "Ownership & Borrowing",
        &["04-ownership", "05-references-and-borrowing", "06-strings-and-slices"],
    ),
    (
        "Collections",
        &["07-arrays", "08-vectors", "09-tuples", "10-hashmap"],
    ),
    (
        "Custom Types & Patterns",
        &["11-structs", "12-enums", "13-pattern-matching", "14-methods"],
    ),
    (
        "Generics & Abstraction",
        &["15-generics", "16-traits", "17-type-conversion", "18-lifetimes"],
    ),
    ("Project Structure", &["19-crates-and-modules"]),
    ("Error Handling", &["20-result-and-panic"]),
    (
        "Advanced Rust",
        &["21-functional-programming", "22-smart-pointers", "23-async-await"],
    ),
];

struct Chapter {
    slug: String,
    number: String,
    title: String,
    description: String,
    body_html: String,
    group: String,
}

fn parse_chapter(slug: &str, group: &str, raw: &str) -> Chapter {
    let number = slug.split('-').next().unwrap_or("00").to_string();
    let lines: Vec<&str> = raw.lines().collect();
    let title = lines
        .first()
        .and_then(|l| l.strip_prefix("# "))
        .unwrap_or(slug)
        .to_string();

    let mut body_start = 1;
    let mut description = String::new();
    if let Some(second) = lines.get(1) {
        if let Some(d) = second.strip_prefix("desc: ") {
            description = d.to_string();
            body_start = 2;
        }
    }

    let body_src = lines[body_start.min(lines.len())..].join("\n");
    let body_html = markdown_to_html(&body_src);

    Chapter {
        slug: slug.to_string(),
        number,
        title,
        description,
        body_html,
        group: group.to_string(),
    }
}

fn load_chapters(content_dir: &Path) -> Vec<Chapter> {
    let mut chapters = Vec::new();
    for (group, slugs) in MANIFEST {
        for slug in *slugs {
            let path = content_dir.join(format!("{slug}.md"));
            let raw = fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("failed to read {path:?}: {e}"));
            chapters.push(parse_chapter(slug, group, &raw));
        }
    }
    chapters
}

fn build_sidebar(chapters: &[Chapter], active_slug: Option<&str>) -> String {
    let mut html = String::new();
    html.push_str("<nav id=\"sidebar\" class=\"sidebar\" aria-label=\"Chapters\">\n");
    html.push_str(
        "<div class=\"brand-block\"><a href=\"index.html\"><div class=\"brand-title\"><span class=\"mark\">&gt;_</span> rustlearn</div><div class=\"brand-sub\">field notes &amp; drills</div></a></div>\n",
    );

    let mut current_group = "";
    for ch in chapters {
        if ch.group != current_group {
            if !current_group.is_empty() {
                html.push_str("</div>\n");
            }
            html.push_str(&format!(
                "<div class=\"nav-group\"><div class=\"nav-group-title\">{}</div>\n",
                ch.group
            ));
            current_group = &ch.group;
        }
        let active = if Some(ch.slug.as_str()) == active_slug {
            " active"
        } else {
            ""
        };
        html.push_str(&format!(
            "<a class=\"nav-link{active}\" href=\"{slug}.html\"><span class=\"n\">{num}</span><span>{title}</span></a>\n",
            active = active,
            slug = ch.slug,
            num = ch.number,
            title = ch.title
        ));
    }
    html.push_str("</div>\n</nav>\n");
    html
}

fn page_shell(title: &str, sidebar_html: &str, main_html: &str) -> String {
    let mut out = String::new();
    out.push_str("<!doctype html>\n<html lang=\"en\">\n<head>\n");
    out.push_str("<meta charset=\"utf-8\">\n");
    out.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
    out.push_str("<title>");
    out.push_str(title);
    out.push_str("</title>\n");
    out.push_str("<meta name=\"description\" content=\"A from-scratch Rust field guide: ownership, borrowing, generics, error handling, smart pointers, and async, with fully worked, unambiguous code examples.\">\n");
    out.push_str("<link rel=\"preconnect\" href=\"https://fonts.googleapis.com\">\n");
    out.push_str("<link rel=\"preconnect\" href=\"https://fonts.gstatic.com\" crossorigin>\n");
    out.push_str("<link href=\"https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@500;600;700&family=Source+Serif+4:opsz,wght@8..60,400;8..60,600&family=JetBrains+Mono:wght@400;500;600&display=swap\" rel=\"stylesheet\">\n");
    out.push_str("<style>\n");
    out.push_str(STYLE);
    out.push_str("\n</style>\n</head>\n<body>\n");
    out.push_str("<a class=\"skip-link\" href=\"#main\">Skip to content</a>\n");
    out.push_str("<header class=\"topbar\" aria-label=\"Site header\">\n");
    out.push_str("  <span class=\"topbar-brand\"><span class=\"mark\">&gt;_</span> rustlearn</span>\n");
    out.push_str("  <button class=\"hamburger\" aria-label=\"Toggle navigation\" aria-expanded=\"false\" onclick=\"var s=document.getElementById('sidebar');var open=s.classList.toggle('open');this.setAttribute('aria-expanded',open);\">");
    out.push_str("<span></span><span></span><span></span></button>\n");
    out.push_str("</header>\n");
    out.push_str("<div class=\"shell\">\n");
    out.push_str(sidebar_html);
    out.push_str("<main id=\"main\" class=\"main\">\n");
    out.push_str(main_html);
    out.push_str("\n<footer class=\"site-footer\">built with a hand-rolled Rust static site generator &mdash; no JS required</footer>\n");
    out.push_str("</main>\n</div>\n</body>\n</html>\n");
    out
}

fn ownership_diagram_svg() -> &'static str {
    r##"<svg class="hero-diagram" viewBox="0 0 460 360" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Diagram of a String value: a stack record holding pointer, length, and capacity, pointing to heap-allocated bytes">
  <defs>
    <marker id="arrow" markerWidth="9" markerHeight="9" refX="7" refY="3" orient="auto">
      <path d="M0,0 L7,3 L0,6 Z" fill="#3E8E86"/>
    </marker>
  </defs>
  <text x="20" y="28" font-family="JetBrains Mono, monospace" font-size="11" letter-spacing="2" fill="#7E96A3">SHEET — OWNERSHIP MODEL</text>

  <rect x="20" y="50" width="150" height="120" rx="3" fill="none" stroke="#3E8E86" stroke-width="1.4"/>
  <text x="34" y="44" font-family="JetBrains Mono, monospace" font-size="11" fill="#D79A4C">STACK</text>
  <line x1="20" y1="90" x2="170" y2="90" stroke="#21404f" stroke-width="1"/>
  <line x1="20" y1="130" x2="170" y2="130" stroke="#21404f" stroke-width="1"/>

  <text x="34" y="76" font-family="JetBrains Mono, monospace" font-size="13" fill="#DCE7EC">ptr</text>
  <text x="120" y="76" font-family="JetBrains Mono, monospace" font-size="11" fill="#7E96A3">0x7ffc..</text>

  <text x="34" y="116" font-family="JetBrains Mono, monospace" font-size="13" fill="#DCE7EC">len</text>
  <text x="120" y="116" font-family="JetBrains Mono, monospace" font-size="11" fill="#7E96A3">5</text>

  <text x="34" y="156" font-family="JetBrains Mono, monospace" font-size="13" fill="#DCE7EC">capacity</text>
  <text x="120" y="156" font-family="JetBrains Mono, monospace" font-size="11" fill="#7E96A3">5</text>

  <path d="M170,75 C 220,75 220,75 255,75" fill="none" stroke="#3E8E86" stroke-width="1.4" marker-end="url(#arrow)"/>

  <rect x="255" y="50" width="180" height="60" rx="3" fill="none" stroke="#C5511E" stroke-width="1.4" stroke-dasharray="4 3"/>
  <text x="265" y="44" font-family="JetBrains Mono, monospace" font-size="11" fill="#D79A4C">HEAP</text>

  <g font-family="JetBrains Mono, monospace" font-size="14" fill="#DCE7EC">
    <rect x="265" y="68" width="22" height="22" fill="none" stroke="#46606b"/>
    <text x="271" y="84">h</text>
    <rect x="287" y="68" width="22" height="22" fill="none" stroke="#46606b"/>
    <text x="293" y="84">e</text>
    <rect x="309" y="68" width="22" height="22" fill="none" stroke="#46606b"/>
    <text x="315" y="84">l</text>
    <rect x="331" y="68" width="22" height="22" fill="none" stroke="#46606b"/>
    <text x="337" y="84">l</text>
    <rect x="353" y="68" width="22" height="22" fill="none" stroke="#46606b"/>
    <text x="359" y="84">o</text>
  </g>

  <text x="20" y="210" font-family="JetBrains Mono, monospace" font-size="11" fill="#7E96A3">let s1 = String::from("hello");</text>
  <text x="20" y="230" font-family="JetBrains Mono, monospace" font-size="11" fill="#7E96A3">let s2 = s1;  // move — s1 no longer valid</text>

  <rect x="20" y="250" width="150" height="60" rx="3" fill="none" stroke="#46606b" stroke-width="1.2" stroke-dasharray="3 3"/>
  <text x="34" y="244" font-family="JetBrains Mono, monospace" font-size="11" fill="#46606b">s1 (moved)</text>
  <line x1="40" y1="262" x2="150" y2="298" stroke="#8F3811" stroke-width="1.4"/>
  <line x1="150" y1="262" x2="40" y2="298" stroke="#8F3811" stroke-width="1.4"/>

  <rect x="200" y="250" width="150" height="60" rx="3" fill="none" stroke="#3E8E86" stroke-width="1.4"/>
  <text x="214" y="244" font-family="JetBrains Mono, monospace" font-size="11" fill="#D79A4C">s2 (owner)</text>
  <text x="214" y="285" font-family="JetBrains Mono, monospace" font-size="13" fill="#DCE7EC">ptr ──┐</text>
  <path d="M350,280 C 380,280 250,150 290,110" fill="none" stroke="#3E8E86" stroke-width="1.2" stroke-dasharray="3 2" marker-end="url(#arrow)"/>
</svg>"##
}

fn render_home(chapters: &[Chapter]) -> String {
    let sidebar = build_sidebar(chapters, None);

    let mut groups_html = String::new();
    let mut current_group = "";
    for ch in chapters {
        if ch.group != current_group {
            if !current_group.is_empty() {
                groups_html.push_str("</div></div>\n");
            }
            groups_html.push_str(&format!(
                "<div class=\"section-block\"><h2>{}</h2><div class=\"card-grid\">\n",
                ch.group
            ));
            current_group = &ch.group;
        }
        groups_html.push_str(&format!(
            "<a class=\"chapter-card\" href=\"{slug}.html\"><div class=\"num\">SHEET {num}</div><div class=\"title\">{title}</div><div class=\"desc\">{desc}</div></a>\n",
            slug = ch.slug,
            num = ch.number,
            title = ch.title,
            desc = ch.description
        ));
    }
    groups_html.push_str("</div></div>\n");

    let first_slug = &chapters[0].slug;

    let main_html = format!(
        r#"<section class="hero">
  <div>
    <div class="hero-eyebrow">A field guide, not a wiki dump</div>
    <h1>Learn Rust the way the<br><span class="accent">borrow checker</span> thinks.</h1>
    <p>23 short, dense chapters built from real study notes — ownership, borrowing, generics, error handling, smart pointers, and async — every example fully worked so there's no ambiguity about what it does.</p>
    <a class="hero-cta" href="{first_slug}.html">Start at Sheet 01 &rarr;</a>
  </div>
  {diagram}
</section>
{groups}"#,
        first_slug = first_slug,
        diagram = ownership_diagram_svg(),
        groups = groups_html
    );

    page_shell("rustlearn", &sidebar, &main_html)
}

fn render_chapter(chapters: &[Chapter], idx: usize) -> String {
    let ch = &chapters[idx];
    let sidebar = build_sidebar(chapters, Some(&ch.slug));

    let prev = if idx > 0 {
        let p = &chapters[idx - 1];
        format!(
            "<a href=\"{slug}.html\"><span class=\"dir\">&larr; Previous</span>Sheet {num} &middot; {title}</a>",
            slug = p.slug,
            num = p.number,
            title = p.title
        )
    } else {
        "<span></span>".to_string()
    };
    let next = if idx + 1 < chapters.len() {
        let nx = &chapters[idx + 1];
        format!(
            "<a class=\"next\" href=\"{slug}.html\"><span class=\"dir\">Next &rarr;</span>Sheet {num} &middot; {title}</a>",
            slug = nx.slug,
            num = nx.number,
            title = nx.title
        )
    } else {
        "<span></span>".to_string()
    };

    let main_html = format!(
        r#"<article class="sheet">
  <div class="sheet-eyebrow">{group} &middot; Sheet {num} of {total}</div>
  <h1>{title}</h1>
  <div class="content">
{body}
  </div>
  <nav class="chapter-nav" aria-label="Chapter navigation">
    {prev}
    {next}
  </nav>
</article>"#,
        group = ch.group,
        num = ch.number,
        total = chapters.len(),
        title = ch.title,
        body = ch.body_html,
        prev = prev,
        next = next
    );

    page_shell(&format!("{} &mdash; rustlearn", ch.title), &sidebar, &main_html)
}

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let content_dir = root.join("content");
    let dist_dir = root.join("docs");

    fs::create_dir_all(&dist_dir).expect("create dist dir");

    let chapters = load_chapters(&content_dir);
    println!("Loaded {} chapters", chapters.len());

    let home_html = render_home(&chapters);
    fs::write(dist_dir.join("index.html"), home_html).expect("write index.html");

    for (idx, ch) in chapters.iter().enumerate() {
        let html = render_chapter(&chapters, idx);
        fs::write(dist_dir.join(format!("{}.html", ch.slug)), html)
            .unwrap_or_else(|e| panic!("write {}: {e}", ch.slug));
    }

    println!("Site built at {:?}", dist_dir);
}
