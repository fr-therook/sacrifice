use super::node::Node;
use super::reader::read_pgn;
use super::writer::{Visitor, PgnWriter};

use crate::Chess;

use std::collections::HashMap;
use uuid::Uuid;

// A simple BTree data structure, plus header and initial position.
// It also holds a hashmap for quick node lookup.
pub struct GameTree {
    pub headers: HashMap<String, String>,
    pub initial_position: Chess,

    pub root: Node,

    pub node_map: HashMap<Uuid, Node>,
}

impl Default for GameTree {
    fn default() -> Self {
        let headers = HashMap::new();
        let initial_position = Chess::default();

        let root = Node::new();

        let node_map = HashMap::new();

        Self {
            headers,
            initial_position,

            root,

            node_map,
        }
    }
}

impl GameTree {
    pub fn from_pgn(pgn_str: &str) -> Self {
        read_pgn(pgn_str).unwrap()
    }
}

impl GameTree {
    pub fn node_from_id(&self, id: Uuid) -> Option<Node> {
        self.node_map.get(&id).cloned()
    }
}

impl std::fmt::Display for GameTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut visitor = if let Some(max_width) = f.width() {
            PgnWriter::with_max_width(max_width as u32)
        } else {
            PgnWriter::new()
        };
        let line_vec = self.accept(&mut visitor);

        for line in line_vec {
            writeln!(f, "{}", line)?;
        }

        Ok(())
    }
}

impl GameTree {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result {
        visitor.begin_game();

        visitor.begin_headers();
        for (key, value) in &self.headers {
            visitor.visit_header(key, value);
        }
        visitor.end_headers();

        self.root.accept(
            &self.initial_position,
            visitor,
        );

        visitor.end_game()
    }
}