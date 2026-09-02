use std::collections::HashSet;
use pyo3::*;

use crate::plugin2027::{
    color::Color, piece_shape::PieceShape, rotation::Rotation,
    utils::{coordinate::Coordinate, coordinate_helper::CoordinateSetExt},
};

#[pyclass]
#[derive(Debug, Clone)]
pub struct Piece {
    #[pyo3(get, set)]
    pub color: Color,
    #[pyo3(get, set)]
    pub kind: PieceShape,
    #[pyo3(get, set)]
    pub rotation: Rotation,
    #[pyo3(get, set)]
    pub is_flipped: bool,
    #[pyo3(get, set)]
    pub position: Coordinate,
}

#[pymethods]
impl Piece {
    #[new]
    pub fn new(
        color: Color,
        kind: PieceShape,
        rotation: Rotation,
        is_flipped: bool,
        position: Coordinate,
    ) -> Self {
        Self { color, kind, rotation, is_flipped, position }
    }

    /// Die normalisierte Form des Steins (gedreht/gespiegelt, aber nicht verschoben).
    pub fn shape(&self) -> HashSet<Coordinate> {
        self.kind
            .coordinates()
            .flip(self.is_flipped)
            .rotate(self.rotation)
    }

    /// Die tatsächlichen Koordinaten, die der Stein am Ende auf dem Feld einnimmt.
    pub fn coordinates(&self) -> HashSet<Coordinate> {
        self.shape()
            .iter()
            .map(|c| self.position.add_vector(&c.as_vector()))
            .collect()
    }

    /// Dreht und spiegelt den Stein entsprechend den gegebenen Parametern, Position bleibt gleich.
    pub fn transform(&self, rotation: Rotation, is_flipped: bool) -> Piece {
        Piece::new(self.color, self.kind, rotation, is_flipped, self.position)
    }

    fn __str__(&self) -> String {
        self.to_string()
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
    fn __eq__(&self, other: &Piece) -> bool {
        self == other
    }
    fn __ne__(&self, other: &Piece) -> bool {
        self != other
    }
    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

/// Gleichheit basiert nur auf Farbe und den tatsächlichen Koordinaten (wie im Original).
impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color && self.coordinates() == other.coordinates()
    }
}
impl Eq for Piece {}

impl std::hash::Hash for Piece {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.color.hash(state);
        let mut coords: Vec<Coordinate> = self.coordinates().into_iter().collect();
        // sortieren, damit die Reihenfolge das Hash-Ergebnis nicht beeinflusst
        coords.sort_by_key(|c| (c.x, c.y));
        coords.hash(state);
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rotation_part = if self.rotation != Rotation::NONE {
            format!(", {}", self.rotation)
        } else {
            String::new()
        };
        let flipped_part = if self.is_flipped { ", gespiegelt" } else { "" };
        write!(
            f,
            "{}({}{}{})[{},{}]",
            self.kind, self.color, rotation_part, flipped_part, self.position.x, self.position.y
        )
    }
}