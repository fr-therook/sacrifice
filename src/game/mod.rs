mod node;
mod pgn;
mod writer;

use std::collections::HashMap;
use shakmaty::Position;
use node::Node;
use crate::game::writer::{Visitor, PgnWriter};
use crate::Move;

pub struct Game {
    headers: HashMap<String, String>,

    root: Node,

    initial_position: shakmaty::Chess,
    cur_node: Node,
}

impl Game {
    pub fn new() -> Self {
        let headers = HashMap::new();
        let root = Node::new();
        let initial_position = shakmaty::Chess::default();
        let cur_node = root.clone();

        Self {
            headers,
            root,
            initial_position,
            cur_node,
        }
    }

    pub fn from_pos(initial_position: shakmaty::Chess) -> Self {
        let headers = HashMap::new();
        let root = Node::new();
        let cur_node = root.clone();

        Self {
            headers,
            root,
            initial_position,
            cur_node,
        }
    }

    pub fn from_pgn(pgn_str: &str) -> Self {
        pgn::read_pgn(pgn_str).unwrap()
    }
}

impl Game {
    pub fn headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }

    pub fn root(&self) -> Node {
        self.root.clone()
    }

    pub fn board(&self) -> shakmaty::Chess {
        let mut board = self.initial_position.clone();

        let move_vec = self.cur_node.moves();
        for _move in move_vec {
            board.play_unchecked(&_move);
        }

        board
    }

    pub fn push_main_variation(&mut self, _move: Move) {
        let new_node = Node::from_node(self.cur_node.clone(), _move);
        self.cur_node.set_mainline(new_node.clone());
        self.cur_node = new_node;
    }
}

impl Game {
    pub fn print_mainline(&self) {
        let mut visitor = PgnWriter::with_max_width(64);
        let line_vec = self.accept(&mut visitor);

        for line in line_vec {
            println!("{}", line);
        }
    }

    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result {
        visitor.begin_game();

        visitor.begin_headers();
        for (key, value) in &self.headers {
            visitor.visit_header(key, value);
        }
        visitor.end_headers();

        self.root.accept(
            &self.initial_position,
            visitor
        );

        visitor.end_game()
    }
}

#[cfg(test)]
mod tests {
    use crate::game::Game;

    #[test]
    fn pgn() {
        let pgn_str = r#"
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

1. d4 d5 2. Bf4 c5 {D00 Queen's Pawn Game: Accelerated London System, Steinitz
Countergambit} 3. e3 Nc6 4. dxc5 e5 5. Bg3 Bxc5 6. Bb5 Ne7 7. Bxe5 O-O 8. Nf3
Bg4 (8... Nxe5 9. Nxe5 Qa5+ 10. Nc3 Bb4 11. O-O Bxc3 12. bxc3 Qxb5) 9. h3 Bxf3 $1
10. Qxf3 Nxe5 11. Qf4 N7g6 12. Qg3 Bd6 (12... Qa5+ 13. Nd2 Qxb5) 13. f4 Qh4 14.
Qxh4 Nxh4 15. fxe5 Bxe5 16. c3 Nxg2+ 17. Kf2 Nh4 18. Nd2 Ng6 {Black offers draw}
19. Nf3 Bd6 20. Rad1 a6 21. Bd3 Rad8 22. Bxg6 fxg6 23. Rxd5 Bg3+ 24. Kxg3 Rxd5
25. e4 Rd3 26. Rf1 h5 27. h4 Kf7 28. e5 Ke6 29. Kg2 Rfxf3 30. Rxf3 Rxf3 31. Kxf3
Kxe5 32. Ke3 g5 33. hxg5 h4 34. Kf3 Kf5 35. Kg2 Kxg5 36. Kh3 g6 37. b4 b5 38. a3
Kh5 39. Kh2 Kg4 40. Kg2 h3+ 41. Kh2 g5 42. Kh1 Kg3 43. Kg1 g4 44. Kh1 Kf3 45.
Kg1 g3 46. Kh1 g2+ 47. Kg1 h2+ 48. Kxh2 Kf2 49. Kh3 g1=Q 50. Kh4 Kf3 51. Kh5 Kf4
52. Kh6 Kf5 53. Kh7 Kf6 54. Kh6 Qg6# {Black wins by checkmate.} 0-1
"#;

        let game = Game::from_pgn(pgn_str);
        game.print_mainline();

        assert!(false)
    }
}
