use pyo3::prelude::*;

use crate::plugins::penguins::coordinate::{HexCoordinate, CartesianCoordinate};
use crate::plugins::penguins::field::Field;
use crate::plugins::penguins::penguin::Penguin;
use crate::plugins::penguins::r#move::Move;
use crate::plugins::penguins::team::{Team, TeamEnum};
use crate::plugins::penguins::vector::Vector;
use crate::plugins::penguins::bitboard::BitBoard;


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
                if self.board.one & (1 << index) != 0 {
                    let team: Team = Team::new("ONE".to_string());
                    let penguin: Penguin = Penguin::new(coordinate, team);
                    let field: Field = Field::new(coordinate, Some(penguin), 0);
                    row.push(field);
                } else if self.board.two & (1 << index) != 0 {
                    let team = Team::new("TWO".to_string());
                    let penguin: Penguin = Penguin::new(coordinate, team);
                    let field: Field = Field::new(coordinate, Some(penguin), 0);
                    row.push(field);
                } else {
                    let fish: i32 = self.board.get_fish(coordinate.to_cartesian().to_index());
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
        for i in 0..63 {
            if empty_bits & (1 << i) != 0 {
                let coordinate: HexCoordinate = CartesianCoordinate::from_index(i).to_hex();
                let fish: i32 = self.board.get_fish(coordinate.to_cartesian().to_index());
                let field: Field = Field::new(coordinate, None, fish);
                empty_fields.push(field);
            }
        }
        empty_fields
    }

    fn is_occupied(&self,  field: Field) -> bool {
        self.board.is_occupied(field.coordinate.to_cartesian().to_index())
    }

    pub fn is_valid(&self, coordinate: HexCoordinate) -> bool {
        self.board.is_valid(coordinate.to_cartesian().to_index())
    }

    pub fn get_field(&self, coordinate: HexCoordinate) -> Field {
        if self.is_valid(coordinate) {
            panic!("Coordinate is not valid");
        } else {
            let index: u64 = coordinate.to_cartesian().to_index();
            let bitboard: BitBoard = self.board.get_field(index);
            if bitboard.one != 0 {
                let team: Team = Team::new("ONE".to_string());
                let penguin: Penguin = Penguin::new(coordinate, team);
                let field: Field = Field::new(coordinate, Some(penguin), 0);
                field
            } else if bitboard.two != 0 {
                let team: Team = Team::new("TWO".to_string());
                let penguin: Penguin = Penguin::new(coordinate, team);
                let field: Field = Field::new(coordinate, Some(penguin), 0);
                field
            } else {
                let fish: i32 = self.board.get_fish(index);
                let field: Field = Field::new(coordinate, None, fish);
                field
            }
        }
    }

    pub fn contains_field(&self, field: Field) -> bool {
        self.board.contains_field(field.coordinate.to_cartesian().to_index())
    }

    pub fn contains(&self, fields: Vec<Field>) -> bool {
        for field in fields {
            if !self.contains_field(field) {
                return false;
            }
        }
        true
    }

    pub fn get_directive_moves(&self, coordinate: HexCoordinate, direction: Vector, team: Team) -> Vec<Move> {
        self.board.get_directive_moves(coordinate.to_cartesian().to_index(), direction, team)
    }

    pub fn get_moves_from(&self, coordinate: HexCoordinate, team: Team) -> Vec<Move> {
        self.board.get_moves_from(coordinate.to_cartesian().to_index(), team)
    }

    pub fn get_penguins(&self) -> Vec<Field> {
        let mut penguins = Vec::new();
        for i in 0..64 {
            if self.board.one & (1 << i) != 0 {
                let coordinate: HexCoordinate = CartesianCoordinate::from_index(i).to_hex();
                penguins.push(Field {
                    coordinate: coordinate.clone(),
                    fish: 0,
                    penguin: Some(Penguin {
                        position: coordinate.clone(),
                        team: Team::new("ONE".to_string()),
                    }),
                });
            }
            if self.board.two & (1 << i) != 0 {
                let coordinate: HexCoordinate = CartesianCoordinate::from_index(i).to_hex();
                penguins.push(Field {
                    coordinate: coordinate.clone(),
                    fish: 0,
                    penguin: Some(Penguin {
                        position: coordinate.clone(),
                        team: Team::new("TWO".to_string()),
                    }),
                });
            }
        }
        penguins
    }

    pub fn get_team_penguins(&self, team: Team) -> Vec<Field> {
        let mut penguins = Vec::new();
        for i in 0..64 {
            if team.name == TeamEnum::ONE && self.board.one & (1 << i) != 0 {
                let coordinate: HexCoordinate = CartesianCoordinate::from_index(i).to_hex();
                penguins.push(Field {
                    coordinate: coordinate.clone(),
                    fish: 0,
                    penguin: Some(Penguin {
                        position: coordinate.clone(),
                        team: Team::new("ONE".to_string()),
                    }),
                });
            }
            if team.name == TeamEnum::TWO && self.board.two & (1 << i) != 0 {
                let coordinate: HexCoordinate = CartesianCoordinate::from_index(i).to_hex();
                penguins.push(Field {
                    coordinate: coordinate.clone(),
                    fish: 0,
                    penguin: Some(Penguin {
                        position: coordinate.clone(),
                        team: Team::new("TWO".to_string()),
                    }),
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