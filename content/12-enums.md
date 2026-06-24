# Enums
desc: Tagged unions, variants that can carry different data, and the built-in Option enum.

An `enum` defines a type as one of several named **variants**. Unlike enums in C, Rust variants can carry their own data, making them a real tagged union.

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    if let IpAddr::V4(addr) = home {
        println!("home is IPv4: {addr}");
    }
    if let IpAddr::V6(addr) = loopback {
        println!("loopback is IPv6: {addr}");
    }
}
```

## Variants with named fields

```rust
enum Message {
    Quit,                       // no data
    Move { x: i32, y: i32 },    // named fields, like a mini-struct
    Write(String),              // a single unnamed value
    ChangeColor(i32, i32, i32), // multiple unnamed values
}

fn main() {
    let msgs = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg);
    }
}

fn show_message(msg: Message) {
    match msg {
        Message::Move { x, y } => println!("move to ({x}, {y})"),
        Message::ChangeColor(r, g, b) => println!("color set to ({r}, {g}, {b})"),
        _ => println!("no associated data"),
    }
}
```

## `Option<T>`: no null, just an explicit maybe

Rust has no `null`. Instead, the standard library defines `Option<T>`. The compiler forces you to handle the `None` case before you can use the value, so a null pointer style bug becomes a compile error instead of a crash.

```rust
enum Option<T> {
    None,
    Some(T),
}
```

```rust
fn main() {
    let some_number: Option<i32> = Some(5);
    let no_number: Option<i32> = None;

    match some_number {
        Some(n) => println!("got {n}"),
        None => println!("nothing here"),
    }

    // unwrap_or gives you a fallback instead of writing out the match
    assert_eq!(no_number.unwrap_or(0), 0);
}
```
