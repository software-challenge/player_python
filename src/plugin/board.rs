use pyo3::prelude::*;

use crate::plugin::coordinate::{HexCoordinate, CartesianCoordinate};
use crate::plugin::field::Field;
use crate::plugin::penguin::Penguin;
use crate::plugin::r#move::Move;
use crate::plugin::team::TeamEnum;
use crate::plugin::vector::Vector;
use crate::plugin::bitboard::BitBoard;


#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct Board {
    #[pyo3(get, set)]
    pub board: BitBoard, 
}

#[pymethods]
impl Board {
    #[new]
    pub fn new(board: Vec<Vec<Field>>) -> Self {
        let mut bitboard = BitBoard::default();
        for row in board.iter() {
            for field in row.iter() {
                bitboard.set_field(field);
            }
        }
        Board {
            board: bitboard,
        }
    }

    pub fn get_fields(&self) -> Vec<Vec<Field>> {
        let mut fields = Vec::new();
        for i in 0..7 {
            let mut row = Vec::new();
            for j in 0..7 {
                let index = i * 5 + j;
                let coordinate: HexCoordinate = CartesianCoordinate::new(i, j).to_hex();
                if self.board.one >> index & 1 != 0 {
                    let penguin: Penguin = Penguin::new(coordinate, TeamEnum::ONE);
                    let field: Field = Field::new(coordinate, Some(penguin), 0);
                    row.push(field);
                } else if self.board.two >> index & 1 != 0 {
                    let penguin: Penguin = Penguin::new(coordinate, TeamEnum::TWO);
                    let field: Field = Field::new(coordinate, Some(penguin), 0);
                    row.push(field);
                } else {
                    let fish: i32 = self.board.get_fish(coordinate.to_cartesian().to_index().unwrap());
                    let field: Field = Field::new(coordinate, None, fish);
                    row.push(field);
                }
            }
            fields.push(row);
        }
        fields
    }

    fn get_empty_fields(&self) -> Vec<Field> {
        let mut empty_fields = Vec::new();
        let empty_bits: u64 = self.board.get_empty_bits();
        for index in 0..63 {
            if empty_bits >> index & 1 != 0 {
                let coordinate: HexCoordinate = CartesianCoordinate::from_index(index).to_hex();
                let fish: i32 = self.board.get_fish(coordinate.to_cartesian().to_index().unwrap());
                let field: Field = Field::new(coordinate, None, fish);
                empty_fields.push(field);
            }
        }
        empty_fields
    }

    fn is_occupied(&self,  field: Field) -> bool {
        self.board.is_occupied(field.coordinate.to_cartesian().to_index().unwrap())
    }

    pub fn is_valid(&self, coordinate: HexCoordinate) -> bool {
        self.board.is_valid(coordinate.to_cartesian().to_index().unwrap())
    }

    pub fn get_field(&self, coordinate: HexCoordinate) -> Field {
        if self.is_valid(coordinate) {
            panic!("Coordinate is not valid");
        } else {
            let index: u64 = coordinate.to_cartesian().to_index().unwrap();
            return self.board.get_field(index);
        }
    }

    pub fn contains_field(&self, field: Field) -> bool {
        self.board.contains_field(field.coordinate.to_cartesian().to_index().unwrap())
    }

    pub fn contains(&self, fields: Vec<Field>) -> bool {
        for field in fields {
            if !self.contains_field(field) {
                return false;
            }
        }
        true
    }

    pub fn get_directive_moves(&self, coordinate: HexCoordinate, direction: Vector, team: TeamEnum) -> Vec<Move> {
        self.board.get_directive_moves(coordinate.to_cartesian().to_index().unwrap(), direction, team)
    }

    pub fn possible_moves_from(&self, coordinate: HexCoordinate, team: TeamEnum) -> Vec<Move> {
        self.board.possible_moves_from(coordinate.to_cartesian().to_index().unwrap(), team)
    }

    pub fn get_penguins(&self) -> Vec<Penguin> {
        let mut penguins: Vec<Penguin> = Vec::new();
        for index in 0..64 {
            if self.board.one >> index & 1 != 0 {
                let coordinate: HexCoordinate = CartesianCoordinate::from_index(index).to_hex();
                penguins.push(Penguin {
                    position: coordinate.clone(),
                    team: TeamEnum::ONE
                });
            }
            if self.board.two >> index & 1 != 0 {
                let coordinate: HexCoordinate = CartesianCoordinate::from_index(index).to_hex();
                penguins.push(Penguin {
                    position: coordinate.clone(),
                    team: TeamEnum::TWO
                });
            }
        }
        penguins
    }

    pub fn get_teams_penguins(&self, team: TeamEnum) -> Vec<Penguin> {
        let mut penguins: Vec<Penguin> = Vec::new();
        for index in 0..64 {
            if team == TeamEnum::ONE && self.board.one >> index & 1 != 0 {
                let coordinate: HexCoordinate = CartesianCoordinate::from_index(index).to_hex();
                penguins.push(Penguin {
                    position: coordinate.clone(),
                    team: TeamEnum::ONE
                });
            }
            if team == TeamEnum::TWO && self.board.two >> index & 1 != 0 {
                let coordinate: HexCoordinate = CartesianCoordinate::from_index(index).to_hex();
                penguins.push(Penguin {
                    position: coordinate.clone(),
                    team: TeamEnum::TWO
                });
            }
        }
        penguins
    }

    fn __repr__(&self) -> String {
        self.board.__repr__()
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.__repr__()) 
    }
}