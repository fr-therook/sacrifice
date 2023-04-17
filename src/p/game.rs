use crate::prelude::*;
use crate::node::NodeAcceptorExt;

use std::collections::{HashMap};

use crate::header::Header;
use crate::{reader, writer, reader::NodeBuilder, reader::VisitedGame};
use crate::{Chess, Move, Position};

use super::NodeImpl;

#[derive(Default, Clone)]
pub struct NodeBuilderImpl {}

impl<N: Node> NodeBuilder<N> for NodeBuilderImpl {
    fn new_node(&mut self) -> N {
        N::default()
    }

    fn new_node_from(&mut self, parent: N, m: Move) -> N {
        N::from_node(parent, m)
    }

    fn remove_node(&mut self, _node: N) {}
}

/// A chess game with possible variations.
///
/// It contains a simple BTree, plus header and initial position.
pub struct GameImpl {
    header: Header,
    opt_headers: HashMap<String, String>,

    initial_position: Chess,

    root: NodeImpl,
    node_builder: NodeBuilderImpl,
}

impl Default for GameImpl {
    /// Initialize a chess game with no moves yet.
    fn default() -> Self {
        let header = Header::default();
        let opt_headers = HashMap::new();
        let initial_position = Chess::default();

        let root = NodeImpl::default();
        let node_builder = NodeBuilderImpl::default();

        Self {
            header,
            opt_headers,
            initial_position,

            root,
            node_builder,
        }
    }
}

impl GameImpl {
    fn from_visited_game(visited_game: VisitedGame<NodeImpl, NodeBuilderImpl>) -> Self {
        Self {
            header: visited_game.header,
            opt_headers: visited_game.opt_headers,
            initial_position: visited_game.initial_position,
            root: visited_game.root,
            node_builder: visited_game.node_builder,
        }
    }
}

impl Game<NodeImpl> for GameImpl {
    fn from_pgn(pgn: &str) -> Self {
        let visited_game: VisitedGame<NodeImpl, NodeBuilderImpl> = reader::read_pgn(pgn).unwrap();
        Self::from_visited_game(visited_game)
    }

    fn root(&self) -> NodeImpl {
        self.root.clone()
    }

    fn initial_position(&self) -> Chess {
        self.initial_position.clone()
    }

    fn add_node(&mut self, mut parent: NodeImpl, m: Move) -> Option<NodeImpl> {
        // Check if the move is legal
        let cur_position = parent.board(&self.initial_position);
        if !cur_position.is_legal(&m) {
            return None; // Not legal move
        }

        let new_node = self.node_builder.new_node_from(parent.clone(), m);
        let mut variation_vec = parent.variation_vec();
        variation_vec.push(new_node.clone());
        parent.set_variation_vec(variation_vec);
        Some(new_node)
    }

    fn remove_node(&mut self, node: NodeImpl) -> Option<NodeImpl> {
        let mut parent = if let Some(val) = node.parent() {
            val
        } else {
            println!(
                "node has no parent - attempting to delete root node?"
            );
            return None;
        };

        // Remove this node and its children from node map
        // {
        //     let mut node_queue: VecDeque<NodeImpl> = VecDeque::from([node.clone()]);
        //     while !node_queue.is_empty() {
        //         let node = node_queue.pop_front().unwrap();
        //         self.node_map.remove(&node.id());
        //         for variation_node in node.variations() {
        //             node_queue.push_back(variation_node);
        //         }
        //     }
        // }

        // Remove this node from its parent
        if parent.remove_variation(node.clone()) {
            return Some(node);
        }

        // How did we get here?
        println!(
            "node has parent, yet is not its child",
        );

        None
    }
}

impl std::fmt::Display for GameImpl {
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

impl GameImpl {
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
