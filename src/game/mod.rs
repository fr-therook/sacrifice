mod p;

#[cfg(test)]
mod tests;

use crate::Move;
use uuid::Uuid;

// A chess game.
pub struct Game {
    inner: p::GameTree,

    cur_node: p::Node,
}

// Constructors
impl Default for Game {
    // A chess game with default position and no moves.
    fn default() -> Self {
        Self::from_p_game(p::GameTree::default())
    }
}

impl Game {
    // Load a chess game from PGN.
    pub fn from_pgn(pgn_str: &str) -> Self {
        Self::from_p_game(p::GameTree::from_pgn(pgn_str))
    }

    fn from_p_game(inner: p::GameTree) -> Self {
        let cur_node = inner.root.clone();

        Self { inner, cur_node }
    }
}

impl std::fmt::Display for Game {
    // Outputs PGN of this game.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

// Proxy methods of p::GameTree
impl Game {
    // Returns the game's root node.
    pub fn root(&self) -> Uuid {
        self.inner.root()
    }

    // Check if a uuid corresponds to a node in the game tree.
    // Returns the same uuid if exists.
    pub fn exists(&self, node_id: Uuid) -> Option<Uuid> {
        self.inner.exists(node_id)
    }

    // Returns the parent node
    pub fn parent(&self, node_id: Uuid) -> Option<Uuid> {
        self.inner.parent(node_id)
    }

    // Returns the following variations of a node.
    pub fn children(&self, node_id: Uuid) -> Vec<Uuid> {
        self.inner.children(node_id)
    }

    // Get the move before the given node.
    pub fn prev_move(&self, node_id: Uuid) -> Option<Move> {
        self.inner.prev_move(node_id)
    }
}
