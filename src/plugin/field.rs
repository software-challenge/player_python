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
            Field::Position1 => write!(f, "1"),
            Field::Position2 => write!(f, "2"),
            Field::Hedgehog => write!(f, "I"),
            Field::Salad => write!(f, "S"),
            Field::Carrots => write!(f, "C"),
            Field::Hare => write!(f, "H"),
            Field::Market => write!(f, "M"),
            Field::Goal => write!(f, "G"),
            Field::Start => write!(f, "S"),
        }
    }
}
