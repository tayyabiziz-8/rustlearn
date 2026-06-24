# Structs
desc: Named fields, tuple structs, update syntax, and partial moves.

A `struct` groups related data under named fields.

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someuser12"),
        email: String::from("someuser@example.com"),
        sign_in_count: 1,
    };

    user1.username = String::from("john_doe"); // requires `user1` to be `mut`

    // Struct update syntax: take every field NOT explicitly listed from user1.
    // This moves any non-Copy fields out of user1 (here: username), so
    // user1 as a whole becomes partially invalid afterward.
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{}", user2.email);
}
```

## Field init shorthand

When a variable's name matches the field name, you can skip the `field: field` repetition.

```rust
struct Person {
    name: String,
    age: u8,
}

fn build_person(name: String, age: u8) -> Person {
    Person { name, age } // shorthand for `name: name, age: age`
}

fn main() {
    let p = build_person(String::from("Alice"), 30);
    assert_eq!(p.age, 30);
}
```

## Tuple structs

A tuple struct has a name but unnamed, positional fields — useful for giving a tuple shape a distinct type.

```rust
struct Point(i32, i32, i32);

fn main() {
    let v = Point(0, 127, 255);
    check_color(v);
}

fn check_color(p: Point) {
    let Point(x, _, z) = p; // destructure by position
    assert_eq!(x, 0);
    assert_eq!(p.1, 127); // or access by index directly
    assert_eq!(z, 255);
}
```

## Partial moves

Destructuring a struct can move *some* fields out while leaving others behind, as long as the remaining fields are only borrowed (`ref`) or are themselves `Copy`.

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: Box<u8>,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out; `age` is only borrowed via `ref`.
    let Person { name, ref age } = person;

    println!("age: {age}");
    println!("name: {name}");

    // person as a whole is now partially moved — person.name is gone —
    // but the still-valid field can still be reached through it:
    println!("age via person: {}", person.age);

    // println!("{person:?}"); // compile error: partially moved value
}
```
