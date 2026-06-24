# Tuples
desc: Fixed-size, heterogeneous groupings, accessed by position.

A tuple groups a fixed number of values, which can each be a *different* type. Access fields with `.0`, `.1`, etc., or destructure them by pattern.

```rust
fn main() {
    let t1: (u8, i16) = (0, -1);
    let t2: (u8, (i16, u32)) = (0, (-1, 1));

    assert_eq!(t1.0, 0);
    assert_eq!((t2.1).0, -1);

    let (a, b) = t1; // destructure into separate bindings
    assert_eq!(a, 0);
    assert_eq!(b, -1);
}
```

```rust
fn main() {
    let t = (String::from("hello"), String::from("world"));
    let (s1, s2) = t.clone(); // clone the whole tuple since String isn't Copy
    println!("{s1}, {s2}, {t:?}");
}
```

> The `println!("{t:?}")` form above needs the `Debug` trait, which tuples implement automatically as long as every element does. Tuples longer than 12 elements lose this automatic `Debug`/`PartialEq` support — past that point, reach for a named `struct` instead.
