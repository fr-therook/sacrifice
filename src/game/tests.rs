use super::Game;
use crate::{Move, Role, Square};

const GAME_0: &str = r#"
[Event "Casual Rapid game"]
[Site "https://lichess.org/5uSupub7"]
[Date "2023.03.06"]
[Round "?"]
[White "maia1"]
[Black "soyflourbread"]
[Result "0-1"]
[UTCDate "2023.03.06"]
[UTCTime "00:32:56"]
[WhiteElo "1537"]
[BlackElo "1500"]
[WhiteTitle "BOT"]
[TimeControl "600+3"]
[ECO "D00"]
[Opening "Queen's Pawn Game: Accelerated London System, Steinitz Countergambit"]
[Termination "Normal"]
[Annotator "lichess.org"]

{Chess, when played perfectly, ends in a draw}
1. d4 {The best opening move}
(1. e4 {This blunder allows the Sicilian Defense} 1... c5)
1... d5 2. Bf4 c5 {D00 Queen's Pawn Game: Accelerated London System, Steinitz Countergambit}
3. e3 Nc6 4. dxc5 e5 5. Bg3 Bxc5 6. Bb5 Ne7 7. Bxe5 O-O 8. Nf3 Bg4 $2
({Apparently this is best} 8... Nxe5 9. Nxe5 Qa5+ 10. Nc3 Bb4 11. O-O Bxc3 12. bxc3 Qxb5)
9. h3 $2 Bxf3 $3 10. Qxf3 Nxe5 11. Qf4 N7g6 12. Qg3 Bd6
(12... Qa5+ 13. Nd2 Qxb5)
13. f4 Qh4 14. Qxh4 Nxh4 15. fxe5 Bxe5 16. c3 Nxg2+ 17. Kf2 Nh4 18. Nd2 Ng6
{Black offers draw}
19. Nf3 Bd6 20. Rad1 a6 21. Bd3 Rad8 22. Bxg6 fxg6 23. Rxd5 Bg3+ 24. Kxg3 Rxd5
25. e4 Rd3 26. Rf1 h5 27. h4 Kf7 28. e5 Ke6 29. Kg2 Rfxf3 30. Rxf3 Rxf3 31. Kxf3
Kxe5 32. Ke3 g5 33. hxg5 h4 34. Kf3 Kf5 35. Kg2 Kxg5 36. Kh3 g6 37. b4 b5 38. a3
Kh5 39. Kh2 Kg4 40. Kg2 h3+ 41. Kh2 g5 42. Kh1 Kg3 43. Kg1 g4 44. Kh1 Kf3 45.
Kg1 g3 46. Kh1 g2+ 47. Kg1 h2+ 48. Kxh2 Kf2 49. Kh3 g1=Q 50. Kh4 Kf3 51. Kh5 Kf4
52. Kh6 Kf5 53. Kh7 Kf6 54. Kh6 Qg6#
{Black wins by checkmate.}
0-1
"#;

#[test]
fn pgn() {
    let game = Game::from_pgn(GAME_0);
    println!("----Begin PGN----");
    println!("{:64}", game);
    println!("----End PGN----");
}

const FOOLS_MOVES: [Move; 4] = [
    Move::Normal {
        role: Role::Pawn,
        from: Square::F2,
        capture: None,
        to: Square::F3,
        promotion: None,
    },
    Move::Normal {
        role: Role::Pawn,
        from: Square::E7,
        capture: None,
        to: Square::E5,
        promotion: None,
    },
    Move::Normal {
        role: Role::Pawn,
        from: Square::G2,
        capture: None,
        to: Square::G4,
        promotion: None,
    },
    Move::Normal {
        role: Role::Queen,
        from: Square::D8,
        capture: None,
        to: Square::H4,
        promotion: None,
    },
];

#[test]
fn node_add() {
    let mut game = Game::default();
    let mut node_id = game.root();
    let mut node_id_vec = vec![node_id];
    for m in FOOLS_MOVES {
        node_id = game.add_node(node_id, m).unwrap();
        node_id_vec.push(node_id);
    }

    assert_ne!(game.exists(node_id_vec[0]), None);
    assert_ne!(game.exists(node_id_vec[1]), None);
    assert_ne!(game.exists(node_id_vec[2]), None);
    assert_ne!(game.exists(node_id_vec[3]), None);
    assert_ne!(game.exists(node_id_vec[4]), None);
    assert_eq!(game.node_map.keys().len(), 5);
}

#[test]
fn node_del() {
    let mut game = Game::default();
    let mut node_id = game.root();
    let mut node_id_vec = vec![node_id];
    for m in FOOLS_MOVES {
        node_id = game.add_node(node_id, m).unwrap();
        node_id_vec.push(node_id);
    }
    game.add_node(
        node_id_vec[3],
        Move::Normal {
            role: Role::Pawn,
            from: Square::D7,
            capture: None,
            to: Square::D5,
            promotion: None,
        },
    );

    game.remove_node(node_id_vec[3]); // 2. g4
    assert_ne!(game.exists(node_id_vec[0]), None);
    assert_ne!(game.exists(node_id_vec[1]), None);
    assert_ne!(game.exists(node_id_vec[2]), None);

    assert_eq!(game.exists(node_id_vec[3]), None);
    assert_eq!(game.exists(node_id_vec[4]), None);
    assert_eq!(game.node_map.keys().len(), 3); // root node, 1. f3 and 1 ...e5
}

#[test]
fn node_promote() {
    let mut game = Game::default();
    let mut node_id = game.root();
    let mut node_id_vec = vec![node_id];
    for m in FOOLS_MOVES {
        node_id = game.add_node(node_id, m).unwrap();
        node_id_vec.push(node_id);
    }
    let promote_node_id = game
        .add_node(
            node_id_vec[3],
            Move::Normal {
                role: Role::Pawn,
                from: Square::D7,
                capture: None,
                to: Square::D5,
                promotion: None,
            },
        )
        .unwrap();
    assert_eq!(
        game.promote_variation(promote_node_id),
        Some(promote_node_id)
    );

    println!("----Begin PGN----");
    println!("{:64}", game);
    println!("----End PGN----");
}
