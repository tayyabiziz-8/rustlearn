# Crates and Modules
desc: How Rust projects are organized: crates, modules, paths, and visibility.

A **crate** is the unit Rust compiles at once. It is either a *binary crate* (`src/main.rs`, produces an executable) or a *library crate* (`src/lib.rs`, produces something other crates can depend on). A **module** (`mod`) is how you organize code within a crate into a namespace tree.

## Modules, inline

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("added to waitlist");
        }
    }

    mod serving {
        // private by default, only visible inside front_of_house
        fn take_order() {
            println!("order taken");
        }
    }
}

fn main() {
    // Absolute path, starting from the crate root:
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path also works here, since we are at the crate root:
    front_of_house::hosting::add_to_waitlist();
}
```

## Everything is private unless marked `pub`

This is the default that makes Rust's module system useful as actual encapsulation, not just file organization. A function, struct field, or module is invisible outside its parent module until you explicitly say otherwise.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Without `pub mod hosting` above, the line below would not compile,
// even though add_to_waitlist is pub:
// error[E0603]: module `hosting` is private
fn main() {
    front_of_house::hosting::add_to_waitlist();
}
```

## `use`: bringing a path into scope

Writing out the full path every time gets old fast. `use` creates a shortcut.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("waitlisted");
        }
    }
}

use crate::front_of_house::hosting;

fn main() {
    hosting::add_to_waitlist(); // shortcut, instead of the full path
}
```

> Convention: `use` a function's parent module (`hosting::add_to_waitlist()`), not the function itself. That keeps the call site clear about where the function comes from. For structs, enums, and traits, the opposite convention applies: `use` the type itself directly (e.g. `use std::collections::HashMap;`, then just write `HashMap::new()`).

## Splitting modules across files

The examples above use `mod front_of_house { ... }` with the body inline, but in a real project each module typically gets its own file. Writing `mod front_of_house;` with no body, just a semicolon, tells Rust to load this module's contents from a file:

```text
src/
├── main.rs            // contains: mod front_of_house;
├── front_of_house.rs   // contents of the front_of_house module
└── front_of_house/
    └── hosting.rs       // contents of front_of_house::hosting
```

```rust
// src/main.rs
mod front_of_house; // Rust looks for ./front_of_house.rs

use front_of_house::hosting;

fn main() {
    hosting::add_to_waitlist();
}
```

```rust
// src/front_of_house.rs
pub mod hosting; // Rust looks for ./front_of_house/hosting.rs
```

```rust
// src/front_of_house/hosting.rs
pub fn add_to_waitlist() {
    println!("waitlisted");
}
```

## Re-exporting with `pub use`

`pub use` re-exports a path under the current module, so callers do not need to know your internal structure to reach it.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Without this, an external user would need:
//   crate::front_of_house::hosting::add_to_waitlist()
// With it, they can write:
//   crate::hosting::add_to_waitlist()
pub use crate::front_of_house::hosting;

fn main() {
    hosting::add_to_waitlist();
}
```

## External crates

Dependencies come from [crates.io](https://crates.io) and are declared in `Cargo.toml`:

```text
[dependencies]
serde = "1.0"
rand = "0.8"
```

Since the 2018 edition, you do not need `extern crate` to use them. `use` alone is enough:

```text
use rand::Rng;

fn roll() -> u32 {
    rand::thread_rng().gen_range(1..=6)
}
```
