use super::header::Header;
use super::node::Node;
use super::Game;

use crate::Chess;

use pgn_reader::{Nag, RawComment};
use std::collections::HashMap;

use uuid::Uuid;

// Predecessor of Game struct
struct PartialGame {
    header: Header,
    opt_headers: HashMap<String, String>,

    initial_position: Chess,

    root: Node,

    node_map: HashMap<Uuid, Node>,

    variation_stack: Vec<Node>,
    in_variation: bool,

    starting_comment: Option<String>,
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
    type Result = Game;

    fn begin_game(&mut self) {
        let root = Node::new();
        let mut node_map: HashMap<Uuid, Node> = HashMap::new();
        node_map.insert(root.id(), root.clone());

        let variation_stack = vec![root.clone()];

        let inner = PartialGame {
            header: Header::default(),
            opt_headers: HashMap::new(),
            initial_position: Chess::new(),

            root,

            node_map,

            variation_stack,
            in_variation: false,

            starting_comment: None,
        };

        *self = GameVisitor::InGame { inner }
    }

    fn header(&mut self, key: &[u8], value: pgn_reader::RawHeader<'_>) {
        let inner = if let Some(val) = self.try_get_inner() {
            val
        } else {
            return;
        };

        if key == b"FEN" {
            let pos = shakmaty::fen::Fen::from_ascii(value.as_bytes())
                .ok()
                .and_then(|f| f.into_position(shakmaty::CastlingMode::Standard).ok());

            if let Some(pos) = pos {
                inner.initial_position = pos;
            }
        }

        let key = std::str::from_utf8(key).unwrap();
        let value = std::str::from_utf8(value.as_bytes()).unwrap();

        if !inner.header.parse(key, value) {
            inner.opt_headers.insert(key.to_string(), value.to_string());
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
        new_node.set_starting_comment(inner.starting_comment.clone());
        inner.node_map.insert(new_node.id(), new_node.clone());
        *cur_node = new_node;

        inner.starting_comment = None;
        inner.in_variation = true;
    }

    fn nag(&mut self, nag: Nag) {
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

        cur_node.push_nag(nag.0);
    }

    fn comment(&mut self, comment: RawComment<'_>) {
        let inner = if let Some(val) = self.try_get_inner() {
            val
        } else {
            return;
        };

        let comment = std::str::from_utf8(comment.as_bytes()).unwrap().to_string();

        let cur_node = if let Some(val) = inner.variation_stack.last_mut() {
            val
        } else {
            return;
        };

        if inner.in_variation // Regular comment
            || (cur_node.parent().is_none() && cur_node.variations().is_empty())
        {
            // Comment is before any move
            let new_comment = if let Some(val) = cur_node.comment() {
                format!("{} {}", val, comment)
            } else {
                comment
            };
            cur_node.set_comment(Some(new_comment));
            return;
        }

        let starting_comment = if let Some(val) = &inner.starting_comment {
            format!("{} {}", val, comment)
        } else {
            comment
        };
        inner.starting_comment = Some(starting_comment);
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
            return Game::default();
        };

        let header = inner.header.clone();
        let opt_headers = inner.opt_headers.clone();
        let initial_position = inner.initial_position.clone();

        let root = inner.root.clone();

        let node_map = inner.node_map.clone();

        *self = Self::None;

        Game {
            header,
            opt_headers,
            initial_position,

            root,

            node_map,
        }
    }
}

pub fn read_pgn(pgn: &str) -> std::io::Result<Game> {
    let mut reader = pgn_reader::BufferedReader::new_cursor(pgn);

    let mut visitor = GameVisitor::new();
    let game = reader.read_game(&mut visitor)?.unwrap();

    Ok(game)
}
