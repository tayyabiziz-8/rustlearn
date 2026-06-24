# Lifetimes
desc: Lifetime annotations, why the compiler sometimes needs them, structs holding references, and 'static.

Every reference is valid for some scope - its **lifetime**. Most of the time the compiler works this out on its own (see "elision" below). Lifetime annotations only become necessary when the compiler can't infer, on its own, how the lifetime of an output relates to the lifetimes of the inputs.

Lifetimes don't change how long anything actually lives - they're a way of *describing*, to the compiler, a relationship that's already true in your code, so it can verify there's no dangling reference.

## The motivating problem

```rust
// This will NOT compile:
//
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }
//
// error[E0106]: missing lifetime specifier
// The function might return x or it might return y - the compiler can't
// tell which input the output reference is tied to, so it can't verify
// the returned reference will still be valid wherever it ends up used.
```

The fix: tell the compiler, explicitly, that the output lives exactly as long as the *shorter* of the two inputs.

```rust
// 'a is a lifetime parameter, declared like a generic type parameter.
// This signature says: "the returned &str is valid for at least as long
// as BOTH x and y are valid."
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("long string is long");
    let result;
    {
        let s2 = String::from("xyz");
        result = longest(s1.as_str(), s2.as_str());
        assert_eq!(result, "long string is long");
    } // s2 dropped here - but `result` was already used above, while s2
      // was still alive, so this compiles fine.
}
```

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("long string is long");
    let result;
    {
        let s2 = String::from("xyz");
        result = longest(s1.as_str(), s2.as_str());
        // If we tried to use `result` AFTER this inner block ends, it
        // would be a compile error: result might be borrowing s2, and
        // s2 is about to be dropped. The error surfaces here, at compile
        // time, instead of as a dangling pointer at runtime.
    }
    // println!("{result}"); // would fail to compile if uncommented
}
```

## Elision: most of the time you write nothing

The compiler applies three rules before giving up and asking for an annotation: (1) each reference parameter gets its own lifetime, (2) if there's exactly one input lifetime, it's assigned to all outputs, (3) if one parameter is `&self`/`&mut self`, its lifetime is assigned to all outputs. This covers the overwhelming majority of functions, which is why you rarely see `'a` in everyday code.

```rust
// No annotation needed: rule 2 applies (one input reference, so the
// output borrows from it).
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn main() {
    assert_eq!(first_word("hello world"), "hello");
}
```

## Structs that hold a reference

A struct can't hold a reference without naming the lifetime - Rust needs to guarantee the struct never outlives the data it's borrowing.

```rust
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn announce(&self, prefix: &str) -> &str {
        println!("{prefix}: {}", self.part);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();

    let excerpt = Excerpt { part: first_sentence };
    // `excerpt` can't outlive `novel`, because `part` borrows from it -
    // the compiler enforces that statically.

    assert_eq!(excerpt.announce("note"), "Call me Ishmael");
}
```

## `'static`: lives for the whole program

`'static` is a special lifetime meaning "valid for the entire duration of the program." String literals are `'static` because they're baked directly into the binary.

```rust
fn main() {
    let s: &'static str = "I live for the whole program";
    println!("{s}");
}
```

> `'static` is sometimes reached for as a quick fix to silence a lifetime error - but that usually just moves the problem, by forcing data to live longer than it needs to (or forcing a clone). Treat a `'static` requirement as a sign to double-check whether the reference should really be returned at all, rather than as the default fix.
