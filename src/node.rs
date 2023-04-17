use crate::{Chess, Move, Position};
use std::collections::HashSet;

pub trait Node: Sized + Clone + PartialEq + Default {
    // Constructors
    fn from_node(parent: Self, m: Move) -> Self;

    // Required properties

    /// Returns the parent node of the given node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - id of the given node
    ///
    /// # Examples
    ///
    /// ```
    /// use sacrifice::prelude::*;
    ///
    /// let game = sacrifice::read_pgn("1. e4 e5");
    /// let root = game.root();
    /// assert!(root.parent().is_none()); // root node needs no parent
    /// let mainline_node_1 = root.mainline().unwrap(); // 1. e4 node
    /// assert_eq!(
    ///   mainline_node_1.parent(),
    ///   Some(root)
    /// );
    /// ```
    fn parent(&self) -> Option<Self>;

    /// Returns the move that leads to the given node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - id of the given node
    ///
    /// # Examples
    ///
    /// ```
    /// use sacrifice::prelude::*;
    ///
    /// let game = sacrifice::read_pgn("1. e4 e5");
    /// let mainline_node_1 = game.root().mainline().unwrap(); // 1. e4 node
    /// assert_eq!(
    ///   mainline_node_1.prev_move().unwrap().to(),
    ///   sacrifice::Square::E4
    /// );
    /// ```
    fn prev_move(&self) -> Option<Move>;

    fn variation_vec(&self) -> Vec<Self>;
    fn set_variation_vec(&mut self, new_variation_vec: Vec<Self>) -> Vec<Self>;

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
    /// use sacrifice::prelude::*;
    ///
    /// let ok_str = "Ok";
    /// let pgn_str = format!("1. e4 ({{ {} }} 1. d4) 1... e5", ok_str);
    /// let game = sacrifice::read_pgn(pgn_str.as_str());
    /// let variation_node_1_0 = game.root().other_variations()[0].clone(); // {Ok} 1. d4
    /// assert_eq!(
    ///   variation_node_1_0.starting_comment(),
    ///   Some(ok_str.to_string())
    /// );
    /// ```
    fn starting_comment(&self) -> Option<String>;

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
    /// use sacrifice::prelude::*;
    ///
    /// let game = sacrifice::read_pgn("1. e4 (1. d4) 1... e5");
    /// let mut variation_node_1_0 = game.root().other_variations()[0].clone(); // {Ok} 1. d4
    /// assert!(variation_node_1_0.starting_comment().is_none()); // 1... e5
    /// variation_node_1_0.set_starting_comment(Some("Ok".to_string()));
    /// assert_eq!(
    ///   variation_node_1_0.starting_comment(),
    ///   Some("Ok".to_string())
    /// );
    /// ```
    fn set_starting_comment(&mut self, new_comment: Option<String>) -> Option<String>;

    /// Returns the NAGs of the given node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - id of the given node
    ///
    /// # Examples
    ///
    /// ```
    /// use sacrifice::prelude::*;
    ///
    /// let game = sacrifice::read_pgn("1. e4?? c5!");
    /// let mainline_node_1 = game.root().mainline().unwrap(); // [1. e4??]
    /// assert!(mainline_node_1.nags().unwrap().contains(&4)); // ?? -> $4
    /// let mainline_node_2 = mainline_node_1.mainline().unwrap(); // [1... c5!]
    /// assert!(mainline_node_2.nags().unwrap().contains(&1)); // ! -> $1
    /// ```
    fn nags(&self) -> Option<HashSet<u8>>;
    fn set_nags(&mut self, new_nags: HashSet<u8>) -> Option<HashSet<u8>>;

    /// Returns the comment on a given node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - id of the given node
    ///
    /// # Examples
    ///
    /// ```
    /// use sacrifice::prelude::*;
    ///
    /// let e4_comment_str = "this blunders into the Sicilian Defense";
    /// let pgn_str = format!("1. e4 {{ {} }}  1... c5", e4_comment_str);
    /// let game = sacrifice::read_pgn(pgn_str.as_str());
    /// let mainline_node_1 = game.root().mainline().unwrap(); // 1. e4
    /// assert_eq!(
    ///   mainline_node_1.comment(),
    ///   Some(e4_comment_str.to_string())
    /// );
    /// ```
    fn comment(&self) -> Option<String>;

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
    /// use sacrifice::prelude::*;
    ///
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
    fn set_comment(&self, new_comment: Option<String>) -> Option<String>;
}

pub trait NodePropertiesExt: Node {
    fn push_nag(&mut self, nag: u8);
    fn clear_nags(&mut self);

    /// Returns the mainline variation of the given node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - id of the given node
    ///
    /// # Examples
    ///
    /// ```
    /// use sacrifice::prelude::*;
    ///
    /// let game = sacrifice::read_pgn("1. e4 e5");
    /// let mainline_node_1 = game.root().mainline(); // 1. e4 node
    /// assert!(mainline_node_1.is_some()); // It exists
    /// ```
    fn mainline(&self) -> Option<Self>;

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
    /// use sacrifice::prelude::*;
    ///
    /// let game = sacrifice::read_pgn("1. e4 (1. d4) 1... e5");
    /// let variation_nodes_1 = game.root().other_variations(); // [1. d4]
    /// assert!(!variation_nodes_1.is_empty()); // It exists
    /// ```
    fn other_variations(&self) -> Vec<Self>;

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
    /// use sacrifice::prelude::*;
    ///
    /// let game = sacrifice::read_pgn("1. e4 (1. d4) 1... e5");
    /// let root = game.root();
    /// let e4_node = root.mainline().expect("e4 node should exist");
    /// let e4_siblings = e4_node.siblings();
    /// assert!(!e4_siblings.is_empty()); // It exists
    /// ```
    fn siblings(&self) -> Vec<Self>;

