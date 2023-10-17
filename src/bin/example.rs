fn main() {
    let mut game = sacrifice::read_pgn(
        "1. e4 { this blunders into the Sicilian Defense }  1... c5"
    );
    println!("{}", game); // exports the PGN string

    let mut root = game.root();

    // Play the Open Sicilian with 2. Nf3
    let open_sicilian = sacrifice::Move::Normal {
        role: sacrifice::Role::Knight,
        from: sacrifice::Square::G1,
        to: sacrifice::Square::F3,
        capture: None,
        promotion: None,
    };
    let mut new_node = root.new_variation(open_sicilian).unwrap(); // 2. Nf3 node
    println!("{}", game); // exports the PGN string after 2. Nf3

    // Take back the previous 2. Nf3 move
    new_node.remove_node();
    println!("{}", game);

    // What if someone want to play 1. d4?
    let queens_pawn = sacrifice::Move::Normal {
        role: sacrifice::Role::Pawn,
        from: sacrifice::Square::D2,
        to: sacrifice::Square::D4,
        capture: None,
        promotion: None,
    };
    let new_node = root.new_variation(queens_pawn); // 1. d4 node
    println!("{}", game); // 1. e4 (1. d4) 1... c5
}
