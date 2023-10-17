use crate::{Chess, Move, Position};

use std::collections::HashSet;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct ParentState {
    /// This node's parent
    node: Node,
    /// The move that leads to this position
    move_next: Move,

    /// Comment about the start of a variation
    starting_comment: Option<String>,
    /// this node's nag attributes
    nag_set: HashSet<u8>,
}

#[derive(Debug, Clone, Default)]
pub struct NodeImpl {
    parent: Option<ParentState>,

    /// Position of current node
    position: Chess,

    /// Children nodes (variations), including mainline
    variation_vec: Vec<Node>,
    /// Comment against this node
    comment: Option<String>,
}

/// A node in the game tree.
#[derive(Debug, Clone, Default)]
pub struct Node(Rc<RefCell<NodeImpl>>);

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

// Constructors
impl Node {
    pub(crate) fn from_position(position: Chess) -> Self {
        let ret = NodeImpl {
            position,
            ..NodeImpl::default()
        };
        let ret = Rc::new(RefCell::new(ret));

        Self(ret)
    }

    pub(crate) fn from_node(node: Self, move_next: Move) -> Option<Self> {
        let position_next = if let Ok(inner) = node.position()
            .play(&move_next) {
            inner
        } else { return None; };

        let ret = NodeImpl {
            parent: Some(ParentState {
                node,
                move_next,
                starting_comment: None,
                nag_set: HashSet::new(),
            }),

            position: position_next,

            variation_vec: Vec::new(),
            comment: None,
        };
        let ret = Rc::new(RefCell::new(ret));

        Some(Self(ret))
    }
}

impl Node {
    /// Returns the parent node of the given node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - id of the given node
    ///
    /// # Examples
    ///
    /// ```
    /// let game = sacrifice::read_pgn("1. e4 e5");
    /// let root = game.root();
    /// assert!(root.parent().is_none()); // root node needs no parent
    /// let mainline_node_1 = root.mainline().unwrap(); // 1. e4 node
    /// assert_eq!(
    ///   mainline_node_1.parent(),
    ///   Some(root)
    /// );
    /// ```
    pub fn parent(&self) -> Option<Self> {
        if let Some(ref parent) = self.0.borrow().parent {
            return Some(parent.node.clone());
        }

        None
    }

    /// Returns the move that leads to the given node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - id of the given node
    ///
    /// # Examples
    ///
    /// ```
    /// let game = sacrifice::read_pgn("1. e4 e5");
    /// let mainline_node_1 = game.root().mainline().unwrap(); // 1. e4 node
    /// assert_eq!(
    ///   mainline_node_1.prev_move().unwrap().to(),
    ///   sacrifice::Square::E4
    /// );
    /// ```
    pub fn prev_move(&self) -> Option<Move> {
        if let Some(ref parent) = self.0.borrow().parent {
            return Some(parent.move_next.clone());
        }

        None
    }

    pub fn variation_vec(&self) -> Vec<Self> {
        self.0.borrow().variation_vec.clone()
    }

    pub fn set_variation_vec(&mut self, new_variation_vec: Vec<Self>) -> Vec<Self> {
        std::mem::replace(
            &mut self.0.borrow_mut().variation_vec,
            new_variation_vec,
        )
    }

    /// Returns the starting comment (comment that starts a variation)
    /// of the given node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - id of the given node
    ///
    /// # Examples
    ///
    /// ```
    /// let ok_str = "Ok";
    /// let pgn_str = format!("1. e4 ({{ {} }} 1. d4) 1... e5", ok_str);
    /// let game = sacrifice::read_pgn(pgn_str.as_str());
    /// let variation_node_1_0 = game.root().other_variations()[0].clone(); // {Ok} 1. d4
    /// assert_eq!(
    ///   variation_node_1_0.starting_comment(),
    ///   Some(ok_str.to_string())
    /// );
    /// ```
    pub fn starting_comment(&self) -> Option<String> {
        if let Some(ref parent) = self.0.borrow().parent {
            return parent.starting_comment.clone();
        }

        None
    }

