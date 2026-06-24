# Functional Programming
desc: Closures, what they capture, and the iterator adapters that replace most manual loops.

Rust isn't a purely functional language, but closures and iterators give you most of the functional toolkit - and the type system tracks exactly what a closure captures, so there's no hidden runtime cost or surprise mutation.

## Closures

A closure is an anonymous function that can capture variables from the scope it was defined in.

```rust
fn main() {
    let factor = 3;

    // |params| body - type annotations are usually inferred
    let multiply = |x: i32| x * factor; // captures `factor` by reference

    assert_eq!(multiply(5), 15);
    assert_eq!(multiply(10), 30);
}
```

## What gets captured, and how

Rust infers the *least* restrictive capture mode that makes the closure body compile: borrow immutably if possible, mutably if needed, or take ownership (`move`) if you ask for it explicitly.

```rust
fn main() {
    let name = String::from("Ferris");

    // Captures `name` by immutable reference - name is still usable after.
    let greet = || println!("hello, {name}");
    greet();
    println!("still usable: {name}");

    let mut count = 0;
    // Captures `count` by mutable reference, because the body mutates it.
    let mut increment = || {
        count += 1;
        println!("count is now {count}");
    };
    increment();
    increment();
}
```

`move` forces the closure to take ownership of everything it uses, instead of borrowing - essential when the closure needs to outlive the scope it was created in (a very common requirement when spawning a thread).

```rust
use std::thread;

fn main() {
    let data = vec![1, 2, 3];

    // Without `move`, the closure would borrow `data`, and the compiler
    // can't guarantee `data` outlives the spawned thread. `move` makes
    // the closure own its copy outright.
    let handle = thread::spawn(move || {
        println!("data on another thread: {data:?}");
    });

    handle.join().unwrap();
}
```

## The three closure traits

Every closure implements one or more of `Fn`, `FnMut`, `FnOnce`, depending on how it uses its captures - this is what lets a function parameter say "accept any closure that only reads its captures" vs. "...that needs to consume them."

| Trait | Can be called | Typical use |
|---|---|---|
| `Fn` | repeatedly, `&self` | only reads captured variables |
| `FnMut` | repeatedly, `&mut self` | mutates a captured variable |
| `FnOnce` | exactly once, `self` | consumes (moves out of) a captured variable |

```rust
fn call_twice<F: Fn()>(f: F) {
    f();
    f();
}

fn call_once<F: FnOnce() -> String>(f: F) -> String {
    f()
}

fn main() {
    let greeting = String::from("hi");
    call_twice(|| println!("{greeting}")); // only reads -> Fn

    let owned = String::from("consumed");
    let result = call_once(move || owned); // moves `owned` out -> FnOnce
    assert_eq!(result, "consumed");
}
```

## Iterators

An iterator produces values one at a time and is **lazy** - calling `.map()` doesn't do any work by itself; nothing actually runs until something consumes the iterator (`.collect()`, `for`, `.sum()`, etc.).

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let doubled: Vec<i32> = numbers.iter().map(|n| n * 2).collect();
    assert_eq!(doubled, vec![2, 4, 6, 8, 10]);

    let evens: Vec<&i32> = numbers.iter().filter(|&&n| n % 2 == 0).collect();
    assert_eq!(evens, vec![&2, &4]);

    let total: i32 = numbers.iter().sum();
    assert_eq!(total, 15);
}
```

## Chaining adapters

This is where iterators replace most hand-written loops - each step describes *what* transformation happens, and the chain only runs through the data once when finally consumed.

```rust
fn main() {
    let words = vec!["apple", "fig", "banana", "kiwi", "cherry"];

    let result: Vec<String> = words
        .iter()
        .filter(|w| w.len() > 3)   // keep words longer than 3 chars
        .map(|w| w.to_uppercase()) // uppercase each
        .take(2)                   // stop after the first 2 matches
        .collect();

    assert_eq!(result, vec!["APPLE", "BANANA"]);
}
```

`fold` is the general-purpose reducer everything else (`sum`, `max`, `count`...) is built on conceptually - it threads an accumulator through every element.

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let product = numbers.iter().fold(1, |acc, &n| acc * n);
    assert_eq!(product, 120);

    // Building a String is a common fold/reduce target too:
    let sentence = vec!["the", "quick", "fox"]
        .iter()
        .fold(String::new(), |mut acc, word| {
            if !acc.is_empty() {
                acc.push(' ');
            }
            acc.push_str(word);
            acc
        });
    assert_eq!(sentence, "the quick fox");
}
```

> Iterator chains aren't just shorter - the compiler typically optimizes them down to the same machine code as the equivalent hand-written loop (a "zero-cost abstraction"), so reaching for `.map().filter().collect()` over a manual loop is a readability choice, not a performance trade-off.
