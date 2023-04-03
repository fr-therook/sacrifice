use super::writer::{Skip, Visitor};

use crate::{Chess, Move, Position};

use std::collections::HashSet;

use std::cell::RefCell;
use std::rc::Rc;

use uuid::Uuid;

#[derive(Clone)]
struct PrevInfo {
    node: Node,                       // parent node
    next_move: Move,                  // this node's previous move
    starting_comment: Option<String>, // Comment about starting a variation
    nag_set: HashSet<u8>,             // this node's nag attributes
}

#[derive(Clone)]
struct NodeImpl {
    prev: Option<PrevInfo>,
    variation_vec: Vec<Node>,
    comment: Option<String>,
}

// A node in the game tree.
#[derive(Clone)]
pub struct Node {
    // Actual relevant data, e.g. previous move, next moves
    inner: Rc<RefCell<NodeImpl>>,

    // Unique identifier for this node.
    // Akin to pointer, but safer and FFI-friendly.
    id: Uuid,
}

// Constructors
impl Node {
    pub fn new() -> Self {
        let inner = NodeImpl {
            prev: None,
            variation_vec: Vec::new(),
            comment: None,
        };
        Self::from_inner(inner)
    }

    pub fn from_node(node: Self, prev_move: Move) -> Self {
        let inner = NodeImpl {
            prev: Some(PrevInfo {
                node,
                next_move: prev_move,
                starting_comment: None,
                nag_set: HashSet::new(),
            }),
            variation_vec: Vec::new(),
            comment: None,
        };

        Self::from_inner(inner)
    }

    fn from_inner(inner: NodeImpl) -> Self {
        let inner = Rc::new(RefCell::new(inner));
        let id = Uuid::new_v4();

        Self { inner, id }
    }
}

// Getter/Setter methods
impl Node {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn parent(&self) -> Option<Node> {
        Some(self.inner.borrow().prev.clone()?.node)
    }

    pub fn prev_move(&self) -> Option<Move> {
        Some(self.inner.borrow().prev.clone()?.next_move)
    }

    pub fn mainline(&self) -> Option<Node> {
        self.inner.borrow().variation_vec.get(0).cloned()
    }

    pub fn variations(&self) -> Vec<Node> {
        self.inner.borrow().variation_vec.clone()
    }

    pub fn new_variation(&mut self, m: Move) -> Node {
        let next_node = Node::from_node(self.clone(), m);
        self.inner
            .borrow_mut()
            .variation_vec
            .push(next_node.clone());
        next_node
    }

    pub fn remove_variation(&mut self, node: Node) -> bool {
        let variations = &mut self.inner.borrow_mut().variation_vec;
        let variations_size = variations.len();
        variations.retain(|v| v.id != node.id);
        variations.len() < variations_size
    }

    pub fn promote_variation(&mut self, node: Node) -> bool {
        let variations = &mut self.inner.borrow_mut().variation_vec;
        let variations_size = variations.len();
        variations.retain(|v| v.id != node.id);
        if variations.len() == variations_size {
            // not its own children
            println!(
                "attempting to promote disconnected node {} in parent node {}",
                node.id, self.id
            );
            return false;
        }

        // TODO: Use node directly from variation_vec
        variations.insert(0, node);

        true
    }

    pub fn starting_comment(&self) -> Option<String> {
        self.inner.borrow().prev.clone()?.starting_comment
    }

    pub fn set_starting_comment(&self, new_comment: Option<String>) {
        if let Some(prev) = self.inner.borrow_mut().prev.as_mut() {
            prev.starting_comment = new_comment;
        }
    }

    pub fn nags(&self) -> Vec<u8> {
        if let Some(prev) = &self.inner.borrow().prev {
            return prev.nag_set.iter().copied().collect();
        }

        Vec::new()
    }

    pub fn push_nag(&self, nag: u8) {
        if let Some(prev) = self.inner.borrow_mut().prev.as_mut() {
            prev.nag_set.insert(nag);
        }
    }

    pub fn pop_nag(&self, nag: u8) {
        if let Some(prev) = self.inner.borrow_mut().prev.as_mut() {
            prev.nag_set.remove(&nag);
        }
    }

    pub fn clear_nags(&self) {
        if let Some(prev) = self.inner.borrow_mut().prev.as_mut() {
            prev.nag_set.clear()
        }
    }

    pub fn comment(&self) -> Option<String> {
        self.inner.borrow().comment.clone()
    }

    pub fn set_comment(&self, new_comment: Option<String>) {
        self.inner.borrow_mut().comment = new_comment;
    }
}

// Methods that traverse the tree
impl Node {
    // Find the root node by traversing up the tree
    #[allow(dead_code, unused_variables)]
    pub fn root(&self) -> Node {
        let mut node: Node = self.clone();
        while let Some(parent) = node.parent() {
            node = parent;
        }
        node
    }

    // Returns the number of half-moves from root to this node.
    #[allow(dead_code, unused_variables)]
    pub fn depth(&self) -> u32 {
        let mut result: u32 = 0;

        let mut node: Node = self.clone();
        while let Some(parent) = node.parent() {
            result += 1;
            node = parent;
        }
        result
    }

    pub fn moves(&self) -> Vec<Move> {
        let mut move_vec: Vec<Move> = Vec::new();

        let mut node: Node = self.clone();
        while let Some(prev_info) = (node.inner.clone().borrow().prev).clone() {
            let parent = prev_info.node;
            let prev_move = prev_info.next_move;

            move_vec.push(prev_move);

            node = parent;
        }

        move_vec.reverse();

        move_vec
    }

    pub fn board(&self, initial_position: &Chess) -> Chess {
        let mut board = initial_position.clone();

        let move_vec = self.moves();
        for _move in move_vec {
            board.play_unchecked(&_move);
        }

        board
    }
}

impl Node {
    fn accept_inner<V: Visitor>(&self, prev_position: &Chess, visitor: &mut V) {
        if let Some(starting_comment) = self.starting_comment() {
            visitor.visit_comment(starting_comment);
        }

        // Visit the mainline node first
        visitor.visit_move(prev_position.clone(), self.prev_move().unwrap());

        for nag in self.nags() {
            visitor.visit_nag(nag);
        }

        if let Some(comment) = self.comment() {
            visitor.visit_comment(comment);
        }
    }

    pub fn accept<V: Visitor>(&self, initial_position: &Chess, visitor: &mut V) {
        // Return if there's no child nodes
        let main_node = if let Some(val) = self.mainline() {
            val
        } else {
            return;
        };

        let prev_position = self.board(initial_position);

        main_node.accept_inner(&prev_position, visitor);

        // Visit variation nodes after
        let mut variation_node_vec = self.inner.borrow().variation_vec.clone();
        variation_node_vec.remove(0);
        for variation_node in variation_node_vec {
            if let Skip(true) = visitor.begin_variation() {
                continue; // Skip this variation
            }

            variation_node.accept_inner(&prev_position, visitor);

            // Recursively visiting variation node
            variation_node.accept(initial_position, visitor);

            visitor.end_variation();
        }

        // Visit mainline recursively last
        main_node.accept(initial_position, visitor);
    }
}
