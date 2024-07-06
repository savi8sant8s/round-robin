## Round-robin algorithm
Create a vector of rounds using the Round-robin algorithm.

#### Example using strings:
```rust
let teams = vec!["A", "B", "C", "D"];
let rounds = generate_rounds(teams);
// [("A", "D"), ("C", "B"), ("A", "C"), ("B", "D"), ("A", "C"), ("B", "D")]
```
#### Example using numbers:
```rust
let teams = vec![1, 2, 3, 4];
let rounds = generate_rounds(teams);
// [(1, 4), (3, 2), (1, 3), (2, 4), (1, "C"), (2, 4)]
```
#### Example using structs:
```rust
#[derive(Debug, Clone)]
struct Team { name: &'static str }
let teams = vec![
    Team { name: "Liverpool" }, Team { name: "Chelsea" },
    Team { name: "M. City" }, Team { name: "M. United" },
];
let rounds = generate_rounds(teams);
// [(Team { name: "Liverpool" }, Team { name: "M. United" }), (Team { name: "M. City" }, Team { name: "Chelsea" }), (Team { name: "Liverpool" }, Team { name: "M. City" }), (Team { name: "Chelsea" }, Team { name: "M. United" }), (Team { name: "Liverpool" }, Team { name: "M. City" }), (Team { name: "Chelsea" }, Team { name: "M. United" })]