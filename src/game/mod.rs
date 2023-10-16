mod node;
pub use node::Node;
mod header;
pub use header::Header;

use std::collections::HashMap;

use crate::pgn::writer;
use crate::Chess;

/// A chess game with possible variations.
///
/// It contains a simple BTree, plus header and initial position.
#[derive(Debug, Clone, Default)]
pub struct Game {
    pub(crate) header: Header,
    pub(crate) opt_headers: HashMap<String, String>,

    pub(crate) initial_position: Chess,

    pub(crate) root: Node,
}

impl Game {
    /// Returns the root node.
    /// (the node before any moves)
    ///
    /// # Examples
    ///
    /// ```
    /// let game = sacrifice::read_pgn("1. e4 e5");
    /// let root_node = game.root();
    /// ```
    pub fn root(&self) -> Node {
        self.root.clone()
    }

    pub fn initial_position(&self) -> Chess {
        self.initial_position.clone()
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut visitor = if let Some(max_width) = f.width() {
            writer::PgnWriter::with_max_width(max_width as u32)
        } else {
            writer::PgnWriter::new()
        };

        use writer::FullAcceptor;
        let line_vec = self.accept(&mut visitor);

        // This always ends with \n.
        for line in line_vec {
            writeln!(f, "{}", line)?;
        }

        Ok(())
    }
}
