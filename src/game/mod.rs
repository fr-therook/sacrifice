mod header;
mod node;

mod reader;
mod writer;

use node::Node;

use header::Header;
use reader::read_pgn;

use shakmaty::Position;
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

use crate::{Chess, Move};

/// This structure represents a chess game with possible variations.
/// A simple BTree data structure, plus header and initial position.
/// It also holds a hashmap for quick node lookup.
///
/// # Examples
///
/// ```
/// let mut game = sacrifice::Game::from_pgn("1. e4 c5");
/// let open_sicilian = sacrifice::Move::Normal {
///    role: sacrifice::Role::Knight,
///    from: sacrifice::Square::G1,
///    capture: None,
///    to: sacrifice::Square::F3,
///    promotion: None,
/// };
/// // Play the Open Sicilian
/// assert_ne!(game.add_node(game.root(), open_sicilian), None);
/// println!("{}", game); // prints the PGN of the default position
/// ```
pub struct Game {
    header: Header,
    initial_position: Chess,
    opt_headers: HashMap<String, String>,

    root: Node,

    node_map: HashMap<Uuid, Node>,
}

impl Default for Game {
    /// Initialize a chess game with no moves yet.
    ///
    /// # Examples
    ///
    /// ```
    /// let game = sacrifice::Game::default();
    /// ```
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
    /// Parse one chess game from PGN string.
    ///
    /// # Arguments
    ///
    /// * `pgn_str` - the "import formatted" PGN string
    ///
    /// # Examples
    ///
    /// ```
    /// let game = sacrifice::Game::from_pgn("1. e4 e5");
    /// println!("{}", game); // Exports the game's PGN with default headers
    /// ```
    pub fn from_pgn(pgn_str: &str) -> Self {
        read_pgn(pgn_str).unwrap()
    }
}

// Accessing/manipulating a single node.
impl Game {
    fn try_node(&self, id: Uuid) -> Option<Node> {
        self.node_map.get(&id).cloned()
    }

    /// Returns the id of the root node.
    /// (the node before any moves)
    ///
    /// # Examples
    ///
    /// ```
    /// let game = sacrifice::Game::from_pgn("1. e4 e5");
    /// let root_node = game.root();
    /// assert_ne!(game.exists(root_node), None);
    /// ```
    pub fn root(&self) -> Uuid {
        self.root.id()
    }

    /// Check if given node id corresponds to a valid node in the game tree.
    /// Returns `Some(node_id)` if found.
    ///
    /// # Arguments
    ///
    /// * `node_id` - the node id to check
    ///
    /// # Examples
    ///
    /// ```
    /// let game = sacrifice::Game::default();
    /// let random_node = uuid::Uuid::new_v4();
    /// assert_eq!(game.exists(random_node), None);
    /// ```
    pub fn exists(&self, node_id: Uuid) -> Option<Uuid> {
        let node = self.try_node(node_id)?;
        assert_eq!(node.id(), node_id, "id-node hashmap outdated");
        Some(node.id())
    }

