use sac::Position;

#[cxx::bridge(namespace = "sacrifice")]
mod ffi {
    pub enum Color {
        Black = 0,
        White = 1,
    }

    pub enum Role {
        Pawn = 1,
        Knight = 2,
        Bishop = 3,
        Rook = 4,
        Queen = 5,
        King = 6,
    }

    pub struct Piece {
        pub color: Color,
        pub role: Role,
    }

    #[derive(PartialEq)]
    pub struct Square {
        pub index: u8,
    }

    extern "Rust" {
        fn square_from_coords(file: u8, rank: u8) -> Square;

        fn file(self: &Square) -> u8;
        fn rank(self: &Square) -> u8;
    }

    extern "Rust" {
        type Move;
        fn clone(&self) -> Box<Move>;

        fn from(&self) -> Square;
        fn to(&self) -> Square;

        fn is_promotion(&self) -> bool;
        fn set_promotion(&mut self, role: Role);

        fn is_en_passant(&self) -> bool;
        fn is_castle(&self) -> bool;
        fn castle_rook_from(&self) -> Square;
        fn castle_rook_to(&self) -> Square;

        fn to_string(&self) -> String;
    }

    extern "Rust" {
        type CurPosition;
        fn turn(&self) -> Color;

        fn squares(&self) -> Vec<Square>;
        fn pieces(&self) -> Vec<Piece>;

        fn piece_at(&self, square: Square) -> *const Piece;
        fn legal_move(&self, src: Square, dest: Square) -> *const Move;

        fn hints(&self, src: Square) -> Vec<Square>;
        fn captures(&self, src: Square) -> Vec<Square>;
    }

    extern "Rust" {
        type Node;

        fn position(&self) -> Box<CurPosition>;

        fn prev_move(&self) -> *const Move;
        fn prev_node(&self) -> *const Node;
        fn next_mainline_node(&self) -> *const Node;

        fn variations(&self) -> Vec<Node>;
        fn siblings(&self) -> Vec<Node>;
        fn mainline_nodes(&self) -> Vec<Node>;

        fn new_variation(&self, m: &Move) -> *const Node;
    }

    extern "Rust" {
        type GameTree;
        fn game_default() -> Box<GameTree>;

        fn root(&self) -> Box<Node>;
        fn initial_position(&self) -> Box<CurPosition>;

        fn pgn(&self) -> String;
    }
}

macro_rules! convert_enum {
    ($src: ty, $dst: ty, $($variant: ident,)+) => {
        impl From<$src> for $dst {
            fn from(value: $src) -> $dst {
                match value {
                    $(<$src>::$variant => <$dst>::$variant,)*
                    _ => unreachable!(),
                }
            }
        }
    }
}

convert_enum!(sac::Color, ffi::Color, Black, White,);

convert_enum!(
    sac::Role,
    ffi::Role,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
);

convert_enum!(
    ffi::Role,
    sac::Role,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
);

impl From<sac::Piece> for ffi::Piece {
    fn from(value: sac::Piece) -> ffi::Piece {
        ffi::Piece {
            color: value.color.into(),
            role: value.role.into(),
        }
    }
}

impl From<sac::Square> for ffi::Square {
    fn from(value: sac::Square) -> ffi::Square {
        ffi::Square {
            index: u8::from(value),
        }
    }
}

impl From<ffi::Square> for sac::Square {
    fn from(value: ffi::Square) -> sac::Square {
        sac::Square::new(value.index as u32)
    }
}

fn square_from_coords(file: u8, rank: u8) -> ffi::Square {
    let sq = sac::Square::from_coords(sac::File::new(file as u32), sac::Rank::new(rank as u32));
    ffi::Square {
        index: u8::from(sq),
    }
}

impl ffi::Square {
    fn file(&self) -> u8 {
        self.index & 7
    }
    fn rank(&self) -> u8 {
        self.index >> 3
    }
}

struct Move {
    inner: sac::Move,
    san: sac::SanPlus,
}

impl Move {
    fn clone(&self) -> Box<Move> {
        Box::new(Move {
            inner: self.inner.clone(),
            san: self.san.clone(),
        })
    }

    fn from(&self) -> ffi::Square {
        self.inner
            .from()
            .expect("a chess move always comes from somewhere")
            .into()
    }

