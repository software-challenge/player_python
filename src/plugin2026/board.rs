use pyo3::*;

use crate::plugin2026::{
    field_type::FieldType,
    utils::{
        constants::PluginConstants,
        coordinate::Coordinate, 
        direction::Direction
    }
};

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Board {
    #[pyo3(get, set)]
    pub map: Vec<Vec<FieldType>>,
}

#[pymethods]
impl Board {
    #[new]
    pub fn new(map: Vec<Vec<FieldType>>) -> Self {
        Self { map }
    }

    fn __str__(&self) -> String {self.to_string()}
    fn __repr__(&self) -> String {format!("{:?}", self)}
    fn __eq__(&self, other: &Board) -> bool {self == other}
    fn __ne__(&self, other: &Board) -> bool {self != other}
    fn deepcopy(&self) -> Board {self.clone()}

    pub fn get_field(&self, position: &Coordinate) -> Option<FieldType> {

        let x = usize::try_from(position.x).ok()?;
        let y = usize::try_from(position.y).ok()?;

        self.map
            .get(y)?
            .get(x)
            .cloned()
    }

    pub fn get_fields_by_type(&self, field: FieldType) -> Vec<Coordinate> {

        let mut positions: Vec<Coordinate> = vec![];

        for (y, row) in self.map.iter().enumerate() {
            for (x, f) in row.iter().enumerate() {
                if *f == field {
                    positions.push(Coordinate {x: x as isize, y: y as isize});
                }
            }
        }

        positions
    }

    pub fn get_fields_in_direction(&self, position: &Coordinate, direction: &Direction) -> Vec<FieldType> {

        let mut fields: Vec<FieldType> = Vec::new();

        for scalar in 1..PluginConstants::BOARD_WIDTH {
            let new_pos = position.add_vector(&direction.to_vector().scale(scalar as isize));
            if let Some(field) = self.get_field(&new_pos) {
                fields.push(field);
            } else {break}
        }

        fields
    }

    pub fn get_fields_on_line(&self, position: &Coordinate, direction: &Direction) -> Vec<FieldType> {

        let mut fields: Vec<FieldType> = Vec::new();

        if let Some(field) = self.get_field(position) {
            fields.push(field);
        } else {return fields}

        let in_dir = self.get_fields_in_direction(position, direction);
        let in_mirror = self.get_fields_in_direction(position, &direction.to_mirrored());

        // merge vecs
        for f in in_dir {
            fields.push(f);
        }

        for f in in_mirror {
            fields.insert(0, f);
        }

        fields
    }

    pub fn get_fish_on_line(&self, position: &Coordinate, direction: &Direction) -> Vec<FieldType> {

        let fields_on_line = self.get_fields_on_line(position, direction);

        let fish_only: Vec<FieldType> = fields_on_line.iter()
            .filter(|&f| matches!(f, 
                FieldType::OneS | FieldType::OneM | FieldType::OneL |
                FieldType::TwoS | FieldType::TwoM | FieldType::TwoL
            ))
            .cloned()
            .collect();

        fish_only
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in self.map.iter().rev() {
            for field in row {
                write!(f, "{} ", *field)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
