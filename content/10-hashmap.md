# HashMap
desc: Key-value storage, the Entry API, and using a custom struct as a key.

`HashMap<K, V>` stores key-value pairs, heap-allocated, with no guaranteed iteration order. Average-case lookup, insert, and remove are O(1).

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69);
    scores.insert("Katie", 58);

    // .get() returns Option<&V> because there is no guarantee the key exists
    let score = scores.get("Sunface");
    assert_eq!(score, Some(&98));

    if scores.contains_key("Daniel") {
        let score = scores["Daniel"]; // indexing panics if the key is missing
        assert_eq!(score, 95);
        scores.remove("Daniel");
    }

    assert_eq!(scores.len(), 3);

    for (name, score) in &scores {
        println!("{name}: {score}");
    }
}
```

## Building from pairs

```rust
use std::collections::HashMap;

fn main() {
    let teams = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let map_a: HashMap<_, _> = HashMap::from(teams);
    let map_b: HashMap<_, _> = teams.into_iter().collect();

    assert_eq!(map_a, map_b);
}
```

## The Entry API: insert-if-missing without a second lookup

```rust
use std::collections::HashMap;

fn main() {
    let mut player_stats: HashMap<&str, u8> = HashMap::new();

    // Insert a default only if the key is not already present.
    player_stats.entry("health").or_insert(100);
    assert_eq!(player_stats["health"], 100);

    // or_insert again does nothing, since "health" already exists.
    player_stats.entry("health").or_insert_with(random_stat_buff);
    assert_eq!(player_stats["health"], 100);

    // or_insert returns a mutable reference into the map, so you can
    // modify the value in place.
    let health = player_stats.entry("health").or_insert(50);
    *health -= 50;
    assert_eq!(player_stats["health"], 50);
}

fn random_stat_buff() -> u8 {
    42 // pretend this rolls a random buff
}
```

## Using a struct as a key

Any type used as a `HashMap` key must implement `Eq` and `Hash` (and `PartialEq`, which `Eq` requires). The `#[derive(...)]` attribute handles this automatically.

```rust
use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    fn new(name: &str, country: &str) -> Viking {
        Viking { name: name.to_string(), country: country.to_string() }
    }
}

fn main() {
    let vikings = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);

    for (viking, health) in &vikings {
        println!("{viking:?} has {health} hp");
    }
}
```
