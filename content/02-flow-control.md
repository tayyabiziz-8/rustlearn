# Flow Control
desc: if/else as an expression, for/while/loop, break with a value, and labeled loops.

## if / else

`if` is an expression in Rust - both branches must produce the same type if you're going to bind the result.

```rust
fn main() {
    let n = -5;

    if n < 0 {
        println!("{n} is negative");
    } else if n > 0 {
        println!("{n} is positive");
    } else {
        println!("{n} is zero");
    }

    // Using if/else to produce a value:
    let description = if n < 0 { "negative" } else { "non-negative" };
    assert_eq!(description, "negative");
}
```

## for

`for` consumes anything that implements `IntoIterator`. Ranges, arrays, and `&collection` are the most common.

```rust
fn main() {
    for n in 0..5 {
        print!("{n} "); // 0 1 2 3 4 - exclusive end
    }
    println!();

    for n in 1..=5 {
        print!("{n} "); // 1 2 3 4 5 - inclusive end, because of `=`
    }
    println!();

    let names = [String::from("Liming"), String::from("Hanmeimei")];
    for name in &names {
        // borrowing `&names` instead of moving it - `names` is still usable after
        println!("{name}");
    }

    for (i, v) in names.iter().enumerate() {
        println!("element {i} is {v}");
    }
}
```

## while

```rust
fn main() {
    let mut n = 1;
    while n < 6 {
        if n % 3 == 0 {
            println!("fizz");
        } else {
            println!("{n}");
        }
        n += 1;
    }
    println!("loop finished, n = {n}");
}
```

## loop - and breaking with a value

`loop` runs forever until a `break`. Unlike `for`/`while`, `break` can carry a value out of a `loop`, which makes `loop` an expression.

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // this value becomes the result of the loop
        }
    };

    assert_eq!(result, 20);
}
```

## Labeled loops

When loops are nested, a bare `break`/`continue` only affects the innermost loop. Label the outer loop to target it directly.

```rust
fn main() {
    let mut count = 0;

    'outer: loop {
        'inner: loop {
            count += 1;
            if count == 3 {
                continue 'outer; // skip back to the outer loop
            }
            if count >= 8 {
                break 'outer; // exit both loops
            }
        }
    }

    assert_eq!(count, 8);
}
```