    fn to(&self) -> ffi::Square {
        if let sac::Move::Castle { king: _, rook: _ } = self.inner {
            // Treat castling as special case
            let castling_side = self.inner.castling_side().unwrap();

            let to_rank = self.inner.from().unwrap().rank();
            let to_file = castling_side.king_to_file();

            let dest_sq = sac::Square::from_coords(to_file, to_rank);
            return dest_sq.into();
        }

        self.inner.to().into()
    }

    fn is_promotion(&self) -> bool {
        self.inner.is_promotion()
    }

    fn set_promotion(&mut self, role: ffi::Role) {
        if let sac::Move::Normal {
            ref mut promotion, ..
        } = self.inner
        {
            *promotion = Some(role.into());
        }
    }

    fn is_en_passant(&self) -> bool {
        self.inner.is_en_passant()
    }

    fn is_castle(&self) -> bool {
        self.inner.is_castle()
    }

    fn castle_rook_from(&self) -> ffi::Square {
        if let sac::Move::Castle { king: _, rook } = self.inner {
            return rook.into();
        }

        ffi::Square { index: 0 }
    }

    fn castle_rook_to(&self) -> ffi::Square {
        if self.inner.is_castle() {
            let castling_side = self.inner.castling_side().unwrap();
            let from_rank = self.inner.from().unwrap().rank();
            let to_file = castling_side.rook_to_file();

            return sac::Square::from_coords(to_file, from_rank).into();
        }

        ffi::Square { index: 0 }
    }

    fn to_string(&self) -> String {
        format!("{}", self.san)
    }
}

struct CurPosition(sac::Chess);

impl CurPosition {
    fn turn(&self) -> ffi::Color {
        self.0.turn().into()
    }

    fn squares(&self) -> Vec<ffi::Square> {
        let board = self.0.board().clone();

        board
            .into_iter()
            .map(|(sq, _)| sq.into())
            .collect::<Vec<ffi::Square>>()
    }

    fn pieces(&self) -> Vec<ffi::Piece> {
        let board = self.0.board().clone();

        board
            .into_iter()
            .map(|(_, p)| p.into())
            .collect::<Vec<ffi::Piece>>()
    }

    fn piece_at(&self, square: ffi::Square) -> *const ffi::Piece {
        let square: sac::Square = square.into();

        let ret: Box<ffi::Piece> = if let Some(inner) = self.0.board().piece_at(square) {
            Box::new(inner.into())
        } else {
            return std::ptr::null();
        };

        Box::into_raw(ret)
    }

    fn legal_move(&self, src: ffi::Square, dst: ffi::Square) -> *const Move {
        fn _impl(pos: &CurPosition, src_sq: ffi::Square, dst_sq: ffi::Square) -> Option<sac::Move> {
            let src_sq: sac::Square = src_sq.into();
            let dest_sq: sac::Square = dst_sq.into();

            let move_vec: Vec<sac::Move> = pos
                .0
                .legal_moves()
                .into_iter()
                .filter(|v| v.from().unwrap() == src_sq)
                .collect::<Vec<sac::Move>>();

            for m in move_vec {
                if let sac::Move::Castle { king, rook } = m {
                    let castling_side = m.castling_side().unwrap();
                    let to_file = castling_side.king_to_file();
                    let to_king_sq = sac::Square::from_coords(to_file, king.rank());
                    if to_king_sq == dest_sq {
                        return Some(m);
                    }
                    if rook == dest_sq {
                        return Some(m);
                    }
                    continue;
                }

                if m.to() == dest_sq {
                    // A legal move!
                    return Some(m);
                }
            }

            None
        }

        let ret = if let Some(inner) = _impl(self, src, dst) {
            inner
        } else {
            return std::ptr::null();
        };

        let san = sac::SanPlus::from_move(self.0.clone(), &ret);
        let ret = Box::new(Move { inner: ret, san });

        Box::into_raw(ret)
    }

    fn hints(&self, src: ffi::Square) -> Vec<ffi::Square> {
        self.legal_moves(src).0
    }

    fn captures(&self, src: ffi::Square) -> Vec<ffi::Square> {
        self.legal_moves(src).1
    }
}