    /// Sets the starting comment of the given node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - id of the given node
    /// * `new_comment` - value of the new starting comment
    ///
    /// # Examples
    ///
    /// ```
    /// let game = sacrifice::read_pgn("1. e4 (1. d4) 1... e5");
    /// let mut variation_node_1_0 = game.root().other_variations()[0].clone(); // {Ok} 1. d4
    /// assert!(variation_node_1_0.starting_comment().is_none()); // 1... e5
    /// variation_node_1_0.set_starting_comment(Some("Ok".to_string()));
    /// assert_eq!(
    ///   variation_node_1_0.starting_comment(),
    ///   Some("Ok".to_string())
    /// );
    /// ```
    pub fn set_starting_comment(&mut self, comment_next: Option<String>) -> Option<String> {
        if let Some(ref mut parent) = self.0.borrow_mut().parent {
            return std::mem::replace(&mut parent.starting_comment, comment_next);
        }

        None
    }

    /// Returns the NAGs of the given node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - id of the given node
    ///
    /// # Examples
    ///
    /// ```
    /// let game = sacrifice::read_pgn("1. e4?? c5!");
    /// let mainline_node_1 = game.root().mainline().unwrap(); // [1. e4??]
    /// assert!(mainline_node_1.nags().unwrap().contains(&4)); // ?? -> $4
    /// let mainline_node_2 = mainline_node_1.mainline().unwrap(); // [1... c5!]
    /// assert!(mainline_node_2.nags().unwrap().contains(&1)); // ! -> $1
    /// ```
    pub fn nags(&self) -> Option<HashSet<u8>> {
        if let Some(ref parent) = self.0.borrow().parent {
            return Some(parent.nag_set.clone());
        }

        None
    }

    pub fn set_nags(&mut self, nags_next: HashSet<u8>) -> Option<HashSet<u8>> {
        if let Some(ref mut parent) = self.0.borrow_mut().parent {
            return Some(std::mem::replace(&mut parent.nag_set, nags_next));
        }

        None
    }

    /// Returns the comment on a given node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - id of the given node
    ///
    /// # Examples
    ///
    /// ```
    /// let e4_comment_str = "this blunders into the Sicilian Defense";
    /// let pgn_str = format!("1. e4 {{ {} }}  1... c5", e4_comment_str);
    /// let game = sacrifice::read_pgn(pgn_str.as_str());
    /// let mainline_node_1 = game.root().mainline().unwrap(); // 1. e4
    /// assert_eq!(
    ///   mainline_node_1.comment(),
    ///   Some(e4_comment_str.to_string())
    /// );
    /// ```
    pub fn comment(&self) -> Option<String> {
        self.0.borrow().comment.clone()
    }

    /// Sets the comment on a given node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - id of the given node
    /// * `new_comment` - new value of the comment
    ///
    /// # Examples
    ///
    /// ```
    /// let e4_comment_str = "this blunders into the Sicilian Defense";
    /// let pgn_str = format!("1. e4 {{ {} }}  1... c5", e4_comment_str);
    /// let game = sacrifice::read_pgn(pgn_str.as_str());
    /// let mainline_node_1 = game.root().mainline().unwrap(); // 1. e4
    /// assert_eq!(
    ///   mainline_node_1.comment(),
    ///   Some(e4_comment_str.to_string())
    /// );
    /// let e4_comment_alt_str = "best by test";
    /// mainline_node_1.set_comment(Some(e4_comment_alt_str.to_string()));
    /// assert_eq!(
    ///   mainline_node_1.comment(),
    ///   Some(e4_comment_alt_str.to_string()) // it just is
    /// );
    /// ```
    pub fn set_comment(&self, comment_next: Option<String>) -> Option<String> {
        std::mem::replace(&mut self.0.borrow_mut().comment, comment_next)
    }
}

impl Node {
    pub fn push_nag(&mut self, nag: u8) {
        if let Some(mut nags) = self.nags() {
            nags.insert(nag);
            self.set_nags(nags);
        }
    }

    pub fn clear_nags(&mut self) {
        self.set_nags(HashSet::new());
    }

