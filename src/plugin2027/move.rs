use pyo3::prelude::*;

use crate::plugin2027::{color::Color, piece::Piece};

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Move {
    SetMove { piece: Piece },
    SkipMove { color: Color },
}

#[pymethods]
impl Move {
    #[staticmethod]
    pub fn set_move(piece: Piece) -> Move {
        Move::SetMove { piece }
    }

    #[staticmethod]
    pub fn skip_move(color: Color) -> Move {
        Move::SkipMove { color }
    }

    pub fn get_color(&self) -> Color {
        match self {
            Move::SetMove { piece } => piece.color,
            Move::SkipMove { color } => *color,
        }
    }

    pub fn as_piece(&self) -> Option<Piece> {
        match self {
            Move::SetMove { piece } => Some(piece.clone()),
            Move::SkipMove { .. } => None,
        }
    }

    fn __str__(&self) -> String {self.to_string()}
    fn __repr__(&self) -> String {format!("{:?}", self)}
    fn __eq__(&self, other: &Move) -> bool {self == other}
    fn __ne__(&self, other: &Move) -> bool {self != other}
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::SetMove { piece } => write!(f, "Setze {}", piece),
            Move::SkipMove { color } => write!(f, "{} setzt aus", color),
        }
    }
}