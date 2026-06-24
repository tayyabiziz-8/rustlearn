# Result and Panic
desc: Recoverable errors with Result<T, E>, the ? operator, and when panic! is actually the right call.

Rust splits errors into two categories and treats them very differently:

- **Unrecoverable** errors use `panic!`. The program prints an error and unwinds (or aborts), with no path back. Reserved for bugs and states the program genuinely cannot continue from.
- **Recoverable** errors use `Result<T, E>`. This is an ordinary value the caller is *expected* to handle. It is the normal way to signal "this operation might not work," and the compiler will not let you silently ignore it.

## panic!

```rust
fn main() {
    let v = vec![1, 2, 3];
    // v[10]; // panics at runtime: "index out of bounds: the len is 3 but the index is 10"

    // panic! directly, with a custom message:
    if v.is_empty() {
        panic!("expected at least one element");
    }
    println!("first element: {}", v[0]);
}
```

`unimplemented!()` and `todo!()` are both shorthand for panicking with a message saying this is not done yet. Useful as placeholders while sketching out a design.

## `Result<T, E>`

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("division by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10.0, 2.0) {
        Ok(result) => println!("result: {result}"),
        Err(e) => println!("error: {e}"),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("result: {result}"),
        Err(e) => println!("error: {e}"),
    }
}
```

## `unwrap` and `expect`: opting back into panicking

Sometimes you genuinely know a `Result` will be `Ok`, or you are prototyping and do not want to handle the error path yet. `.unwrap()` returns the `Ok` value or panics. `.expect(msg)` does the same but with a custom panic message, which makes debugging far easier than a bare `unwrap()`.

```rust
fn main() {
    let good: Result<i32, String> = Ok(42);
    assert_eq!(good.unwrap(), 42);

    let parsed = "42".parse::<i32>().expect("input should have been numeric");
    assert_eq!(parsed, 42);

    // "forty-two".parse::<i32>().unwrap(); // panics: "called `Result::unwrap()`
                                             // on an `Err` value: ParseIntError { .. }"
}
```

> Reach for `unwrap` and `expect` in examples, tests, and quick scripts, and in production code only when a failure there genuinely represents a bug rather than an expected condition. Otherwise, propagate the error instead.

## The `?` operator: propagating errors without the boilerplate

`?` on a `Result` does one of two things. If it is `Ok(value)`, it unwraps to `value` and keeps going. If it is `Err(e)`, it immediately returns that error from the enclosing function, converting it with `From` if the error types differ. This replaces a `match` that would otherwise show up after every fallible call.

```rust
use std::num::ParseIntError;

fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    let n = s.parse::<i32>()?; // returns early with Err if parsing fails
    Ok(n * 2)
}

fn main() {
    assert_eq!(parse_and_double("21"), Ok(42));
    assert!(parse_and_double("abc").is_err());
}
```

`?` chains cleanly across multiple fallible steps:

```rust
use std::num::ParseIntError;

fn sum_from_strings(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok(x + y)
}

fn main() {
    assert_eq!(sum_from_strings("10", "32"), Ok(42));
    assert!(sum_from_strings("10", "nope").is_err());
}
```

## A custom error type

Real programs usually have more than one way to fail. Defining your own error `enum` lets `?` unify multiple failure sources behind one type.

```rust
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    EmptyInput,
    NotANumber(ParseIntError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::EmptyInput => write!(f, "input was empty"),
            AppError::NotANumber(e) => write!(f, "not a number: {e}"),
        }
    }
}

// This `From` impl is what lets `?` auto-convert a ParseIntError into an
// AppError inside parse_positive below.
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::NotANumber(e)
    }
}

fn parse_positive(s: &str) -> Result<i32, AppError> {
    if s.is_empty() {
        return Err(AppError::EmptyInput);
    }
    let n: i32 = s.parse()?; // ParseIntError converts to AppError automatically
    Ok(n.abs())
}

fn main() {
    assert_eq!(parse_positive("-7").unwrap(), 7);
    assert!(matches!(parse_positive(""), Err(AppError::EmptyInput)));
    assert!(matches!(parse_positive("x"), Err(AppError::NotANumber(_))));
}
```

> `main` itself can return a `Result`. Write `fn main() -> Result<(), AppError>` and use `?` directly in `main`. If it returns `Err`, the process exits with a non-zero status and prints the error via `Debug`.
