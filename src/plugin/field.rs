use pyo3::*;

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Copy)]
pub enum Field {
    /// Zahlfeld
    Position1,
    /// Flaggenfeld
    Position2,
    /// Igelfeld: Hierauf kann nur rückwärts gezogen werden.
    Hedgehog,
    /// Salatfeld: Beim Betreteten wird im nächsten Zug ein Salat gegessen.
    Salad,
    /// Karottenfeld: Hier dürfen Karotten getauscht werden.
    Carrots,
    /// Hasenfeld: Hier wird sofort eine Hasenkarte gespielt.
    Hare,
    /// Marktfeld: Hier wird eine Hasenkarte gekauft (Variation).
    Market,
    /// Das Zielfeld.
    Goal,
    /// Das Startfeld
    Start,
}

// display

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::Position1 => write!(f, "Pos1"),
            Field::Position2 => write!(f, "Pos2"),
            Field::Hedgehog => write!(f, "Hedgehog"),
            Field::Salad => write!(f, "Salad"),
            Field::Carrots => write!(f, "Carrot"),
            Field::Hare => write!(f, "Hare"),
            Field::Market => write!(f, "Market"),
            Field::Goal => write!(f, "Goal"),
            Field::Start => write!(f, "Start"),
        }
    }
}
