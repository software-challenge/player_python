use pyo3::prelude::*;
use pyo3::exceptions::PyException;
use pyo3::create_exception;

create_exception!(_socha, WrongColor, PyException);
create_exception!(_socha, NotOnBorder, PyException);
create_exception!(_socha, NoSharedCorner, PyException);
create_exception!(_socha, WrongShape, PyException);
create_exception!(_socha, SkipFirstTurn, PyException);
create_exception!(_socha, DuplicateShape, PyException);
create_exception!(_socha, OutOfBounds, PyException);
create_exception!(_socha, Obstructed, PyException);
create_exception!(_socha, TouchesSameColor, PyException);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlokusMoveMistake {
    WrongColor,
    NotOnBorder,
    NoSharedCorner,
    WrongShape,
    SkipFirstTurn,
    DuplicateShape,
    OutOfBounds,
    Obstructed,
    TouchesSameColor,
}

impl BlokusMoveMistake {
    pub fn message(&self) -> &'static str {
        match self {
            BlokusMoveMistake::WrongColor => "Die Farbe des Zuges ist nicht an der Reihe",
            BlokusMoveMistake::NotOnBorder => "Der erste Zug muss an den Rand gesetzt werden",
            BlokusMoveMistake::NoSharedCorner => "Alle Teile müssen ein vorheriges Teil gleicher Farbe über mindestens eine Ecke berühren",
            BlokusMoveMistake::WrongShape => "Der erste Zug muss den festgelegten Spielstein setzen",
            BlokusMoveMistake::SkipFirstTurn => "Der erste Zug muss einen Stein setzen",
            BlokusMoveMistake::DuplicateShape => "Der gewählte Stein wurde bereits gesetzt",
            BlokusMoveMistake::OutOfBounds => "Der Spielstein passt nicht vollständig auf das Spielfeld",
            BlokusMoveMistake::Obstructed => "Der Spielstein würde eine andere Farbe überlagern",
            BlokusMoveMistake::TouchesSameColor => "Der Spielstein berührt ein Feld gleicher Farbe",
        }
    }

    pub fn to_py_err(&self) -> PyErr {
        let msg = self.message();
        match self {
            BlokusMoveMistake::WrongColor => WrongColor::new_err(msg),
            BlokusMoveMistake::NotOnBorder => NotOnBorder::new_err(msg),
            BlokusMoveMistake::NoSharedCorner => NoSharedCorner::new_err(msg),
            BlokusMoveMistake::WrongShape => WrongShape::new_err(msg),
            BlokusMoveMistake::SkipFirstTurn => SkipFirstTurn::new_err(msg),
            BlokusMoveMistake::DuplicateShape => DuplicateShape::new_err(msg),
            BlokusMoveMistake::OutOfBounds => OutOfBounds::new_err(msg),
            BlokusMoveMistake::Obstructed => Obstructed::new_err(msg),
            BlokusMoveMistake::TouchesSameColor => TouchesSameColor::new_err(msg),
        }
    }
}

impl std::fmt::Display for BlokusMoveMistake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl From<BlokusMoveMistake> for PyErr {
    fn from(mistake: BlokusMoveMistake) -> PyErr {
        mistake.to_py_err()
    }
}