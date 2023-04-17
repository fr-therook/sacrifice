pub use shakmaty::{fen::Fen, CastlingMode};
pub use shakmaty::{san::San, san::SanPlus};
pub use shakmaty::{Chess, Position};
pub use shakmaty::{Color, File, Move, Piece, Rank, Role, Square};

pub mod prelude;

mod game;
mod node;

mod header;

mod reader;
mod writer;

mod p;

#[cfg(test)]
mod tests;

/// Parse one chess game from PGN string.
///
/// # Arguments
///
/// * `pgn_str` - the "import formatted" PGN string
///
/// # Examples
///
/// ```
/// use sacrifice::prelude::*;
///
/// let game = sacrifice::read_pgn("1. e4 e5");
/// println!("{}", game); // Exports the game's PGN with default headers
/// ```
pub fn read_pgn(pgn: &str) -> p::GameImpl {
    use crate::game::Game;

    p::GameImpl::from_pgn(pgn)
}
