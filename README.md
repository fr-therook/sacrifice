# sacrifice

[
  ![crates.io version](
  https://img.shields.io/crates/v/sacrifice?color=red&logo=rust&label=crates.io%2Fsacrifice&style=flat-square
  )
](https://crates.io/crates/sacrifice)
[
  ![docs.rs version](
  https://img.shields.io/crates/v/sacrifice?color=blue&logo=docs.rs&label=docs.rs%2Fsacrifice&style=flat-square
  )
](https://docs.rs/sacrifice/)

A feature-rich chess library for Rust based on
[
![crates.io version](
https://img.shields.io/crates/v/shakmaty?color=red&logo=rust&label=crates.io%2Fshakmaty&style=flat-square
)
](https://crates.io/crates/shakmaty).

```
sac = { package = "sacrifice", version = "0.3.0-alpha.2" }
```

## Example

```rust
fn main() {
    let mut game = sac::read_pgn(
        "1. e4 { this blunders into the Sicilian Defense }  1... c5"
    );
    println!("{}", game); // exports the PGN string
  
    let mut root = game.root();

    // Play the Open Sicilian with 2. Nf3
    let open_sicilian = sac::Move::Normal {
        role: sac::Role::Knight,
        from: sac::Square::G1,
        to: sac::Square::F3,
        capture: None,
        promotion: None,
    };
    let mut new_node = root.new_variation(open_sicilian).unwrap(); // 2. Nf3 node
    println!("{}", game); // exports the PGN string after 2. Nf3

    // Take back the previous 2. Nf3 move
    new_node.remove_node();
    println!("{}", game);

    // What if someone want to play 1. d4?
    let queens_pawn = sac::Move::Normal {
        role: sac::Role::Pawn,
        from: sac::Square::D2,
        to: sac::Square::D4,
        capture: None,
        promotion: None,
    };
    let new_node = root.new_variation(queens_pawn); // 1. d4 node
    println!("{}", game); // 1. e4 (1. d4) 1... c5
}
```

## Features

For now, it supports

* Game tree traversal
* PGN se/deserialization
* Comments
* NAG notations
