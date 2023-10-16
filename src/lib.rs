pub use shakmaty::{fen::Fen, CastlingMode};
pub use shakmaty::{san::San, san::SanPlus};
pub use shakmaty::{Chess, Position};
pub use shakmaty::{Color, File, Move, Piece, Rank, Role, Square};

pub mod game;
mod pgn;

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
/// let game = sacrifice::read_pgn("1. e4 e5");
/// println!("{}", game); // Exports the game's PGN with default headers
/// ```
pub fn read_pgn(pgn: &str) -> game::Game {
    pgn::reader::read_pgn(pgn).unwrap()
}
