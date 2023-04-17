use crate::{Chess, Color, Move, Position};

pub struct Skip(pub bool);

pub trait Visitor {
    type Result;

    fn begin_game(&mut self);

    fn begin_headers(&mut self);
    fn visit_header(&mut self, tag_name: &str, tag_value: &str);
    fn end_headers(&mut self);

    fn visit_move(&mut self, board: Chess, next_move: Move);
    fn visit_comment(&mut self, comment: String);
    fn visit_nag(&mut self, nag: u8);

    fn begin_variation(&mut self) -> Skip;
    fn end_variation(&mut self);

    fn visit_result(&mut self, result: &str);

    fn end_game(&mut self) -> Self::Result;
}

pub struct PgnWriter {
    max_width: Option<u32>,

    line_vec: Vec<String>,
    cur_line: String,

    force_move_number: bool,
}

impl PgnWriter {
    pub fn new() -> Self {
        Self {
            max_width: None,

            line_vec: Vec::new(),
            cur_line: String::new(),

            force_move_number: false,
        }
    }

    pub fn with_max_width(max_width: u32) -> Self {
        Self {
            max_width: Some(max_width),

            line_vec: Vec::new(),
            cur_line: String::new(),

            force_move_number: false,
        }
    }
}

impl PgnWriter {
    fn flush(&mut self) {
        let cur_line = self.cur_line.trim();
        if cur_line.is_empty() {
            // Nothing to write
            return;
        }

        self.line_vec.push(cur_line.to_string());
        self.cur_line = String::new();
    }

    fn write_token(&mut self, token: impl AsRef<str>) {
        let token = token.as_ref();

        if let Some(max_width) = self.max_width {
            if ((max_width as usize) < self.cur_line.len())
                || (max_width as usize - self.cur_line.len() < token.len())
            {
                self.flush();
            }
        }

        self.cur_line = format!("{}{}", self.cur_line, token)
    }

    fn write_line(&mut self, new_line: String) {
        self.flush();
        self.line_vec.push(new_line.trim().to_string())
    }
}

impl Visitor for PgnWriter {
    type Result = Vec<String>;

    fn begin_game(&mut self) {
        self.line_vec = Vec::new();
        self.cur_line = String::new();
        self.force_move_number = false;
    }

    fn begin_headers(&mut self) {
        // Nothing to do
    }

    fn visit_header(&mut self, tag_name: &str, tag_value: &str) {
        self.write_line(format!("[{} \"{}\"]", tag_name, tag_value));
    }

    fn end_headers(&mut self) {
        self.write_line(String::new());
    }

    fn visit_move(&mut self, board: Chess, next_move: Move) {
        let move_prefix = if board.turn() == Color::White {
            format!("{}. ", board.fullmoves())
        } else if self.force_move_number {
            format!("{}... ", board.fullmoves())
        } else {
            String::new()
        };

        let san = shakmaty::san::SanPlus::from_move(board, &next_move);
        self.write_token(format!("{}{} ", move_prefix, san));

        self.force_move_number = false;
    }

    fn visit_comment(&mut self, comment: String) {
        self.write_token(format!("{{ {} }} ", comment.trim()));
        self.force_move_number = true;
    }

    fn visit_nag(&mut self, nag: u8) {
        self.write_token(format!("${} ", nag));
    }

    fn begin_variation(&mut self) -> Skip {
        self.force_move_number = true;
        self.write_token("( ");

        Skip(false)
    }

    fn end_variation(&mut self) {
        self.force_move_number = true;
        self.write_token(") ");
    }

    fn visit_result(&mut self, result: &str) {
        self.write_token(format!("{} ", result));
    }

    fn end_game(&mut self) -> Self::Result {
        self.flush(); // Or write a new line?
        std::mem::take(&mut self.line_vec)
    }
}
