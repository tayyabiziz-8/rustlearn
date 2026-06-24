# Pattern Matching
desc: match, if let, matches!, and binding sub-patterns with @.

## match

`match` compares a value against a series of patterns and runs the code for the first one that fits. It is **exhaustive**: the compiler refuses to build unless every possible case is handled, which is what makes it safer than a chain of `if/else if`.

```rust
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dir = Direction::South;

    match dir {
        Direction::East => println!("East"),
        Direction::South | Direction::North => println!("South or North"),
        _ => println!("West"), // catch-all for anything not named above
    }
}
```

`match` is an expression, so each arm can produce a value:

```rust
fn main() {
    let boolean = true;
    let binary = match boolean {
        true => 1,
        false => 0,
    };
    assert_eq!(binary, 1);
}
```

## if let: a shorthand for matching one pattern

When you only care about a single pattern and want to ignore everything else, `if let` is less boilerplate than a `match` with a `_` arm.

```rust
enum Coin {
    Penny,
    Quarter(u8),
}

fn main() {
    let coin = Coin::Quarter(12);

    if let Coin::Quarter(state) = coin {
        println!("a quarter from state {state}");
    } else {
        println!("not a quarter");
    }
}
```

## matches!: a boolean test from a pattern

```rust
fn main() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];

    for ab in alphabets {
        assert!(matches!(ab, 'A'..='Z' | 'a'..='z' | '0'..='9'));
    }
}
```

## Binding with @ and matching ranges inside a struct

`name @ pattern` lets you test a value against a pattern *and* capture it into `name` at the same time.

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 2, y: 20 };

    match p {
        Point { x, y: 0 } => println!("on the x-axis at {x}"),
        Point { x: 0..=5, y: y @ (10 | 20 | 30) } => {
            println!("x is in 0..=5, and y is captured as {y}")
        }
        Point { x, y } => println!("elsewhere: ({x}, {y})"),
    }
}
```

## Ignoring the middle of a tuple with `..`

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first, .., last) => {
            assert_eq!(first, 2);
            assert_eq!(last, 2048);
        }
    }
}
```
