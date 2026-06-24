# Functions
desc: Declaring functions, parameters, return types, and the implicit-return rule.

```rust
fn main() {
    print_char('R');

    let total = sum(3, 4);
    assert_eq!(total, 7);
}

fn print_char(c: char) {
    println!("{c}");
}

// The return type comes after `->`. The last expression in the body
// (no semicolon) is the returned value. No `return` keyword needed.
fn sum(x: i32, y: i32) -> i32 {
    x + y
}
```

You *can* use `return` for an early exit. It is required when you want to leave a function before reaching its final expression.

```rust
fn first_positive(nums: &[i32]) -> Option<i32> {
    for &n in nums {
        if n > 0 {
            return Some(n); // early exit
        }
    }
    None // final expression, no semicolon
}

fn main() {
    assert_eq!(first_positive(&[-3, -1, 4, 7]), Some(4));
    assert_eq!(first_positive(&[-3, -1]), None);
}
```

> A function declared with no `-> Type` returns `()`. Writing `fn f() -> ()` is legal but redundant. Nobody writes it that way in practice.
