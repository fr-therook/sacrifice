mod p;

#[cfg(test)]
mod tests;

use uuid::Uuid;

pub struct Game {
    inner: p::GameTree,

    cur_node: p::Node,
}

impl Game {
    pub fn from_pgn(pgn_str: &str) -> Self {
        let inner = p::GameTree::from_pgn(pgn_str);

        Self::from_p_game(inner)
    }
}

impl Game {
    fn from_p_game(inner: p::GameTree) -> Self {
        let cur_node = inner.root.clone();

        Self {
            inner,
            cur_node,
        }
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

