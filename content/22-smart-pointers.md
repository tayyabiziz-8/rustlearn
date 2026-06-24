# Smart Pointers
desc: Box, Rc, RefCell, and when each one solves a problem that a plain reference can't.

A **smart pointer** is a struct that holds a pointer and adds behavior on top - often some form of automatic cleanup or reference counting. They implement `Deref` (so they can be used like regular references) and `Drop` (so they clean up automatically when they go out of scope).

## `Box<T>`: heap allocation, single owner

`Box<T>` is the simplest smart pointer - it puts a value on the heap and gives you a pointer to it, with the same single-ownership rules as any `let` binding. The heap allocation is freed when the `Box` drops.

Three common reasons to reach for it:

**1. The type is too large for the stack, or you need a stable address.**
```rust
fn main() {
    let large_data = Box::new([0u8; 10_000]);
    println!("allocated {} bytes on the heap", large_data.len());
}
```

**2. A recursive type (a type that contains itself) - the compiler needs a fixed size, which a `Box` provides.**
```rust
// Without Box, the compiler rejects this:
//   error[E0072]: recursive type `List` has infinite size
//
// With Box, each node is a known, pointer-sized slot on the heap.
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{list:?}");
}
```

**3. You want to return a trait object whose concrete type isn't known at compile time.**
```rust
trait Shape {
    fn area(&self) -> f64;
}

struct Circle { radius: f64 }
impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
}

fn make_shape(kind: &str) -> Box<dyn Shape> {
    match kind {
        "circle" => Box::new(Circle { radius: 3.0 }),
        _ => panic!("unknown shape"),
    }
}

fn main() {
    let s = make_shape("circle");
    println!("area: {:.2}", s.area());
}
```

## `Rc<T>`: shared ownership within a single thread

`Rc<T>` ("reference counted") lets multiple parts of a program co-own the same heap value. Each `Rc::clone` increments a counter; each `drop` decrements it; the value is freed when the counter reaches zero.

> `Rc` is deliberately not `Send` - it can only be used on a single thread. For multi-threaded shared ownership, use `Arc<T>` (atomic reference count) instead.

```rust
use std::rc::Rc;

fn main() {
    let value = Rc::new(String::from("hello"));

    let a = Rc::clone(&value); // increments reference count; does NOT copy the String
    let b = Rc::clone(&value);

    println!("{value} / {a} / {b}");
    println!("owners: {}", Rc::strong_count(&value)); // 3

    drop(a);
    println!("after drop: {}", Rc::strong_count(&value)); // 2
} // value and b drop here -> count -> 0 -> String freed
```

The limitation: `Rc<T>` gives you a shared *immutable* reference. You can't mutate through it without combining it with `RefCell`.

## `RefCell<T>`: interior mutability (borrow checking at runtime)

Rust's normal borrow rules are checked at compile time. `RefCell<T>` moves those checks to runtime: `.borrow()` gives an immutable `Ref<T>`, `.borrow_mut()` gives a mutable `RefMut<T>`, and if you violate the rules (e.g., get two mutable borrows) the program **panics** instead of failing to compile.

This is useful when you know your access pattern is safe but the compiler's static analysis can't prove it - a common case is mutable state inside a callback or a struct that needs to mutate itself while being accessed through a shared reference.

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(vec![1, 2, 3]);

    {
        let mut v = data.borrow_mut(); // runtime check: gets the mutable guard
        v.push(4);
    } // guard dropped here - the mutable borrow is released

    println!("{:?}", data.borrow()); // [1, 2, 3, 4]
}
```

## `Rc<RefCell<T>>`: shared, mutable ownership

The most common pattern you'll see: wrap `RefCell<T>` in an `Rc<T>` to get multiple owners who can each mutate the inner value.

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Rc<RefCell<Node>>>,
}

fn main() {
    let leaf = Rc::new(RefCell::new(Node { value: 1, children: vec![] }));
    let branch = Rc::new(RefCell::new(Node {
        value: 5,
        children: vec![Rc::clone(&leaf)],
    }));

    // Mutate the leaf through the Rc<RefCell<...>>:
    leaf.borrow_mut().value = 10;

    // Both `leaf` and `branch.children[0]` see the new value.
    println!("leaf: {}", leaf.borrow().value);
    println!("via branch: {}", branch.borrow().children[0].borrow().value);
}
```

## The `Deref` trait: making a smart pointer feel like a reference

`Deref` is what allows `*box_value` to work just like `*reference` - and it's what powers **deref coercion**: the compiler automatically inserts `*` chains so that `&Box<String>` can be passed wherever `&str` is expected, for example.

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let boxed = MyBox(String::from("hello"));

    // Deref coercion: &MyBox<String> -> &String -> &str automatically.
    fn greet(s: &str) { println!("{s}"); }
    greet(&boxed);
}
```

## The `Drop` trait: running code on cleanup

`Drop` lets you specify what happens when a value goes out of scope. Smart pointers use it to free memory or release resources - you can implement it on your own types too.

```rust
struct Resource {
    name: String,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("releasing resource: {}", self.name);
    }
}

fn main() {
    let _a = Resource { name: "A".to_string() };
    let _b = Resource { name: "B".to_string() };
    println!("before end of scope");
} // B drops, then A drops - LIFO order (reverse of declaration)
```

> To force an early drop, use `std::mem::drop(value)` - this moves the value into `drop`, which runs the destructor immediately. Calling `value.drop()` directly is a compile error (you'd be calling `Drop::drop`, which takes `&mut self`, not by value, and Rust disallows calling the destructor manually to prevent a double-free).
