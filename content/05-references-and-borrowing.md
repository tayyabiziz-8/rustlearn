# References & Borrowing
desc: Accessing data without taking ownership, and the rules that make it safe.

A reference (`&T` or `&mut T`) lets you access a value without taking ownership of it — this is called **borrowing**.

**The borrowing rules, enforced at compile time:**
1. At any given time, you can have *either* one mutable reference *or* any number of immutable references — never both.
2. References must always point to valid data (no dangling references).

## Immutable borrows

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // &s1 borrows, doesn't take ownership
    println!("length of '{s1}' is {len}"); // s1 is still usable
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but it doesn't own the data, so nothing is dropped
```

## Mutable borrows

```rust
fn main() {
    let mut s1 = String::from("hello");
    add_world(&mut s1);
    assert_eq!(s1, "hello world");
}

fn add_world(s: &mut String) {
    s.push_str(" world");
}
```

## Rule 1 in practice: one mutable XOR many immutable

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s; // fine — multiple immutable borrows are allowed
    println!("{r1} and {r2}");
    // r1 and r2's last use was the line above, so their borrow "ends" here
    // (this is the compiler's non-lexical lifetime analysis)

    let r3 = &mut s; // fine now, because r1/r2 are no longer used afterward
    r3.push_str(", world");
    println!("{r3}");
}
```

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    r1.push_str(" world");
    // let r2 = &mut s;     // compile error if used while r1 is still alive:
                             // "cannot borrow `s` as mutable more than once"
    println!("{r1}");
}
```

## Rule 2 in practice: no dangling references

```rust
// This function would NOT compile:
//
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } // s is dropped here — the reference we tried to return would point
//   // at freed memory. The compiler rejects this before it can happen.

// The fix: return the owned value itself, transferring ownership out.
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

fn main() {
    let s = no_dangle();
    assert_eq!(s, "hello");
}
```

> Borrowing a `String` followed by mutating it through the *original* owner is rejected — the immutable borrow and the mutation can't coexist. For example, taking `&s` and then calling `s.clear()` while the reference is still in use is a compile error, because `clear()` requires `&mut self` and you already have an outstanding `&s`.