impl CurPosition {
    fn legal_moves(&self, sq: ffi::Square) -> (Vec<ffi::Square>, Vec<ffi::Square>) {
        let sq: sac::Square = sq.into();
        let mut move_vec = self
            .0
            .legal_moves()
            .into_iter()
            .filter(|v| v.from().unwrap() == sq)
            .collect::<Vec<sac::Move>>();
        move_vec.dedup_by(|l, r| {
            if !l.is_promotion() || !r.is_promotion() {
                return false;
            }

            l.to() == r.to()
        });

        let (castling_move_vec, move_vec): (Vec<sac::Move>, Vec<sac::Move>) =
            move_vec.into_iter().partition(|m| m.is_castle());

        let (capture_vec, move_vec): (Vec<sac::Move>, Vec<sac::Move>) =
            move_vec.into_iter().partition(|m| m.capture().is_some());
        let mut move_vec = move_vec
            .into_iter()
            .map(|m| m.to().into())
            .collect::<Vec<ffi::Square>>();
        let mut capture_vec = capture_vec
            .into_iter()
            .map(|m| m.to().into())
            .collect::<Vec<ffi::Square>>();

        for castling_move in castling_move_vec {
            if let sac::Move::Castle { king: _, rook } = castling_move {
                let castling_side = castling_move.castling_side().unwrap();

                let to_rank = castling_move.from().unwrap().rank();
                let to_file = castling_side.king_to_file();

                move_vec.push(sac::Square::from_coords(to_file, to_rank).into());

                capture_vec.push(rook.into());
            }
        }

        (move_vec, capture_vec)
    }
}

struct Node(sac::game::Node);

impl Node {
    fn position(&self) -> Box<CurPosition> {
        Box::new(CurPosition(self.0.position()))
    }

    fn prev_move(&self) -> *const Move {
        let parent = if let Some(inner) = self.0.parent() {
            inner
        } else {
            return std::ptr::null();
        };
        let m = if let Some(inner) = self.0.prev_move() {
            inner
        } else {
            return std::ptr::null();
        };

        let pos_prev = parent.position();
        let san = sac::SanPlus::from_move(pos_prev, &m);
        let ret = Box::new(Move { inner: m, san });

        Box::into_raw(ret)
    }

    fn prev_node(&self) -> *const Node {
        let ret: Box<Node> = if let Some(inner) = self.0.parent() {
            Box::new(Node(inner))
        } else {
            return std::ptr::null();
        };

        Box::into_raw(ret)
    }

    fn next_mainline_node(&self) -> *const Node {
        let ret: Box<Node> = if let Some(inner) = self.0.mainline() {
            Box::new(Node(inner))
        } else {
            return std::ptr::null();
        };

        Box::into_raw(ret)
    }

    fn variations(&self) -> Vec<Node> {
        self.0
            .other_variations()
            .into_iter()
            .map(Node)
            .collect::<Vec<_>>()
    }

    fn siblings(&self) -> Vec<Node> {
        self.0.siblings().into_iter().map(Node).collect::<Vec<_>>()
    }

    fn mainline_nodes(&self) -> Vec<Node> {
        let mut node = self.0.clone();
        let mut node_vec: Vec<sac::game::Node> = Vec::new();
        while let Some(node_next) = node.mainline() {
            node_vec.push(node_next.clone());
            node = node_next;
        }

        node_vec.into_iter().map(Node).collect::<Vec<_>>()
    }

    fn new_variation(&self, m: &Move) -> *const Node {
        let ret: Box<Node> = if let Some(inner) = self
            .0
            .clone()
            .new_variation(m.inner.clone())
        {
            Box::new(Node(inner))
        } else {
            return std::ptr::null();
        };

        Box::into_raw(ret)
    }
}

#[derive(Debug, Clone, Default)]
struct GameTree(sac::game::Game);

fn game_default() -> Box<GameTree> {
    Box::default()
}

impl GameTree {
    fn root(&self) -> Box<Node> {
        Box::new(Node(self.0.root()))
    }

    fn initial_position(&self) -> Box<CurPosition> {
        Box::new(CurPosition(self.0.initial_position()))
    }

    fn pgn(&self) -> String {
        format!("{}", self.0)
    }
}
