mod header;
mod node;

mod reader;
mod writer;

use node::Node;

use header::Header;
use reader::read_pgn;

use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

use crate::{Chess, Move};

// A simple BTree data structure, plus header and initial position.
// It also holds a hashmap for quick node lookup.
pub struct Game {
    header: Header,
    initial_position: Chess,
    opt_headers: HashMap<String, String>,

    root: Node,

    node_map: HashMap<Uuid, Node>,
}

impl Default for Game {
    fn default() -> Self {
        let header = Header::default();
        let opt_headers = HashMap::new();
        let initial_position = Chess::default();

        let root = Node::new();

        let mut node_map = HashMap::new();
        node_map.insert(root.id(), root.clone());

        Self {
            header,
            opt_headers,
            initial_position,

            root,

            node_map,
        }
    }
}

impl Game {
    pub fn from_pgn(pgn_str: &str) -> Self {
        read_pgn(pgn_str).unwrap()
    }
}

// Accessing/manipulating a single node.
impl Game {
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

    pub fn prev_move(&self, node_id: Uuid) -> Option<Move> {
        let node = self.try_node(node_id)?;
        node.prev_move()
    }

    pub fn mainline(&self, node_id: Uuid) -> Option<Uuid> {
        let node = self.try_node(node_id)?;
        node.mainline().map(|v| v.id())
    }

    pub fn other_variations(&self, node_id: Uuid) -> Vec<Uuid> {
        let node = if let Some(val) = self.try_node(node_id) {
            val
        } else {
            return vec![];
        };
        let mut variations = node
            .variations()
            .into_iter()
            .map(|val| val.id())
            .collect::<Vec<Uuid>>();
        variations.remove(0);
        variations
    }

    pub fn starting_comment(&self, node_id: Uuid) -> Option<String> {
        let node = self.try_node(node_id)?;
        node.starting_comment()
    }

    pub fn set_starting_comment(&self, node_id: Uuid, new_comment: Option<String>) {
        if let Some(node) = self.try_node(node_id) {
            node.set_starting_comment(new_comment)
        }
    }

    pub fn nags(&self, node_id: Uuid) -> Vec<u8> {
        if let Some(node) = self.try_node(node_id) {
            return node.nags();
        }

        Vec::new()
    }

    pub fn set_nags(&self, node_id: Uuid, nag_vec: Vec<u8>) {
        if let Some(node) = self.try_node(node_id) {
            node.clear_nags();

            for nag in nag_vec {
                node.push_nag(nag);
            }
        }
    }

    pub fn comment(&self, node_id: Uuid) -> Option<String> {
        let node = self.try_node(node_id)?;
        node.comment()
    }

    pub fn set_comment(&self, node_id: Uuid, new_comment: Option<String>) {
        if let Some(node) = self.try_node(node_id) {
            node.set_comment(new_comment)
        }
    }

    pub fn board_at(&self, node_id: Uuid) -> Option<Chess> {
        let node = self.try_node(node_id)?;
        Some(node.board(&self.initial_position))
    }

    pub fn moves_before(&self, node_id: Uuid) -> Vec<Move> {
        if let Some(node) = self.try_node(node_id) {
            return node.moves();
        }

        Vec::new()
    }
}

// Methods that changes the order of branches in the tree
impl Game {
    pub fn promote_variation(&mut self, node_id: Uuid) -> Option<Uuid> {
        let node = self.try_node(node_id)?;
        let mut parent = if let Some(val) = node.parent() {
            val
        } else {
            println!(
                "node {} has no parent - attempting to delete root node?",
                node_id
            );
            return None;
        };

        if parent.promote_variation(node) {
            return Some(node_id);
        }

        None
    }
}

// Methods that manipulate the node tree and node map
impl Game {
    pub fn add_node(&mut self, parent_id: Uuid, m: Move) -> Option<Uuid> {
        let mut parent = self.try_node(parent_id)?;
        let new_node = parent.new_variation(m);
        self.node_map.insert(new_node.id(), new_node.clone()); // Update node map
        Some(new_node.id())
    }

    pub fn remove_node(&mut self, node_id: Uuid) -> Option<Uuid> {
        let node = self.try_node(node_id)?;
        let mut parent = if let Some(val) = node.parent() {
            val
        } else {
            println!(
                "node {} has no parent - attempting to delete root node?",
                node_id
            );
            return None;
        };

        // Remove this node and its children from node map
        {
            let mut node_queue: VecDeque<Node> = VecDeque::from([node.clone()]);
            while !node_queue.is_empty() {
                let node = node_queue.pop_front().unwrap();
                self.node_map.remove(&node.id());
                for variation_node in node.variations() {
                    node_queue.push_back(variation_node);
                }
            }
        }

        // Remove this node from its parent
        if parent.remove_variation(node) {
            return Some(node_id);
        }

        // How did we get here?
        println!(
            "node {} has parent {}, yet is not its child",
            node_id,
            parent.id()
        );

        None
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut visitor = if let Some(max_width) = f.width() {
            writer::PgnWriter::with_max_width(max_width as u32)
        } else {
            writer::PgnWriter::new()
        };
        let line_vec = self.accept(&mut visitor);

        // This always ends with \n.
        for line in line_vec {
            writeln!(f, "{}", line)?;
        }

        Ok(())
    }
}

impl Game {
    fn accept<V: writer::Visitor>(&self, visitor: &mut V) -> V::Result {
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

#[cfg(test)]
mod tests;
