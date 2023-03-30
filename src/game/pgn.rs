use std::collections::HashMap;
use crate::game::Game;
use crate::game::node::Node;

// Predecessor of Game struct
struct GameTree {
    headers: HashMap<String, String>,

    initial_position: shakmaty::Chess,

    root: Node,
    variation_stack: Vec<Node>,
    in_variation: bool,
}

enum GameVisitor {
    None,
    InGame { tree: GameTree },
}

impl GameVisitor {
    fn new() -> Self {
        Self::None
    }
}

impl GameVisitor {
    fn try_get_tree(&mut self) -> Option<&mut GameTree> {
        match self {
            GameVisitor::None => None,
            GameVisitor::InGame { tree } => Some(tree)
        }
    }
}

impl pgn_reader::Visitor for GameVisitor {
    type Result = Game;

    fn begin_game(&mut self) {
        let root = Node::new();
        let variation_stack = vec![root.clone()];

        let tree = GameTree {
            headers: HashMap::new(),
            initial_position: shakmaty::Chess::new(),
            root,
            variation_stack,
            in_variation: false,
        };

        *self = GameVisitor::InGame { tree }
    }

    fn header(&mut self, key: &[u8], value: pgn_reader::RawHeader<'_>) {
        let tree = if let Some(val) =
            self.try_get_tree() { val } else { return; };

        tree.headers.insert(
            std::str::from_utf8(key).unwrap().to_string(),
            std::str::from_utf8(value.as_bytes()).unwrap().to_string()
        );

        if key == b"FEN" {
            let pos = shakmaty::fen::Fen::from_ascii(value.as_bytes()).ok()
                .and_then(|f| f.into_position(shakmaty::CastlingMode::Standard).ok());

            if let Some(pos) = pos {
                tree.initial_position = pos;
            }
        }
    }

    fn san(&mut self, san_plus: shakmaty::san::SanPlus) {
        let tree = if let Some(val) =
            self.try_get_tree() { val } else { return; };

        let cur_node = if let Some(val) =
            tree.variation_stack.last_mut() { val } else { return; };

        let cur_position = cur_node.board(&tree.initial_position);

        let m = if let Ok(val) =
            san_plus.san.to_move(&cur_position) { val } else { return; };

        // A legal move
        let next_node = cur_node.new_variation(m);
        std::mem::replace(cur_node, next_node);
        tree.in_variation = true;
    }

    fn begin_variation(&mut self) -> pgn_reader::Skip {
        let tree = if let Some(val) =
            self.try_get_tree() { val } else { return pgn_reader::Skip(true); };

        let cur_node = if let Some(val) =
            tree.variation_stack.last_mut() { val } else { return pgn_reader::Skip(true); };
        let variation_node = if let Some(val) =
            cur_node.parent() { val } else {
            println!("begin_variation called, but root node on top of stack");
            return pgn_reader::Skip(true);
        };

        tree.variation_stack.push(variation_node);
        tree.in_variation = false;

        pgn_reader::Skip(false)
    }

    fn end_variation(&mut self) {
        let tree = if let Some(val) =
            self.try_get_tree() { val } else { return; };

        tree.variation_stack.pop();
    }

    fn end_game(&mut self) -> Self::Result {
        let tree = if let Some(val) =
            self.try_get_tree() { val } else { return Game::new(); };

        let headers = tree.headers.clone();
        let root = tree.root.clone();
        let initial_position = tree.initial_position.clone();
        let cur_node = root.clone();

        Game {
            headers,
            root,
            initial_position,
            cur_node,
        }
    }
}

pub fn read_pgn(pgn: &str) -> std::io::Result<Game> {
    let mut reader = pgn_reader::BufferedReader::new_cursor(&pgn[..]);

    let mut visitor = GameVisitor::new();
    let game = reader.read_game(&mut visitor)?.unwrap();

    Ok(game)
}