    /// Returns the parent node of the given node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - id of the given node
    ///
    /// # Examples
    ///
    /// ```
    /// let game = sacrifice::Game::from_pgn("1. e4 e5");
    /// let root_node = game.root();
    /// assert_eq!(game.parent(root_node), None); // root node needs no parent
    /// let mainline_node_1 = game.mainline(root_node).unwrap(); // 1. e4 node
    /// assert_eq!(game.parent(mainline_node_1).unwrap(), root_node);
    /// ```
    pub fn parent(&self, node_id: Uuid) -> Option<Uuid> {
        let node = self.try_node(node_id)?;
        node.parent().map(|val| val.id())
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
    /// let game = sacrifice::Game::from_pgn("1. e4 e5");
    /// let mainline_node_1 = game.mainline(game.root()).unwrap(); // 1. e4 node
    /// assert_eq!(
    ///   game.prev_move(mainline_node_1).unwrap().to(),
    ///   sacrifice::Square::E4
    /// );
    /// ```
    pub fn prev_move(&self, node_id: Uuid) -> Option<Move> {
        let node = self.try_node(node_id)?;
        node.prev_move()
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
    /// let game = sacrifice::Game::from_pgn("1. e4 e5");
    /// let mainline_node_1 = game.mainline(game.root()); // 1. e4 node
    /// assert_ne!(mainline_node_1, None); // It exists
    /// ```
    pub fn mainline(&self, node_id: Uuid) -> Option<Uuid> {
        let node = self.try_node(node_id)?;
        node.mainline().map(|v| v.id())
    }

    /// Returns variations (excluding mainline) of the given node.
    /// Returns an empty array if no other variation exists.
    ///
    /// # Arguments
    ///
    /// * `node_id` - id of the given node
    ///
    /// # Examples
    ///
    /// ```
    /// let game = sacrifice::Game::from_pgn("1. e4 (1. d4) 1... e5");
    /// let variation_nodes_1 = game.other_variations(game.root()); // [1. d4]
    /// assert!(!variation_nodes_1.is_empty()); // It exists
    /// ```
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
    /// let game = sacrifice::Game::from_pgn("1. e4 ({Ok} 1. d4) 1... e5");
    /// let variation_node_1_0 = game.other_variations(game.root())[0]; // {Ok} 1. d4
    /// assert_eq!(game.starting_comment(variation_node_1_0), Some("Ok".to_string()));
    /// ```
    pub fn starting_comment(&self, node_id: Uuid) -> Option<String> {
        let node = self.try_node(node_id)?;
        node.starting_comment()
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
    /// let game = sacrifice::Game::from_pgn("1. e4 (1. d4) 1... e5");
    /// let variation_node_1_0 = game.other_variations(game.root())[0]; // {Ok} 1. d4
    /// assert!(game.starting_comment(variation_node_1_0).is_none());
    /// game.set_starting_comment(variation_node_1_0, Some("Ok".to_string()));
    /// assert_eq!(game.starting_comment(variation_node_1_0), Some("Ok".to_string()));
    /// ```
    pub fn set_starting_comment(&self, node_id: Uuid, new_comment: Option<String>) {
        if let Some(node) = self.try_node(node_id) {
            node.set_starting_comment(new_comment)
        }
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
    /// let game = sacrifice::Game::from_pgn("1. e4?? c5!");
    /// let mainline_node_1 = game.mainline(game.root()).unwrap(); // [1. e4??]
    /// assert_eq!(game.nags(mainline_node_1), vec![4]); // ?? -> $4
    /// let mainline_node_2 = game.mainline(mainline_node_1).unwrap(); // [1... c5!]
    /// assert_eq!(game.nags(mainline_node_2), vec![1]); // ?? -> $4
    /// ```
    pub fn nags(&self, node_id: Uuid) -> Vec<u8> {
        if let Some(node) = self.try_node(node_id) {
            return node.nags();
        }

        Vec::new()
    }

    /// Overwrite the NAGs of the given node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - id of the given node
    /// * `nag_vec` - new NAG values
    ///
    /// # Examples
    ///
    /// ```
    /// let game = sacrifice::Game::from_pgn("1. e4?? c5!");
    /// let mainline_node_1 = game.mainline(game.root()).unwrap(); // [1. e4??]
    /// assert_eq!(game.nags(mainline_node_1), vec![4]); // ?? -> $4
    /// game.set_nags(mainline_node_1, vec![3]);
    /// assert_eq!(game.nags(mainline_node_1), vec![3]); // $3 -> !!
    /// ```
    pub fn set_nags(&self, node_id: Uuid, nag_vec: Vec<u8>) {
        if let Some(node) = self.try_node(node_id) {
            node.clear_nags();

            for nag in nag_vec {
                node.push_nag(nag);
            }
        }
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
    /// let game = sacrifice::Game::from_pgn("1. e4 { This blunders into the Sicilian Defense }  1... e5");
    /// let mainline_node_1 = game.mainline(game.root()).unwrap(); // 1. e4
    /// assert_eq!(
    ///   game.comment(mainline_node_1),
    ///   Some("This blunders into the Sicilian Defense".to_string())
    /// );
    /// ```
    pub fn comment(&self, node_id: Uuid) -> Option<String> {
        let node = self.try_node(node_id)?;
        node.comment()
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
    /// let game = sacrifice::Game::from_pgn("1. e4 { This blunders into the Sicilian Defense }  1... e5");
    /// let mainline_node_1 = game.mainline(game.root()).unwrap(); // 1. e4
    /// assert_eq!(
    ///   game.comment(mainline_node_1),
    ///   Some("This blunders into the Sicilian Defense".to_string())
    /// );
    /// game.set_comment(mainline_node_1, Some("best by test".to_string()));
    /// assert_eq!(
    ///   game.comment(mainline_node_1),
    ///   Some("best by test".to_string()) // it just is
    /// );
    /// ```
    pub fn set_comment(&self, node_id: Uuid, new_comment: Option<String>) {
        if let Some(node) = self.try_node(node_id) {
            node.set_comment(new_comment)
        }
    }

    /// Returns the board position at a given node.
    /// Returns `None` if given node cannot be found in the tree.
    ///
    /// # Arguments
    ///
    /// * `node_id` - id of the given node
    ///
    /// # Examples
    ///
    /// ```
    /// let game = sacrifice::Game::from_pgn("1. e4 c5");
    /// let mainline_node_1 = game.mainline(game.root()).unwrap(); // 1. e4
    /// let mainline_node_2 = game.mainline(mainline_node_1).unwrap(); // 1... c5
    /// let fen: sacrifice::Fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2".parse().unwrap();
    /// let actual_position: sacrifice::Chess = fen.clone().into_position(sacrifice::CastlingMode::Standard).unwrap();
    /// assert_eq!(game.board_at(mainline_node_2).unwrap(), actual_position)
    /// ```
    pub fn board_at(&self, node_id: Uuid) -> Option<Chess> {
        let node = self.try_node(node_id)?;
        Some(node.board(&self.initial_position))
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
    /// let game = sacrifice::Game::from_pgn("1. e4 c5");
    /// let mainline_node_1 = game.mainline(game.root()).unwrap(); // 1. e4
    /// let mainline_node_2 = game.mainline(mainline_node_1).unwrap(); // 1... c5
    /// let moves = game.moves_before(mainline_node_2); // 1. e4 c5
    /// assert_eq!(moves[0].to(), sacrifice::Square::E4);
    /// assert_eq!(moves[1].to(), sacrifice::Square::C5);
    /// ```
    pub fn moves_before(&self, node_id: Uuid) -> Vec<Move> {
        if let Some(node) = self.try_node(node_id) {
            return node.moves();
        }

        Vec::new()
    }
}

// Methods that changes the order of branches in the tree
impl Game {
    /// Promotes a node to the mainline variation of its parent.
    ///
    /// # Arguments
    ///
    /// * `node_id` - id of the node to promote
    ///
    /// # Examples
    ///
    /// ```
    /// let mut game = sacrifice::Game::from_pgn("1. d4 (1. e4) 1... d5");
    /// let variation_node_1_0 = game.other_variations(game.root())[0]; // (1. e4)
    /// assert_eq!(game.promote_variation(variation_node_1_0), Some(variation_node_1_0)); // Now mainline
    /// assert_eq!(game.mainline(game.root()), Some(variation_node_1_0));
    /// ```
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
    /// Add a move to a given node in the game tree.
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
    /// let mut game = sacrifice::Game::from_pgn("1. d4");
    /// let mainline_node_1 = game.mainline(game.root()).unwrap();
    /// let illegal_move = sacrifice::Move::Normal {
    ///    role: sacrifice::Role::Queen,
    ///    from: sacrifice::Square::D8,
    ///    capture: None,
    ///    to: sacrifice::Square::H4,
    ///    promotion: None,
    /// };
    /// assert_eq!(game.add_node(mainline_node_1, illegal_move), None);
    /// let legal_move = sacrifice::Move::Normal {
    ///    role: sacrifice::Role::Pawn,
    ///    from: sacrifice::Square::E7,
    ///    capture: None,
    ///    to: sacrifice::Square::E5,
    ///    promotion: None,
    /// };
    /// let new_node = game.add_node(mainline_node_1, legal_move);
    /// assert_ne!(new_node, None);
    /// assert_eq!(game.mainline(mainline_node_1).unwrap(), new_node.unwrap());
    /// ```
    pub fn add_node(&mut self, parent_id: Uuid, m: Move) -> Option<Uuid> {
        let mut parent = self.try_node(parent_id)?;

        // Check if the move is legal
        let cur_position = parent.board(&self.initial_position);
        if !cur_position.is_legal(&m) {
            return None; // Not legal move
        }

        let new_node = parent.new_variation(m);
        self.node_map.insert(new_node.id(), new_node.clone()); // Update node map
        Some(new_node.id())
    }

    /// Remove all occurrences of the given node from the game tree.
    /// Returns the given node's id if successful.
    ///
    /// # Arguments
    ///
    /// * `node_id` - id of the given node
    ///
    /// # Examples
    ///
    /// ```
    /// let mut game = sacrifice::Game::from_pgn("1. d4");
    /// let mainline_node_1 = game.mainline(game.root()).unwrap();
    /// assert_ne!(game.remove_node(mainline_node_1), None); // No child nodes left
    /// assert_eq!(game.mainline(game.root()), None);
    /// ```
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
