# Ownership
desc: Rust's compile-time memory model - the rule set that replaces a garbage collector.

Ownership is the set of rules the compiler checks **at compile time**, with zero runtime cost, to decide when memory gets freed.

**The three rules:**
1. Each value has exactly one owner (one variable that's responsible for it).
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value is dropped (its memory is freed).

## Stack vs. heap, in one example

`String` is heap-allocated and growable. The variable itself - a small fixed-size record of pointer, length, and capacity - lives on the stack; the actual character data lives on the heap.

```rust
fn main() {
    let s1 = String::from("hello");
    // s1 on the stack: { ptr -> heap, len: 5, capacity: 5 }
    // heap: the bytes "hello"
}
```

Plain numbers, by contrast, live entirely on the stack and have a known, fixed size - so they're cheap to duplicate.

```rust
fn main() {
    let x = 5;
    let y = x; // a full bit-for-bit copy, two independent values
    assert_eq!(x, 5); // x is still valid - i32 implements the Copy trait
    assert_eq!(y, 5);
}
```

## Move: why `String` behaves differently

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // the stack record (ptr/len/capacity) is copied to s2...
                 // ...but the heap data is NOT duplicated.

    // s1 is now considered moved-out-of and can't be used again.
    // This is Rust preventing a double-free: if both s1 and s2 stayed
    // valid, they'd both try to free the same heap memory when dropped.

    // println!("{s1}"); // compile error: value borrowed after move

    println!("{s2}"); // fine - s2 is the sole owner now
}
```

## Clone: an explicit deep copy

When you actually want two independent copies of heap data, call `.clone()` - it's deliberately explicit, so a deep copy never happens silently.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}"); // both valid - separate heap allocations
}
```

## Ownership and function calls

Passing a value to a function moves it, exactly like an assignment does.

```rust
fn main() {
    let s1 = String::from("hello");
    takes_ownership(s1);
    // s1 is invalid from here on - ownership moved into the function

    let s2 = gives_ownership();
    let s3 = takes_and_gives_back(s2);
    // s2 is invalid; s3 now owns what s2 used to own

    let x = 5;
    makes_copy(x);
    println!("{x}"); // fine - i32 is Copy, so `x` was duplicated, not moved
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // some_string goes out of scope here -> dropped

fn gives_ownership() -> String {
    String::from("world")
} // ownership moves out to the caller

fn takes_and_gives_back(some_string: String) -> String {
    some_string // ownership passes straight through
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}
```

## `Box<T>`: putting a value on the heap on purpose

```rust
fn main() {
    let mut x: Box<i32> = Box::new(5); // a heap-allocated i32, x owns it
    *x = 7; // dereference to read/write the heap value
    assert_eq!(*x, 7);
} // x goes out of scope -> the heap allocation is freed automatically
```
