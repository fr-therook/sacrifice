use super::writer::Visitor;
use std::fmt::Formatter;

#[derive(Clone)]
pub enum GameResult {
    Finished { white_score: u32, black_score: u32 },
    Ongoing,
}

impl From<&str> for GameResult {
    fn from(value: &str) -> Self {
        if value == "*" {
            return Self::Ongoing;
        }

        let vec = value.split("-").collect::<Vec<&str>>();
        if vec.len() != 2 {
            return Self::Ongoing;
        }

        let white_score = if let Ok(val) = vec[0].parse::<u32>() {
            val
        } else {
            return Self::Ongoing;
        };
        let black_score = if let Ok(val) = vec[1].parse::<u32>() {
            val
        } else {
            return Self::Ongoing;
        };

        Self::Finished {
            white_score,
            black_score,
        }
    }
}

impl std::fmt::Display for GameResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameResult::Finished {
                white_score,
                black_score,
            } => write!(f, "{}-{}", white_score, black_score),
            GameResult::Ongoing => write!(f, "*"),
        }
    }
}

#[derive(Clone)]
pub struct Header {
    pub event: Option<String>,
    pub site: Option<String>,
    pub date: Option<String>,
    pub round: Option<String>,
    pub white: Option<String>,
    pub black: Option<String>,
    pub result: GameResult,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            event: None,
            site: None,
            date: None,
            round: None,
            white: None,
            black: None,
            result: GameResult::Ongoing,
        }
    }
}

fn parse_header_value(value: &str) -> Option<String> {
    match value {
        "?" => None,
        "??" => None,
        _ => Some(value.to_string()),
    }
}

fn parse_header_date_value(value: &str) -> Option<String> {
    match value {
        "?" => None,
        "??" => None,
        "????.??.??" => None,
        _ => Some(value.to_string()),
    }
}

fn serialize_header_value(value: &Option<String>, default_str: &str) -> String {
    value.clone().unwrap_or(default_str.to_string())
}

impl Header {
    pub fn parse(&mut self, key: &str, value: &str) -> bool {
        match key {
            "Event" => self.event = parse_header_value(value),
            "Site" => self.site = parse_header_value(value),
            "Date" => self.date = parse_header_date_value(value),
            "Round" => self.round = parse_header_value(value),
            "White" => self.white = parse_header_value(value),
            "Black" => self.black = parse_header_value(value),
            "Result" => self.result = GameResult::from(value),
            _ => return false,
        }

        true
    }

    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_header("Event", &serialize_header_value(&self.event, "?"));
        visitor.visit_header("Site", &serialize_header_value(&self.site, "?"));
        visitor.visit_header("Date", &serialize_header_value(&self.date, "????.??.??"));
        visitor.visit_header("Round", &serialize_header_value(&self.round, "?"));
        visitor.visit_header("White", &serialize_header_value(&self.white, "?"));
        visitor.visit_header("Black", &serialize_header_value(&self.black, "?"));
        visitor.visit_header("Result", self.result.to_string().as_str());
    }
}