    /// Returns the mainline variation of the given node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - id of the given node
    ///
    /// # Examples
    ///
    /// ```
    /// let game = sacrifice::read_pgn("1. e4 e5");
    /// let mainline_node_1 = game.root().mainline(); // 1. e4 node
    /// assert!(mainline_node_1.is_some()); // It exists
    /// ```
    pub fn mainline(&self) -> Option<Self> {
        self.variation_vec().get(0).cloned()
    }

    /// Returns variations (excluding mainline) of the given node.
    ///
    /// Returns an empty array if no other variation exists.
    ///
    /// # Arguments
    ///
    /// * `node_id` - id of the given node
    ///
    /// # Examples
    ///
    /// ```
    /// let game = sacrifice::read_pgn("1. e4 (1. d4) 1... e5");
    /// let variation_nodes_1 = game.root().other_variations(); // [1. d4]
    /// assert!(!variation_nodes_1.is_empty()); // It exists
    /// ```
    pub fn other_variations(&self) -> Vec<Self> {
        let mut variations = self.variation_vec();
        if variations.is_empty() {
            return Vec::new();
        }

        variations.remove(0);
        variations
    }

    /// Returns siblings (other variations of the parent node) of the given node.
    ///
    /// Returns an empty array if no siblings exists.
    ///
    /// # Arguments
    ///
    /// * `node_id` - id of the given node
    ///
    /// # Examples
    ///
    /// ```
    /// let game = sacrifice::read_pgn("1. e4 (1. d4) 1... e5");
    /// let root = game.root();
    /// let e4_node = root.mainline().expect("e4 node should exist");
    /// let e4_siblings = e4_node.siblings();
    /// assert!(!e4_siblings.is_empty()); // It exists
    /// ```
    pub fn siblings(&self) -> Vec<Self> {
        let parent = if let Some(val) = self.parent() {
            val
        } else {
            return Vec::new();
        };

        let mut variation_vec = parent.variation_vec();
        variation_vec.retain(|val| val != self);
        variation_vec
    }

    /// Add a move to a given node in the game tree.
    ///
    /// Returns `None` if the move is illegal, or if given node is not found in the tree.
    ///
    /// # Arguments
    ///
    /// * `parent_id` - designated parent node of the newly created node
    /// * `m` - a (possibly illegal) chess move
    ///
    /// # Examples
    ///
    /// ```
    /// let mut game = sacrifice::read_pgn("1. d4");
    /// let mut mainline_node_1 = game.root().mainline().unwrap();
    /// let illegal_move = sacrifice::Move::Normal {
    ///    role: sacrifice::Role::Queen,
    ///    from: sacrifice::Square::D8,
    ///    to: sacrifice::Square::H4,
    ///    capture: None,
    ///    promotion: None,
    /// };
    /// assert!(mainline_node_1.new_variation(illegal_move).is_none());
    /// let legal_move = sacrifice::Move::Normal {
    ///    role: sacrifice::Role::Pawn,
    ///    from: sacrifice::Square::E7,
    ///    to: sacrifice::Square::E5,
    ///    capture: None,
    ///    promotion: None,
    /// };
    /// let new_node = mainline_node_1.new_variation(legal_move);
    /// assert!(new_node.is_some());
    /// assert_eq!(
    ///   mainline_node_1.mainline().unwrap(),
    ///   new_node.unwrap()
    /// );
    /// ```
    pub fn new_variation(&mut self, move_next: Move) -> Option<Self> {
        let node_next = Self::from_node(self.clone(), move_next)?;
        let mut variation_vec = self.variation_vec();
        variation_vec.push(node_next.clone());
        self.set_variation_vec(variation_vec);
        Some(node_next)
    }

    pub fn remove_variation(&mut self, node: Self) -> bool {
        let mut variation_vec = self.variation_vec();
        let variations_size = variation_vec.len();
        variation_vec.retain(|v| v != &node);
        let removed = variation_vec.len() < variations_size;
        self.set_variation_vec(variation_vec);
        removed
    }

