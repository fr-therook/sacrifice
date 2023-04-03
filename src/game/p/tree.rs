use super::header::Header;
use super::node::Node;
use super::reader::read_pgn;
use super::writer::{PgnWriter, Visitor};

use crate::{Chess, Move};

use std::collections::HashMap;
use uuid::Uuid;

// A simple BTree data structure, plus header and initial position.
// It also holds a hashmap for quick node lookup.
pub struct GameTree {
    pub header: Header,
    pub initial_position: Chess,
    pub opt_headers: HashMap<String, String>,

    pub root: Node,

    pub node_map: HashMap<Uuid, Node>,
}

impl Default for GameTree {
    fn default() -> Self {
        let header = Header::default();
        let opt_headers = HashMap::new();
        let initial_position = Chess::default();

        let root = Node::new();

        let node_map = HashMap::new();

        Self {
            header,
            opt_headers,
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
    fn try_node(&self, id: Uuid) -> Option<Node> {
        self.node_map.get(&id).cloned()
    }

    pub fn root(&self) -> Uuid {
        self.root.id()
    }

    pub fn exists(&self, node_id: Uuid) -> Option<Uuid> {
        let node = self.try_node(node_id)?;
        assert_eq!(node.id(), node_id, "id-node hashmap outdated");
        Some(node.id())
    }

    pub fn parent(&self, node_id: Uuid) -> Option<Uuid> {
        let node = self.try_node(node_id)?;
        node.parent().map(|val| val.id())
    }

    pub fn children(&self, node_id: Uuid) -> Vec<Uuid> {
        let node = if let Some(val) = self.try_node(node_id) {
            val
        } else {
            return vec![];
        };
        node.variations()
            .into_iter()
            .map(|val| val.id())
            .collect::<Vec<Uuid>>()
    }

    pub fn prev_move(&self, node_id: Uuid) -> Option<Move> {
        let node = self.try_node(node_id)?;
        node.prev_move()
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

        // This always ends with \n.
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
        {
            self.header.accept(visitor);

            for (key, value) in &self.opt_headers {
                visitor.visit_header(key, value);
            }
        }
        visitor.end_headers();

        if let Some(comment) = self.root.comment() {
            // Game comment
            visitor.visit_comment(comment);
        }

        self.root.accept(&self.initial_position, visitor);

        let result = self.header.result.to_string();
        visitor.visit_result(result.as_str());

        visitor.end_game()
    }
}
