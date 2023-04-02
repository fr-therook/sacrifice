use super::node::Node;
use super::tree::GameTree;

use crate::Chess;

use std::collections::HashMap;

use uuid::Uuid;

// Predecessor of Game struct
struct PartialGame {
    headers: HashMap<String, String>,

    initial_position: Chess,

    root: Node,

    node_map: HashMap<Uuid, Node>,

    variation_stack: Vec<Node>,
    in_variation: bool,
}

enum GameVisitor {
    None,
    InGame { inner: PartialGame },
}

impl GameVisitor {
    fn new() -> Self {
        Self::None
    }
}

impl GameVisitor {
    fn try_get_inner(&mut self) -> Option<&mut PartialGame> {
        match self {
            GameVisitor::None => None,
            GameVisitor::InGame { inner: tree } => Some(tree),
        }
    }
}

impl pgn_reader::Visitor for GameVisitor {
    type Result = GameTree;

    fn begin_game(&mut self) {
        let root = Node::new();
        let mut node_map: HashMap<Uuid, Node> = HashMap::new();
        node_map.insert(root.id(), root.clone());

        let variation_stack = vec![root.clone()];

        let inner = PartialGame {
            headers: HashMap::new(),
            initial_position: Chess::new(),

            root,

            node_map,

            variation_stack,
            in_variation: false,
        };

        *self = GameVisitor::InGame { inner }
    }

    fn header(&mut self, key: &[u8], value: pgn_reader::RawHeader<'_>) {
        let inner = if let Some(val) = self.try_get_inner() {
            val
        } else {
            return;
        };

        inner.headers.insert(
            std::str::from_utf8(key).unwrap().to_string(),
            std::str::from_utf8(value.as_bytes()).unwrap().to_string(),
        );

        if key == b"FEN" {
            let pos = shakmaty::fen::Fen::from_ascii(value.as_bytes())
                .ok()
                .and_then(|f| f.into_position(shakmaty::CastlingMode::Standard).ok());

            if let Some(pos) = pos {
                inner.initial_position = pos;
            }
        }
    }

    fn san(&mut self, san_plus: shakmaty::san::SanPlus) {
        let inner = if let Some(val) = self.try_get_inner() {
            val
        } else {
            return;
        };

        let cur_node = if let Some(val) = inner.variation_stack.last_mut() {
            val
        } else {
            return;
        };

        let cur_position = cur_node.board(&inner.initial_position);

        let m = if let Ok(val) = san_plus.san.to_move(&cur_position) {
            val
        } else {
            return;
        };

        // A legal move
        let new_node = cur_node.new_variation(m);
        inner.node_map.insert(new_node.id(), new_node.clone());
        *cur_node = new_node;

        inner.in_variation = true;
    }

    fn begin_variation(&mut self) -> pgn_reader::Skip {
        let inner = if let Some(val) = self.try_get_inner() {
            val
        } else {
            return pgn_reader::Skip(true);
        };

        let cur_node = if let Some(val) = inner.variation_stack.last_mut() {
            val
        } else {
            return pgn_reader::Skip(true);
        };
        let variation_node = if let Some(val) = cur_node.parent() {
            val
        } else {
            println!("begin_variation called, but root node on top of stack");
            return pgn_reader::Skip(true);
        };

        inner.variation_stack.push(variation_node);
        inner.in_variation = false;

        pgn_reader::Skip(false)
    }

    fn end_variation(&mut self) {
        let inner = if let Some(val) = self.try_get_inner() {
            val
        } else {
            return;
        };

        inner.variation_stack.pop();
    }

    fn end_game(&mut self) -> Self::Result {
        let inner = if let Some(val) = self.try_get_inner() {
            val
        } else {
            return GameTree::default();
        };

        let headers = inner.headers.clone();
        let initial_position = inner.initial_position.clone();

        let root = inner.root.clone();

        let node_map = inner.node_map.clone();

        *self = Self::None;

        GameTree {
            headers,
            initial_position,

            root,

            node_map,
        }
    }
}

pub fn read_pgn(pgn: &str) -> std::io::Result<GameTree> {
    let mut reader = pgn_reader::BufferedReader::new_cursor(pgn);

    let mut visitor = GameVisitor::new();
    let game = reader.read_game(&mut visitor)?.unwrap();

    Ok(game)
}
