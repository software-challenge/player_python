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
