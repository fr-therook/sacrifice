use crate::prelude::*;
use crate::{Move, Chess};

/// A chess game with possible variations.
pub trait Game<N: Node>: Default + std::fmt::Display {
    fn from_pgn(pgn: &str) -> Self;

    /// Returns the root node.
    /// (the node before any moves)
    ///
    /// # Examples
    ///
    /// ```
    /// use sacrifice::prelude::*;
    ///
    /// let game = sacrifice::read_pgn("1. e4 e5");
    /// let root_node = game.root();
    /// ```
    fn root(&self) -> N;

    fn initial_position(&self) -> Chess;

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
    /// use sacrifice::prelude::*;
    ///
    /// let mut game = sacrifice::read_pgn("1. d4");
    /// let mainline_node_1 = game.root().mainline().unwrap();
    /// let illegal_move = sacrifice::Move::Normal {
    ///    role: sacrifice::Role::Queen,
    ///    from: sacrifice::Square::D8,
    ///    to: sacrifice::Square::H4,
    ///    capture: None,
    ///    promotion: None,
    /// };
    /// assert!(game.add_node(mainline_node_1.clone(), illegal_move).is_none());
    /// let legal_move = sacrifice::Move::Normal {
    ///    role: sacrifice::Role::Pawn,
    ///    from: sacrifice::Square::E7,
    ///    to: sacrifice::Square::E5,
    ///    capture: None,
    ///    promotion: None,
    /// };
    /// let new_node = game.add_node(mainline_node_1.clone(), legal_move);
    /// assert!(new_node.is_some());
    /// assert_eq!(
    ///   mainline_node_1.mainline().unwrap(),
    ///   new_node.unwrap()
    /// );
    /// ```
    fn add_node(&mut self, parent: N, m: Move) -> Option<N>;

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
    /// use sacrifice::prelude::*;
    ///
    /// let mut game = sacrifice::read_pgn("1. d4");
    /// let mainline_node_1 = game.root().mainline().unwrap();
    /// assert!(game.remove_node(mainline_node_1).is_some()); // No child nodes left
    /// assert!(game.root().mainline().is_none());
    /// ```
    fn remove_node(&mut self, node: N) -> Option<N>;
}
