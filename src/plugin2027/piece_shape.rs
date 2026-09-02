use pyo3::prelude::*;
use std::collections::{HashSet};

use crate::plugin2027::{
    utils::{coordinate::Coordinate, vector::Vector, coordinate_helper::CoordinateSetExt},
    rotation::Rotation,
};

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceShape {
    Mono,
    Domino,
    TrioL,
    TrioI,
    TetroO,
    TetroT,
    TetroI,
    TetroL,
    TetroZ,
    PentoL,
    PentoT,
    PentoV,
    PentoS,
    PentoZ,
    PentoI,
    PentoP,
    PentoW,
    PentoU,
    PentoR,
    PentoX,
    PentoY,
}

#[pymethods]
impl PieceShape {
    /// Alle 21 Formen in Reihenfolge (entspricht dem Enum-Index in Kotlin).
    #[staticmethod]
    pub fn all() -> Vec<PieceShape> {
        vec![
            PieceShape::Mono, PieceShape::Domino, PieceShape::TrioL, PieceShape::TrioI,
            PieceShape::TetroO, PieceShape::TetroT, PieceShape::TetroI, PieceShape::TetroL,
            PieceShape::TetroZ, PieceShape::PentoL, PieceShape::PentoT, PieceShape::PentoV,
            PieceShape::PentoS, PieceShape::PentoZ, PieceShape::PentoI, PieceShape::PentoP,
            PieceShape::PentoW, PieceShape::PentoU, PieceShape::PentoR, PieceShape::PentoX,
            PieceShape::PentoY,
        ]
    }

    /// Gibt die Form anhand ihres Index zurück (analog zu Kotlins `shapes`-Map).
    #[staticmethod]
    pub fn from_index(index: usize) -> Option<PieceShape> {
        PieceShape::all().get(index).copied()
    }

    /// Die normalisierten (an (0,0) ausgerichteten) Koordinaten der Grundform.
    pub fn coordinates(&self) -> HashSet<Coordinate> {
        let coords: &[(isize, isize)] = match self {
            PieceShape::Mono    => &[(0, 0)],
            PieceShape::Domino  => &[(0, 0), (1, 0)],
            PieceShape::TrioL   => &[(0, 0), (0, 1), (1, 1)],
            PieceShape::TrioI   => &[(0, 0), (0, 1), (0, 2)],
            PieceShape::TetroO  => &[(0, 0), (1, 0), (0, 1), (1, 1)],
            PieceShape::TetroT  => &[(0, 0), (1, 0), (2, 0), (1, 1)],
            PieceShape::TetroI  => &[(0, 0), (0, 1), (0, 2), (0, 3)],
            PieceShape::TetroL  => &[(0, 0), (0, 1), (0, 2), (1, 2)],
            PieceShape::TetroZ  => &[(0, 0), (1, 0), (1, 1), (2, 1)],
            PieceShape::PentoL  => &[(0, 0), (0, 1), (0, 2), (0, 3), (1, 3)],
            PieceShape::PentoT  => &[(0, 0), (1, 0), (2, 0), (1, 1), (1, 2)],
            PieceShape::PentoV  => &[(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
            PieceShape::PentoS  => &[(1, 0), (2, 0), (3, 0), (0, 1), (1, 1)],
            PieceShape::PentoZ  => &[(0, 0), (1, 0), (1, 1), (1, 2), (2, 2)],
            PieceShape::PentoI  => &[(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)],
            PieceShape::PentoP  => &[(0, 0), (1, 0), (0, 1), (1, 1), (0, 2)],
            PieceShape::PentoW  => &[(0, 0), (0, 1), (1, 1), (1, 2), (2, 2)],
            PieceShape::PentoU  => &[(0, 0), (0, 1), (1, 1), (2, 1), (2, 0)],
            PieceShape::PentoR  => &[(0, 1), (1, 1), (1, 2), (2, 1), (2, 0)],
            PieceShape::PentoX  => &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            PieceShape::PentoY  => &[(0, 1), (1, 0), (1, 1), (1, 2), (1, 3)],
        };
        coords
            .iter()
            .map(|&(x, y)| Coordinate::new(x, y))
            .collect::<HashSet<_>>()
            .align()
    }

    /// Das kleinstmögliche Rechteck, das die Form umfasst.
    pub fn dimension(&self) -> Vector {
        self.coordinates().area()
    }

    /// Die Form als Menge von Vektoren relativ zu (0,0).
    pub fn as_vectors(&self) -> HashSet<Vector> {
        self.coordinates()
            .iter()
            .map(|c| c.get_difference(&Coordinate::new(0, 0)))
            .collect()
    }

    /// Die Anzahl der Felder, die diese Form belegt.
    pub fn size(&self) -> usize {
        self.coordinates().len()
    }

    /// Alle eindeutigen Varianten der Form (Rotation + Spiegelung), ohne Duplikate.
    /// Gibt eine Liste von (Koordinatenmenge, Rotation, isFlipped) zurück.
    pub fn variants(&self) -> Vec<(HashSet<Coordinate>, Rotation, bool)> {
        let base = self.coordinates();
        let mut seen: Vec<HashSet<Coordinate>> = Vec::new();
        let mut result = Vec::new();

        for rotation in Rotation::all() {
            for flip in [false, true] {
                let shape = base.rotate(rotation).flip(flip);
                if !seen.contains(&shape) {
                    seen.push(shape.clone());
                    result.push((shape, rotation, flip));
                }
            }
        }
        result
    }

    /// Transformiert die Form entsprechend Rotation und Spiegelung.
    /// Entspricht Kotlins `transform`/`get`-Operator.
    pub fn transform(&self, rotation: Rotation, should_flip: bool) -> HashSet<Coordinate> {
        self.coordinates().rotate(rotation).flip(should_flip)
    }
    
    pub fn name(&self) -> String {
        format!("{:?}", self)
    }
}

impl std::fmt::Display for PieceShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            PieceShape::Mono   => "Mono",
            PieceShape::Domino => "Domino",
            PieceShape::TrioL  => "Trio-L",
            PieceShape::TrioI  => "Trio-I",
            PieceShape::TetroO => "Tetro-O",
            PieceShape::TetroT => "Tetro-T",
            PieceShape::TetroI => "Tetro-I",
            PieceShape::TetroL => "Tetro-L",
            PieceShape::TetroZ => "Tetro-Z",
            PieceShape::PentoL => "Pento-L",
            PieceShape::PentoT => "Pento-T",
            PieceShape::PentoV => "Pento-V",
            PieceShape::PentoS => "Pento-S",
            PieceShape::PentoZ => "Pento-Z",
            PieceShape::PentoI => "Pento-I",
            PieceShape::PentoP => "Pento-P",
            PieceShape::PentoW => "Pento-W",
            PieceShape::PentoU => "Pento-U",
            PieceShape::PentoR => "Pento-R",
            PieceShape::PentoX => "Pento-X",
            PieceShape::PentoY => "Pento-Y",
        };
        write!(f, "{}", name)
    }
}