# Variables & Basic Types
desc: Mutability, destructuring, integer/float types, chars, and the unit type.

By default, variables in Rust are **immutable**. You opt into mutability explicitly with `mut`. The compiler tells you up front which values are allowed to change.

```rust
fn main() {
    let _a = 2; // prefixing with `_` tells the compiler "I know this is unused"
    let mut x: i32 = 5;
    x += 2;
    println!("value is: {}", x);
}
```

## Destructuring assignment

You can bind multiple variables in one pattern, and even assign into an *existing* binding using a bare pattern (no `let`):

```rust
fn main() {
    let (mut x, y) = (1, 2);
    x += 2;
    assert_eq!(x, 3);
    assert_eq!(y, 2);

    // Pattern can also be the target of a later assignment.
    let (a, b);
    (a, ..) = (3, 4);   // `..` ignores the rest of the tuple
    [.., b] = [10, 20]; // `..` here ignores the leading elements
    assert_eq!(a, 3);
    assert_eq!(b, 20);
}
```

## Integer and float types

| Width | Signed | Unsigned |
|---|---|---|
| 8-bit  | `i8`  | `u8`  |
| 16-bit | `i16` | `u16` |
| 32-bit | `i32` | `u32` |
| 64-bit | `i64` | `u64` |
| arch (32 or 64-bit, depends on the CPU) | `isize` | `usize` |

If you don't annotate a type, Rust defaults to `i32` for integers and `f64` for floats.

```rust
fn main() {
    let x = 38_u8;            // type is u8, the underscore is just a visual separator
    let y: u16 = 38_u8 as u16; // explicit conversion with `as`

    // Floating point is imprecise. This assertion panics, even though it
    // looks true on paper.
    // assert!(0.1 + 0.2 == 0.3); // panics: 0.1 + 0.2 == 0.30000000000000004

    println!("{x} {y}");
}
```

## Chars, ASCII, and bit operations

A Rust `char` is always 4 bytes. It represents a full Unicode scalar value, not just ASCII.

```rust
fn main() {
    for c in 'a'..='z' {
        print!("{} ", c as u8); // casting a char to u8 gives its ASCII code
    }
    println!();

    let y: u32 = 0b0000_1010;
    println!("{:04b}", y);     // binary, zero-padded to 4 digits: 1010
    println!("{:x}", 255);     // hex: ff

    let shifted = 1_u32 << 5;  // left shift 5 places
    assert_eq!(shifted, 32);
}
```

## The unit type

A function with no `-> ReturnType` returns `()`, the **unit type**. It is Rust's way of saying "nothing to return here." It is not the same as `void` in C; it is an actual zero-sized value you can bind.

```rust
fn print_status() {
    println!("done");
} // implicitly returns ()

fn main() {
    let v: () = print_status();
    assert_eq!(v, ());
}
```

> A trailing semicolon after the last expression in a function body turns that expression into a statement, and statements evaluate to `()`. So `fn five() -> i32 { 5; }` is a compile error because the body now evaluates to `()`, not `i32`.

## Expressions vs. statements

A block `{ ... }` is itself an expression. Its value is whatever the last line evaluates to, **as long as that line has no trailing semicolon**.

```rust
fn main() {
    let y = {
        let x_squared = 3 * 3;
        let x_cube = x_squared * 3;
        x_cube + x_squared + 3 // no semicolon, so this is the block's value
    };
    assert_eq!(y, 39);
}
```

## Diverging functions

A function that never returns has the special return type `!` (called "never"). The compiler knows nothing after the call can possibly run.

```rust
fn explode(reason: &str) -> ! {
    panic!("fatal: {reason}");
}

fn main() {
    let code = 1;
    if code != 0 {
        explode("non-zero exit code");
    }
    println!("only reached if code == 0");
}
```