    /// Promotes a node to the mainline variation of its parent.
    ///
    /// # Arguments
    ///
    /// * `node_id` - id of the node to promote
    ///
    /// # Examples
    ///
    /// ```
    /// let mut game = sacrifice::read_pgn("1. d4 (1. e4) 1... d5");
    /// let variation_node_1_0 = game.root().other_variations()[0].clone(); // (1. e4)
    /// assert!(
    ///   game.root().promote_variation(variation_node_1_0.clone()), // promote 1. e4 to mainline
    /// );
    /// assert_eq!(
    ///   game.root().mainline(),
    ///   Some(variation_node_1_0.clone())
    /// );
    /// ```
    pub fn promote_variation(&mut self, node: Self) -> bool {
        let mut variation_vec = self.variation_vec();
        let variations_size = variation_vec.len();
        variation_vec.retain(|v| v != &node);
        if variation_vec.len() == variations_size {
            // not its own children
            println!("attempting to promote disconnected node in parent node");
            return false;
        }
        // TODO: Use node directly from variation_vec
        variation_vec.insert(0, node);
        self.set_variation_vec(variation_vec);

        true
    }
}

impl Node {
    pub fn root(&self) -> Self {
        let mut node = self.clone();
        while let Some(parent) = node.parent() {
            node = parent;
        }
        node
    }

    pub fn depth(&self) -> u32 {
        let mut result: u32 = 0;

        let mut node: Self = self.clone();
        while let Some(parent) = node.parent() {
            result += 1;
            node = parent;
        }
        result
    }

    /// Returns the array of moves that leads to the given node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - id of the given node
    ///
    /// # Examples
    ///
    /// ```
    /// let game = sacrifice::read_pgn("1. e4 c5");
    /// let mainline_node_1 = game.root().mainline().unwrap(); // 1. e4
    /// let mainline_node_2 = mainline_node_1.mainline().unwrap(); // 1... c5
    /// let moves = mainline_node_2.moves(); // 1. e4 c5
    /// assert_eq!(moves[0].to(), sacrifice::Square::E4);
    /// assert_eq!(moves[1].to(), sacrifice::Square::C5);
    /// ```
    pub fn moves(&self) -> Vec<Move> {
        let mut move_vec: Vec<Move> = Vec::new();

        let mut node: Self = self.clone();
        while let Some(parent) = node.parent() {
            let prev_move = node.prev_move().expect("node has no prev_move");
            move_vec.push(prev_move);
            node = parent;
        }
        move_vec.reverse();

        move_vec
    }

    /// Returns the board position at a given node.
    ///
    /// Returns `None` if given node cannot be found in the tree.
    ///
    /// # Arguments
    ///
    /// * `node_id` - id of the given node
    ///
    /// # Examples
    ///
    /// ```
    /// let game = sacrifice::read_pgn("1. e4 c5");
    /// let mainline_node_1 = game.root().mainline().unwrap(); // 1. e4
    /// let mainline_node_2 = mainline_node_1.mainline().unwrap(); // 1... c5
    /// let fen: sacrifice::Fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2".parse().unwrap();
    /// let actual_position: sacrifice::Chess = fen.clone().into_position(sacrifice::CastlingMode::Standard).unwrap();
    /// assert_eq!(
    ///   mainline_node_2.position(),
    ///   actual_position
    /// )
    /// ```
    pub fn position(&self) -> Chess {
        self.0.borrow().position.clone()
    }

    /// Remove all occurrences of the given node from the game tree.
    ///
    /// Returns the given node's id if successful.
    ///
    /// # Arguments
    ///
    /// * `node_id` - id of the given node
    ///
    /// # Examples
    ///
    /// ```
    /// let mut game = sacrifice::read_pgn("1. d4");
    /// let mut mainline_node_1 = game.root().mainline().unwrap();
    /// assert!(mainline_node_1.remove_node().is_some()); // No child nodes left
    /// assert!(game.root().mainline().is_none());
    /// ```
    pub fn remove_node(&mut self) -> Option<Node> {
        let mut parent = if let Some(val) = self.parent() {
            val
        } else {
            println!("node has no parent - attempting to delete root node?");
            return None;
        };

        // Remove this node from its parent
        if parent.remove_variation(self.clone()) {
            return Some(self.clone());
        }

        // How did we get here?
        println!("node has parent, yet is not its child");

        None
    }
}
