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

```sh
$ cargo add sacrifice
```

## Example

```rust
use sacrifice::prelude::*;

fn main() {
    let mut game = sacrifice::read_pgn(
        "1. e4 { this blunders into the Sicilian Defense }  1... c5"
    );
    println!("{}", game); // exports the PGN string

    // Play the Open Sicilian with 2. Nf3
    let open_sicilian = sacrifice::Move::Normal {
        role: sacrifice::Role::Knight,
        from: sacrifice::Square::G1,
        to: sacrifice::Square::F3,
        capture: None,
        promotion: None,
    };
    let new_node = game.add_node(game.root(), open_sicilian); // 2. Nf3 node
    println!("{}", game); // exports the PGN string after 2. Nf3

    // Take back the previous 2. Nf3 move
    game.remove_node(new_node);
    println!("{}", game);

    // What if someone want to play 1. d4?
    let queens_pawn = sacrifice::Move::Normal {
        role: sacrifice::Role::Pawn,
        from: sacrifice::Square::D2,
        to: sacrifice::Square::D4,
        capture: None,
        promotion: None,
    };
    let new_node = game.add_node(game.root(), queens_pawn); // 1. d4 node
    println!("{}", game); // 1. e4 (1. d4) 1... c5
}
```

## Features

For now, it supports

* PGN se/deserialization
* Comments
* NAG notations
