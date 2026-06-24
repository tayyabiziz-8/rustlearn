# Generics
desc: Writing code once that works across many types, with zero runtime cost.

Generics let you write a struct, enum, or function once and have it work for many concrete types - the compiler generates a specialized version for each type actually used (called *monomorphization*), so there's no runtime overhead compared to writing it by hand for each type.

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    // T is inferred as i32, U as String, from the values given
    let p = Point { x: 5, y: "hello".to_string() };
    assert_eq!(p.x, 5);
    assert_eq!(p.y, "hello");
}
```

## Generic methods

```rust
struct Val<T> {
    val: T,
}

impl<T> Val<T> {
    fn value(&self) -> &T {
        &self.val
    }
}

fn main() {
    let x = Val { val: 3.0 };
    let y = Val { val: "hello".to_string() };
    println!("{}, {}", x.value(), y.value());
}
```

## A generic method can introduce its own extra type parameters

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    // V and W belong to this method only, separate from the struct's T, U
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point { x: self.x, y: other.y }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: "Hello", y: '中' };

    let p3 = p1.mixup(p2); // p1's x, p2's y

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');
}
```

## Bounding what a generic type can do

Without constraints, a generic `T` could be anything - so the compiler won't let you call methods on it. A **trait bound** (covered in full in [Traits](16-traits.html)) says "T must support this behavior."

```rust
use std::fmt::Display;

fn print_twice<T: Display>(value: T) {
    println!("{value}");
    println!("{value}"); // works because Display guarantees a formatting method
}

fn main() {
    print_twice(42);
    print_twice("hello");
}
```
