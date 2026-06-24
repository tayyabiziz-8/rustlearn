# Traits
desc: Shared behavior across types, default implementations, trait bounds, and dynamic dispatch.

A `trait` defines behavior that multiple types can share — similar in spirit to an interface in other languages, but with the option of providing a default implementation.

```rust
trait Hello {
    // Default implementation — types can use this as-is...
    fn say_hi(&self) -> String {
        String::from("hi")
    }

    // ...or override it. This one has no body, so every implementer MUST
    // provide their own.
    fn say_something(&self) -> String;
}

struct Student;
impl Hello for Student {
    fn say_something(&self) -> String {
        String::from("I'm a good student")
    }
    // say_hi is not overridden — it uses the trait's default
}

struct Teacher;
impl Hello for Teacher {
    fn say_hi(&self) -> String {
        String::from("Hi, I'm your new teacher")
    }
    fn say_something(&self) -> String {
        String::from("I'm not a bad teacher")
    }
}

fn main() {
    let s = Student;
    assert_eq!(s.say_hi(), "hi");
    assert_eq!(s.say_something(), "I'm a good student");

    let t = Teacher;
    assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
    assert_eq!(t.say_something(), "I'm not a bad teacher");
}
```

## Trait bounds: requiring behavior from a generic parameter

```rust
#[derive(Debug)]
struct Inches(f64);

#[derive(Debug, PartialEq, PartialOrd)]
struct Centimeters(f64);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        Centimeters(self.0 * 2.54)
    }
}

fn main() {
    let foot = Inches(12.0);
    let meter = Centimeters(100.0);

    // PartialOrd (derived above) is what makes `<` valid for Centimeters
    let comparison = if foot.to_centimeters() < meter { "smaller" } else { "bigger" };

    assert_eq!(comparison, "smaller");
    println!("One foot is {comparison} than one meter.");
}
```

## Static dispatch vs. dynamic dispatch

There are two ways to write a function that accepts "anything implementing trait `Animal`":

```rust
trait Animal {
    fn speak(&self) -> &str;
}

struct Dog;
struct Cat;
impl Animal for Dog {
    fn speak(&self) -> &str { "Woof" }
}
impl Animal for Cat {
    fn speak(&self) -> &str { "Meow" }
}

// STATIC DISPATCH: `impl Animal` generates a separate, specialized copy of
// this function for every concrete type it's called with. No runtime cost,
// but every call site must know the concrete type at compile time.
fn greet_static(animal: &impl Animal) {
    println!("static: {}", animal.speak());
}

// DYNAMIC DISPATCH: `&dyn Animal` is a "fat pointer" — data pointer plus a
// vtable pointer for method lookup. One function body handles every type,
// resolved at runtime, at the cost of a small indirection per call.
fn greet_dynamic(animal: &dyn Animal) {
    println!("dynamic: {}", animal.speak());
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    greet_static(&dog);
    greet_static(&cat);

    // dyn is also what lets you store DIFFERENT concrete types in one Vec —
    // see the trait-object example in the Vectors chapter.
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(dog), Box::new(cat)];
    for a in &animals {
        greet_dynamic(a.as_ref());
    }
}
```

> Rule of thumb: reach for `impl Trait` / generics by default (faster, and errors show up at the call site). Reach for `dyn Trait` when you need a heterogeneous collection, or when the concrete type genuinely isn't known until runtime — e.g. plugins loaded by name.
