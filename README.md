# sacrifice

[![crates.io version](https://img.shields.io/crates/v/sacrifice?style=flat-square)](https://crates.io/crates/sacrifice)
[![docs.rs version](https://img.shields.io/badge/docs.rs-sacrifice-green?style=flat-square)](https://docs.rs/sacrifice/)

A feature-rich chess library for Rust based on
[shakmaty](https://docs.rs/shakmaty/latest/shakmaty/).

```sh
$ cargo add sacrifice
```

## Example

```rust
let mut game = sacrifice::Game::from_pgn(
  "1. e4 { This blunders into the Sicilian Defense }  1... c5"
);
println!("{}", game); // exports the PGN string

// Play the Open Sicilian
let open_sicilian = sacrifice::Move::Normal {
  role: sacrifice::Role::Knight,
  from: sacrifice::Square::G1,
  capture: None,
  to: sacrifice::Square::F3,
  promotion: None,
};
let new_node = game.add_node(game.root(), open_sicilian);
println!("{}", game); // exports the PGN string after 2. Nf3

// Take back 2. Nf3 move
game.remove_node(new_node);
println!("{}", game);

// What if someone want to play 1. d4?
let queens_pawn = sacrifice::Move::Normal {
role: sacrifice::Role::Pawn,
from: sacrifice::Square::D2,
capture: None,
to: sacrifice::Square::D4,
promotion: None,
};
let new_node = game.add_node(game.root(), queens_pawn);
println!("{}", game); // 1. e4 (1. d4) 1... c5

// Promote 1. d4
game.promote_variation(new_node);
println!("{}", game); // 1. d4 (1. e4 c5)
```

## Features

Currently it supports

* PGN se/deserialization
* Comments
* NAG notations
