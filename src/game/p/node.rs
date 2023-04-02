use super::writer::{Skip, Visitor};

use crate::{Chess, Move, Position};

use std::cell::RefCell;
use std::rc::Rc;

use uuid::Uuid;

#[derive(Clone)]
struct PrevInfo {
    node: Node,
    // might be the root node
    next_move: Move,
}

#[derive(Clone)]
struct NodeImpl {
    prev: Option<PrevInfo>,
    variation_vec: Vec<Node>,
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

impl Node {
    pub fn new() -> Self {
        let inner = NodeImpl {
            prev: None,
            variation_vec: Vec::new(),
        };
        Self::from_inner(inner)
    }

    pub fn from_node(node: Self, prev_move: Move) -> Self {
        let inner = NodeImpl {
            prev: Some(PrevInfo {
                node,
                next_move: prev_move,
            }),
            variation_vec: Vec::new(),
        };

        Self::from_inner(inner)
    }
}

impl Node {
    fn from_inner(inner: NodeImpl) -> Self {
        let inner = Rc::new(RefCell::new(inner));
        let id = Uuid::new_v4();

        Self { inner, id }
    }
}

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

    pub fn variations(&self) -> Vec<Node> {
        self.inner.borrow().variation_vec.clone()
    }

    pub fn mainline(&self) -> Option<Node> {
        self.inner.borrow().variation_vec.get(0).cloned()
    }

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
    pub fn new_variation(&mut self, m: Move) -> Node {
        let next_node = Node::from_node(self.clone(), m);
        self.inner
            .borrow_mut()
            .variation_vec
            .push(next_node.clone());
        next_node
    }
}

impl Node {
    pub fn accept<V: Visitor>(&self, initial_position: &Chess, visitor: &mut V) {
        // Return if there's no child nodes
        let main_node = if let Some(val) = self.mainline() {
            val
        } else {
            return;
        };

        // Visit the mainline node first
        visitor.visit_move(self.board(initial_position), main_node.prev_move().unwrap());

        // Visit variation nodes after
        let mut variation_node_vec = self.inner.borrow().variation_vec.clone();
        variation_node_vec.remove(0);
        for variation_node in variation_node_vec {
            if let Skip(true) = visitor.begin_variation() {
                continue; // Skip this variation
            }

            // Visit first move of this variation
            visitor.visit_move(
                self.board(initial_position),
                variation_node.prev_move().unwrap(),
            );

            // Recursively visiting variation node
            variation_node.accept(initial_position, visitor);

            visitor.end_variation();
        }

        // Visit mainline recursively last
        main_node.accept(initial_position, visitor);
    }
}
