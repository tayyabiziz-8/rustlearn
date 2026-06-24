# rustlearn

A static site generator for a Rust learning reference, written entirely in Rust. No external dependencies. Run one command, get a complete HTML site.

---

## What it is

23 chapters covering Rust from variables to async/await, built from personal study notes. Every code example is fully worked, no ellipsis, no "fill in the rest yourself."

The generator itself is ~400 lines of Rust across four files: a Markdown parser, a Rust syntax highlighter, and a page renderer. It produces plain HTML and CSS with no JavaScript, no build pipeline, and no runtime.

---

## Project structure

```
├── Cargo.toml
├── src/
│   ├── main.rs          — orchestrator: loads chapters, renders pages, writes dist/
│   ├── highlight.rs     — hand-rolled Rust syntax highlighter
│   ├── markdown.rs      — Markdown → HTML converter
│   └── util.rs          — shared HTML escaping
├── content/
│   └── NN-slug.md       — one file per chapter
├── static/
│   └── style.css        — full design system, inlined into every page at build time
└── dist/                — generated output (do not edit directly)
```

---

## Requirements

- Rust + Cargo ([rustup.rs](https://rustup.rs))
- Nothing else

---

## Running it

```bash
cargo run
```

Reads every file in `content/`, writes a complete site to `dist/`. Takes under a second.

To view locally:

```bash
cd dist && python3 -m http.server 8080
```

Then open `http://localhost:8080`.

---

## Adding a chapter

1. Create `content/24-your-topic.md`:

```
# Your Topic Title
desc: One sentence shown on the home page card.

Your content here, in Markdown.
```

2. Add the slug to the `MANIFEST` array in `src/main.rs`:

```rust
("Your Group", &["24-your-topic"]),
```

3. Run `cargo run`.

---

## Editing content

Edit the `.md` file in `content/`, run `cargo run`. The entire site regenerates.

Markdown supported: `# headings`, ` ```rust ``` ` fenced code blocks, `- lists`, `> blockquotes`, `**bold**`, `*italic*`, `` `inline code` ``, `[links](url)`, `---` horizontal rules.

---

## Chapters

| # | Title | Group |
|---|-------|-------|
| 01 | Variables & Basic Types | Foundations |
| 02 | Flow Control | Foundations |
| 03 | Functions | Foundations |
| 04 | Ownership | Ownership & Borrowing |
| 05 | References & Borrowing | Ownership & Borrowing |
| 06 | Strings & Slices | Ownership & Borrowing |
| 07 | Arrays | Collections |
| 08 | Vectors | Collections |
| 09 | Tuples | Collections |
| 10 | HashMap | Collections |
| 11 | Structs | Custom Types & Patterns |
| 12 | Enums | Custom Types & Patterns |
| 13 | Pattern Matching | Custom Types & Patterns |
| 14 | Methods & Associated Functions | Custom Types & Patterns |
| 15 | Generics | Generics & Abstraction |
| 16 | Traits | Generics & Abstraction |
| 17 | Type Conversion | Generics & Abstraction |
| 18 | Lifetimes | Generics & Abstraction |
| 19 | Crates & Modules | Project Structure |
| 20 | Result & Panic | Error Handling |
| 21 | Functional Programming | Advanced Rust |
| 22 | Smart Pointers | Advanced Rust |
| 23 | Async / Await | Advanced Rust |
