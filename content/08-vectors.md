# Vectors
desc: Dynamically-sized, heap-allocated, growable lists and the conversions in and out of them.

`Vec<T>` is the array's growable cousin: heap-allocated, resizable at runtime, created with `Vec::new()` or the `vec!` macro.

```rust
fn main() {
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];        // macro form, infers Vec<i32>
    let v3 = vec![0; 5];           // [0, 0, 0, 0, 0]

    assert_eq!(v1.len(), 0);
    assert_eq!(v2, [1, 2, 3]);
    assert_eq!(v3, [0, 0, 0, 0, 0]);
}
```

## Push, pop, extend

```rust
fn main() {
    let mut v1 = vec![1, 2, 4];
    v1.pop();      // removes and returns the last element (Some(4))
    v1.push(3);    // v1 is now [1, 2, 3]

    let mut v2 = Vec::new();
    v2.extend(&v1); // appends a copy of every element in v1

    assert_eq!(v1, v2);
}
```

## Converting to and from a Vec

```rust
fn main() {
    // Array -> Vec
    let arr = [1, 2, 3];
    let from_arr: Vec<i32> = Vec::from(arr);
    let via_into: Vec<i32> = arr.into();
    assert_eq!(from_arr, via_into);

    // String -> Vec<u8>
    let s = "hello".to_string();
    let bytes: Vec<u8> = s.into_bytes();
    assert_eq!(bytes, vec![104, 101, 108, 108, 111]);

    // Any Iterator can be collected into a Vec
    let squares: Vec<i32> = (1..=5).map(|n| n * n).collect();
    assert_eq!(squares, vec![1, 4, 9, 16, 25]);
}
```

## Safe access with `.get()`

```rust
fn main() {
    let mut v = vec![1, 2, 3];

    for i in 0..5 {
        match v.get(i) {
            Some(existing) => {
                let doubled = existing + 1;
                v[i] = doubled; // safe: we just confirmed index i exists
            }
            None => v.push(i as i32 + 2), // grow the vector instead of panicking
        }
    }

    assert_eq!(v, vec![2, 3, 4, 5, 6]);
}
```

## A vector of trait objects

Because every element of a `Vec<T>` must be the same concrete type, storing *different* types that share behavior means storing them behind `Box<dyn Trait>`. See [Traits](16-traits.html) for what `dyn` means.

```rust
trait IpAddr {
    fn display(&self) -> String;
}

struct V4(String);
impl IpAddr for V4 {
    fn display(&self) -> String {
        format!("ipv4: {}", self.0)
    }
}

struct V6(String);
impl IpAddr for V6 {
    fn display(&self) -> String {
        format!("ipv6: {}", self.0)
    }
}

fn main() {
    let addrs: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for addr in &addrs {
        println!("{}", addr.display());
    }
}
```
