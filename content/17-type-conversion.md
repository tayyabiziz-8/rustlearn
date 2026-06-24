# Type Conversion
desc: as-casting and its sharp edges, From/Into, TryFrom/TryInto, and parsing strings.

Rust never converts types implicitly. No silent `int` to `float` promotion like in C. Every conversion is something you write explicitly, which means the compiler can show you exactly where data might be reshaped or lost.

## `as`: the blunt instrument

`as` works between numeric types, and between a few other specific pairs like `char` and `u32`. It is infallible by design and never panics, which means it has to make a decision when a value does not fit. That decision is "truncate" or "saturate," not "error."

```rust
fn main() {
    // Integer-to-integer: truncates by dropping the high bits.
    let big: i32 = 300;
    let truncated = big as u8;
    assert_eq!(truncated, 44); // 300 % 256 == 44, almost certainly not what you wanted

    // Float-to-integer: saturates at the target type's bounds instead of
    // wrapping or producing garbage. This has been the stable behavior
    // since Rust 1.45.
    let too_big = 1e10_f64 as i32;
    assert_eq!(too_big, i32::MAX);

    // Widening (small type to big type) is always lossless.
    let small: u8 = 38;
    let widened = small as u16;
    assert_eq!(widened, 38);

    // char <-> u32
    let code = 'R' as u32;
    assert_eq!(code, 82);
}
```

> Because `as` silently truncates, it is the wrong tool whenever an out-of-range value would be a bug rather than something to clamp. Use `TryFrom` (below) when you want a conversion that can fail loudly instead.

## `From` / `Into`: infallible, type-directed conversion

Implementing `From<A> for B` gives you `B::from(a)` for free, and also `a.into()`, because the standard library provides a blanket `Into` implementation for every `From`. These are the idiomatic way to convert between your own types.

```rust
struct Celsius(f64);
struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

fn main() {
    let boiling = Celsius(100.0);

    let f1 = Fahrenheit::from(Celsius(100.0));
    let f2: Fahrenheit = boiling.into(); // same conversion, called the other way

    assert_eq!(f1.0, 212.0);
    assert_eq!(f2.0, 212.0);
}
```

`From` is also how the standard library lets one function signature accept several input types. `String::from(&str)`, `Vec::from([T; N])`, and `?`-based error conversion (see [Result and Panic](20-result-and-panic.html)) all lean on it.

## `TryFrom` / `TryInto`: conversion that can fail

When a conversion might not be valid for every input, implement `TryFrom` instead. It returns a `Result`, so the failure path is forced into the open instead of silently truncating like `as` would.

```rust
use std::convert::TryFrom;

struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(format!("{value} is odd"))
        }
    }
}

fn main() {
    let ok = EvenNumber::try_from(8);
    let err = EvenNumber::try_from(7);

    assert!(ok.is_ok());
    assert_eq!(ok.unwrap().0, 8);
    assert_eq!(err.unwrap_err(), "7 is odd");

    // u8::try_from catches exactly the case `as` silently mangled earlier:
    let too_big: i32 = 300;
    let result: Result<u8, _> = u8::try_from(too_big);
    assert!(result.is_err());
}
```

## Parsing strings into numbers

`str::parse::<T>()` is built on `FromStr` and returns a `Result`. Parsing is a textbook example of a conversion that can fail because the text might not be a valid number at all.

```rust
fn main() {
    let good: Result<i32, _> = "42".parse();
    assert_eq!(good, Ok(42));

    let bad: Result<i32, _> = "forty-two".parse();
    assert!(bad.is_err());

    // The turbofish ::<T> tells parse() what to produce when it cannot be
    // inferred from context.
    let pi = "3.14".parse::<f64>().unwrap();
    assert!((pi - 3.14).abs() < f64::EPSILON);
}
```

## The other direction: making a type printable

`ToString` is implemented automatically for anything that implements `Display`, so implement `Display` and `.to_string()` comes along for free.

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 3, y: 7 };
    assert_eq!(p.to_string(), "(3, 7)");
    println!("{p}"); // Display also powers {} in println!
}
```
