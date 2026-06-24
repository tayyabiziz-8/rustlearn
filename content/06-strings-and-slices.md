# Strings & Slices
desc: String vs &str vs string literal, UTF-8 byte indexing, and building/growing strings.

Three things in Rust are all "string-shaped," and they're easy to confuse at first:

- **`String`** - heap-allocated, owns its data, growable, mutable.
- **`&str`** ("string slice") - a *view* into UTF-8 bytes somewhere else; it doesn't own the data.
- **String literal** (`"hello"`) - baked into the binary; its type is `&'static str`, a string slice.

## Slicing a String

```rust
fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    assert_eq!(hello, "hello");
    assert_eq!(world, "world");
}
```

`&String` slices to `&str` automatically wherever a `&str` is expected - this is called **deref coercion**.

```rust
fn first_word(s: &str) -> &str {
    &s[..1]
}

fn main() {
    let s = String::from("hello world");
    let first = first_word(&s); // &String coerces to &str here
    assert_eq!(first, "h");
}
```

> Slices borrow from their source. If you call `.clear()` on the original `String` while a slice into it is still alive, that's a compile error - the slice would otherwise point at memory that's about to be wiped.

## UTF-8: indexing is by byte, not by character

A Rust `String` is UTF-8 bytes under the hood. ASCII characters take 1 byte; many others take more - so slicing at the wrong boundary panics.

```rust
fn main() {
    let s = String::from("hello, 世界");

    let h = &s[..1];
    assert_eq!(h, "h"); // 'h' is 1 byte in UTF-8

    // "世" starts at byte offset 7 and is 3 bytes long
    let world_char = &s[7..10];
    assert_eq!(world_char, "世");

    // Always iterate by character, not by raw byte index, when you're
    // not sure of the boundaries:
    for (i, c) in s.chars().enumerate() {
        if c == '世' {
            println!("'世' is character index {i}");
        }
    }
}
```

## Building and growing a String

```rust
fn main() {
    let mut s = String::with_capacity(25); // pre-allocate to avoid repeated reallocation
    s.push(','); // single char
    s.push_str(" world"); // a &str
    s += "!"; // shorthand for push_str, via the Add trait

    assert_eq!(s, ", world!");
}
```

```rust
fn main() {
    let s1 = String::from("hello, ");
    let s2 = String::from("world!");

    // `+` calls a method roughly shaped like `fn add(self, other: &str) -> String`.
    // It takes `self` by value, so s1 is MOVED here and can't be used afterward.
    let s3 = s1 + &s2;

    assert_eq!(s3, "hello, world!");
    // s1 is no longer valid; s2 is still valid (it was only borrowed)
}
```

## Converting between String and bytes

```rust
fn main() {
    let s = String::from("hello");

    let bytes_view: &[u8] = s.as_bytes(); // borrows - s is still usable
    assert_eq!(bytes_view, b"hello");

    let bytes_owned: Vec<u8> = s.into_bytes(); // consumes s, returns owned Vec<u8>
    assert_eq!(bytes_owned, vec![104, 101, 108, 108, 111]);
}
```
