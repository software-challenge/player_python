use pyo3::*;

use crate::plugin2027::{
    color::Color, field::Field, field_content::FieldContent,
    utils::{constants::Constants, coordinate::Coordinate},
};

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    #[pyo3(get, set)]
    pub map: Vec<Vec<Field>>,
}

#[pymethods]
impl Board {
    #[new]
    pub fn new(map: Option<Vec<Vec<Field>>>) -> Self {
        Self {
            map: map.unwrap_or_else(Self::random_fields),
        }
    }

    /// Zugriff auf ein Feld über Koordinaten (entspricht Kotlins `this[position]`).
    pub fn get(&self, position: Coordinate) -> PyResult<Field> {
        self.map
            .get(position.y as usize)
            .and_then(|row| row.get(position.x as usize))
            .cloned()
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Coordinate out of bounds"))
    }

    /// Setzt den Inhalt eines Feldes über Koordinaten.
    pub fn set_content(&mut self, position: Coordinate, content: FieldContent) {
        if let Some(row) = self.map.get_mut(position.y as usize) {
            if let Some(field) = row.get_mut(position.x as usize) {
                field.content = content;
            }
        }
    }

    pub fn get_content(&self, position: Coordinate) -> Option<FieldContent> {
        self.map
            .get(position.y as usize)
            .and_then(|row| row.get(position.x as usize))
            .map(|field| field.content)
    }

    pub fn is_empty(&self) -> bool {
        self.map
            .iter()
            .all(|row| row.iter().all(|f| f.content == FieldContent::EMPTY))
    }

    pub fn is_obstructed(&self, position: Coordinate) -> bool {
        self.get_content(position)
            .is_some_and(|c| c != FieldContent::EMPTY)
    }

    pub fn get_team(&self, position: Coordinate) -> Option<Color> {
        self.get_content(position).and_then(|c| c.to_team_color())
    }

    pub fn pretty_string(&self) -> String {
        self.map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|f| f.content.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Vergleicht zwei Boards und gibt die Felder zurück, die sich unterscheiden (Werte von `other`).
    pub fn compare(&self, other: &Board) -> Vec<Field> {
        let mut different = Vec::new();
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                let field = &self.map[y][x];
                let other_field = &other.map[y][x];
                if field != other_field {
                    different.push(other_field.clone());
                }
            }
        }
        different
    }

    #[staticmethod]
    pub fn random_fields() -> Vec<Vec<Field>> {
        (0..Constants::BOARD_LENGTH)
            .map(|y| {
                (0..Constants::BOARD_LENGTH)
                    .map(|x| Field {
                        coordinate: Coordinate::new(x as isize, y as isize),
                        content: FieldContent::EMPTY,
                    })
                    .collect()
            })
            .collect()
    }

    #[staticmethod]
    pub fn contains(position: Coordinate) -> bool {
        let len = Constants::BOARD_LENGTH as isize;
        position.x >= 0 && position.x < len && position.y >= 0 && position.y < len
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Board {}", self.pretty_string())
    }
}