    fn new_variation(&mut self, m: Move) -> Self;
    fn remove_variation(&mut self, node: Self) -> bool;

    /// Promotes a node to the mainline variation of its parent.
    ///
    /// # Arguments
    ///
    /// * `node_id` - id of the node to promote
    ///
    /// # Examples
    ///
    /// ```
    /// use sacrifice::prelude::*;
    ///
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
    fn promote_variation(&mut self, node: Self) -> bool;
}

impl<N: Node> NodePropertiesExt for N {
    fn push_nag(&mut self, nag: u8) {
        if let Some(mut nags) = self.nags() {
            nags.insert(nag);
            self.set_nags(nags);
        }
    }
    fn clear_nags(&mut self) {
        self.set_nags(HashSet::new());
    }

    fn mainline(&self) -> Option<Self> {
        self.variation_vec().get(0).cloned()
    }
    fn other_variations(&self) -> Vec<Self> {
        let mut variations = self.variation_vec();
        if variations.is_empty() {
            return Vec::new();
        }

        variations.remove(0);
        variations
    }
    fn siblings(&self) -> Vec<Self> {
        let parent = if let Some(val) = self.parent() {
            val
        } else {
            return Vec::new();
        };

        let mut variation_vec = parent.variation_vec();
        variation_vec.retain(|val| val != self);
        variation_vec
    }

    fn new_variation(&mut self, m: Move) -> Self {
        let next_node = Self::from_node(self.clone(), m);
        let mut variation_vec = self.variation_vec();
        variation_vec.push(next_node.clone());
        self.set_variation_vec(variation_vec);
        next_node
    }
    fn remove_variation(&mut self, node: Self) -> bool {
        let mut variation_vec = self.variation_vec();
        let variations_size = variation_vec.len();
        variation_vec.retain(|v| v != &node);
        let removed = variation_vec.len() < variations_size;
        self.set_variation_vec(variation_vec);
        removed
    }
    fn promote_variation(&mut self, node: Self) -> bool {
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

pub trait NodeTreeTraversalExt: Node {
    fn root(&self) -> Self;
    fn depth(&self) -> u32;

    /// Returns the array of moves that leads to the given node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - id of the given node
    ///
    /// # Examples
    ///
    /// ```
    /// use sacrifice::prelude::*;
    ///
    /// let game = sacrifice::read_pgn("1. e4 c5");
    /// let mainline_node_1 = game.root().mainline().unwrap(); // 1. e4
    /// let mainline_node_2 = mainline_node_1.mainline().unwrap(); // 1... c5
    /// let moves = mainline_node_2.moves(); // 1. e4 c5
    /// assert_eq!(moves[0].to(), sacrifice::Square::E4);
    /// assert_eq!(moves[1].to(), sacrifice::Square::C5);
    /// ```
    fn moves(&self) -> Vec<Move>;

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
    /// use sacrifice::prelude::*;
    ///
    /// let game = sacrifice::read_pgn("1. e4 c5");
    /// let mainline_node_1 = game.root().mainline().unwrap(); // 1. e4
    /// let mainline_node_2 = mainline_node_1.mainline().unwrap(); // 1... c5
    /// let fen: sacrifice::Fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2".parse().unwrap();
    /// let actual_position: sacrifice::Chess = fen.clone().into_position(sacrifice::CastlingMode::Standard).unwrap();
    /// assert_eq!(
    ///   mainline_node_2.board(&game.initial_position()),
    ///   actual_position
    /// )
    /// ```
    fn board(&self, initial_position: &Chess) -> Chess;

    /// Returns the board position before a given node.
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
    /// use sacrifice::prelude::*;
    ///
    /// let game = sacrifice::read_pgn("1. e4 c5");
    /// let mainline_node_1 = game.root().mainline().unwrap(); // 1. e4
    /// let mainline_node_2 = mainline_node_1.mainline().unwrap(); // 1... c5
    /// let fen: sacrifice::Fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1".parse().unwrap();
    /// let actual_position: sacrifice::Chess = fen.clone().into_position(sacrifice::CastlingMode::Standard).unwrap();
    /// assert_eq!(
    ///   mainline_node_2.board_before(&game.initial_position()),
    ///   actual_position
    /// )
    /// ```
    fn board_before(&self, initial_position: &Chess) -> Chess;
}

impl<N: Node> NodeTreeTraversalExt for N {
    fn root(&self) -> Self {
        let mut node = self.clone();
        while let Some(parent) = node.parent() {
            node = parent;
        }
        node
    }

    fn depth(&self) -> u32 {
        let mut result: u32 = 0;

        let mut node: Self = self.clone();
        while let Some(parent) = node.parent() {
            result += 1;
            node = parent;
        }
        result
    }

    fn moves(&self) -> Vec<Move> {
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

    fn board(&self, initial_position: &Chess) -> Chess {
        let mut board = initial_position.clone();

        let move_vec = self.moves();
        for _move in move_vec {
            board.play_unchecked(&_move);
        }

        board
    }

    fn board_before(&self, initial_position: &Chess) -> Chess {
        let mut board = initial_position.clone();

        let mut move_vec = self.moves();
        move_vec.pop(); // Remove latest move
        for _move in move_vec {
            board.play_unchecked(&_move);
        }

        board
    }
}
