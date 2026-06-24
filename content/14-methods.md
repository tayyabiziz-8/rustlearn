# Methods & Associated Functions
desc: impl blocks, &self vs self, and the difference between a method and an associated function.

An `impl` block attaches functions to a type. Inside it, a function that takes `self` (in some form) is a **method**, called with `value.method()`. A function that doesn't take `self` is an **associated function**, called with `Type::function()` — `String::from(...)` is one you've already used.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    assert_eq!(rect.area(), 1500);
}
```

## `&self`, `&mut self`, `self` — three different borrows

```rust
struct Counter {
    count: u32,
}

impl Counter {
    // Associated function — no `self` parameter. Conventionally named `new`.
    fn new() -> Self {
        Self { count: 0 }
    }

    // &self: read-only borrow. Use this whenever you don't need to mutate.
    fn value(&self) -> u32 {
        self.count
    }

    // &mut self: mutable borrow. Needed to change a field.
    fn increment(&mut self) {
        self.count += 1;
    }

    // self (no &): takes ownership and consumes the value. Used when a
    // method should transform something into a different value and the
    // original shouldn't be usable afterward.
    fn into_value(self) -> u32 {
        self.count
    }
}

fn main() {
    let mut c = Counter::new();
    c.increment();
    c.increment();
    assert_eq!(c.value(), 2);

    let final_value = c.into_value();
    // c can no longer be used here — into_value consumed it
    assert_eq!(final_value, 2);
}
```

## Methods on an enum

```rust
#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

impl TrafficLightColor {
    fn name(&self) -> &str {
        match self {
            Self::Red => "red",
            Self::Yellow => "yellow",
            Self::Green => "green",
        }
    }
}

fn main() {
    let c = TrafficLightColor::Yellow;
    assert_eq!(c.name(), "yellow");
    println!("{c:?}"); // {c:?} needs #[derive(Debug)] on the enum
}
```
