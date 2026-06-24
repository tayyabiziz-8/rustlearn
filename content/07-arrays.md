# Arrays
desc: Fixed-size, stack-allocated collections, and safe vs. unsafe indexing.

An array has a **fixed length, known at compile time**, and lives on the stack (unless you put it behind a `Box` or inside something heap-allocated). All elements share the same type.

```rust
fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    assert_eq!(arr.len(), 5);

    let repeated: [i32; 100] = [1; 100]; // [1, 1, 1, ..., 1] — 100 elements
    assert_eq!(repeated[0], 1);
    assert_eq!(repeated.len(), 100);
}
```

Arrays are stack-allocated, so their total size is just `size_of::<Element>() * length`.

```rust
fn main() {
    let arr: [char; 3] = ['a', 'b', 'c'];
    // a char is 4 bytes in Rust (it's a Unicode scalar value, not a byte)
    assert_eq!(std::mem::size_of_val(&arr), 12);
}
```

## `arr[i]` vs. `arr.get(i)`

Indexing with `[]` panics if the index is out of bounds. `.get(i)` returns an `Option<&T>` instead, so out-of-bounds access becomes a value you can handle rather than a crash.

```rust
fn main() {
    let arr = [10, 20, 30];

    let third = arr[2]; // panics here if index >= len
    assert_eq!(third, 30);

    match arr.get(5) {
        Some(v) => println!("found {v}"),
        None => println!("index 5 is out of bounds — handled safely"),
    }
}
```
