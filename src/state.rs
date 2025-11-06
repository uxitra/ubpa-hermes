use std::fmt::Display;

#[derive(Debug, serde::Deserialize)]
pub enum State {
    Fresh,
    Old,
    None,
}

/// utility for displaying the state without manual matching
impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            State::Fresh => "fresh",
            State::Old => "old",
            State::None => "none",
        };

        write!(f, "{}", s)
    }
}
