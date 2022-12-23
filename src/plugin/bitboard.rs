use pyo3::prelude::*;

use crate::plugin::coordinate::{CartesianCoordinate, HexCoordinate};
use crate::plugin::field::Field;
use crate::plugin::penguin::Penguin;
use crate::plugin::r#move::Move;
use crate::plugin::team::TeamEnum;
use crate::plugin::vector::Vector;

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Hash)]
pub struct BitBoard {
    #[pyo3(get, set)]
    pub one: u64,
    #[pyo3(get, set)]
    pub two: u64,
    #[pyo3(get, set)]
    pub fish_0: u64,
    #[pyo3(get, set)]
    pub fish_1: u64,
    #[pyo3(get, set)]
    pub fish_2: u64,
    #[pyo3(get, set)]
    pub fish_3: u64,
    #[pyo3(get, set)]
    pub fish_4: u64,
}

#[pymethods]
impl BitBoard {
    #[new]
    pub fn new(one: u64, two: u64, fish_0: u64, fish_1: u64, fish_2: u64,
               fish_3: u64, fish_4: u64) -> Self {
        BitBoard {
            one,
            two,
            fish_0,
            fish_1,
            fish_2,
            fish_3,
            fish_4,
        }
    }

    pub fn equivalence(&self, other: &BitBoard) -> bool {
        self.one == other.one
            && self.two == other.two
            && self.fish_0 == other.fish_0
            && self.fish_1 == other.fish_1
            && self.fish_2 == other.fish_2
            && self.fish_3 == other.fish_3
            && self.fish_4 == other.fish_4
    }

    pub fn is_empty(&self) -> bool {
        self.one == 0 &&
            self.two == 0 &&
            self.fish_0 == 0 &&
            self.fish_1 == 0 &&
            self.fish_2 == 0 &&
            self.fish_3 == 0 &&
            self.fish_4 == 0
    }

    pub fn intersection(&self, other: &BitBoard) -> BitBoard {
        BitBoard {
            one: self.one & other.one,
            two: self.two & other.two,
            fish_0: self.fish_0 & other.fish_0,
            fish_1: self.fish_1 & other.fish_1,
            fish_2: self.fish_2 & other.fish_2,
            fish_3: self.fish_3 & other.fish_3,
            fish_4: self.fish_4 & other.fish_4,
        }
    }

    pub fn union(&self, other: &BitBoard) -> BitBoard {
        BitBoard {
            one: self.one | other.one,
            two: self.two | other.two,
            fish_0: self.fish_0 | other.fish_0,
            fish_1: self.fish_1 | other.fish_1,
            fish_2: self.fish_2 | other.fish_2,
            fish_3: self.fish_3 | other.fish_3,
            fish_4: self.fish_4 | other.fish_4,
        }
    }

    pub fn difference(&self, other: &BitBoard) -> BitBoard {
        BitBoard {
            one: self.one & !other.one,
            two: self.two & !other.two,
            fish_0: self.fish_0 & !other.fish_0,
            fish_1: self.fish_1 & !other.fish_1,
            fish_2: self.fish_2 & !other.fish_2,
            fish_3: self.fish_3 & !other.fish_3,
            fish_4: self.fish_4 & !other.fish_4,
        }
    }

    pub fn disjoint(&self, other: &BitBoard) -> bool {
        self.intersection(other).is_empty()
    }

    pub fn complement(&self, other: &BitBoard) -> BitBoard {
        BitBoard {
            one: !self.one & other.one,
            two: !self.two & other.two,
            fish_0: !self.fish_0 & other.fish_0,
            fish_1: !self.fish_1 & other.fish_1,
            fish_2: !self.fish_2 & other.fish_2,
            fish_3: !self.fish_3 & other.fish_3,
            fish_4: !self.fish_4 & other.fish_4,
        }
    }

    pub fn implication(&self, other: &BitBoard) -> BitBoard {
        BitBoard {
            one: !self.one | other.one,
            two: !self.two | other.two,
            fish_0: !self.fish_0 | other.fish_0,
            fish_1: !self.fish_1 | other.fish_1,
            fish_2: !self.fish_2 | other.fish_2,
            fish_3: !self.fish_3 | other.fish_3,
            fish_4: !self.fish_4 | other.fish_4,
        }
    }

    pub fn exclusive_or(&self, other: &BitBoard) -> BitBoard {
        BitBoard {
            one: self.one ^ other.one,
            two: self.two ^ other.two,
            fish_0: self.fish_0 ^ other.fish_0,
            fish_1: self.fish_1 ^ other.fish_1,
            fish_2: self.fish_2 ^ other.fish_2,
            fish_3: self.fish_3 ^ other.fish_3,
            fish_4: self.fish_4 ^ other.fish_4,
        }
    }

    pub fn update(&mut self, r#move: &Move) {
        if r#move._from != None {
            let from = r#move._from.clone().unwrap().to_cartesian().to_index().unwrap();
            let to = r#move.to.to_cartesian().to_index().unwrap();
            match r#move.team {
                TeamEnum::ONE => {
                    self.one ^= 1 << from;
                    self.one |= 1 << to;
                }
                TeamEnum::TWO => {
                    self.two ^= 1 << from;
                    self.two |= 1 << to;
                }
            }
        } else {
            let to = r#move.to.to_cartesian().to_index().unwrap();
            match r#move.team {
                TeamEnum::ONE => {
                    self.one |= 1 << to;
                }
                TeamEnum::TWO => {
                    self.two |= 1 << to;
                }
            }
        }
    }

    pub fn set_field(&mut self, field: &Field) {
        if field.penguin.is_some() {
            let penguin: Penguin = field.penguin.clone().unwrap();
            let index: u64 = penguin.position.to_cartesian().to_index().unwrap();
            match penguin.team {
                TeamEnum::ONE => {
                    self.one |= 1 << index;
                }
                TeamEnum::TWO => {
                    self.two |= 1 << index;
                }
            }
        } else {
            let fish: i32 = field.fish.clone();
            let index: u64 = field.coordinate.to_cartesian().to_index().unwrap();
            match fish {
                0 => {
                    self.fish_0 |= 1 << index;
                }
                1 => {
                    self.fish_1 |= 1 << index;
                }
                2 => {
                    self.fish_2 |= 1 << index;
                }
                3 => {
                    self.fish_3 |= 1 << index;
                }
                4 => {
                    self.fish_4 |= 1 << index;
                }
                _ => { panic!("Fish value not allowed.\nFish value was: {}", fish) }
            }
        }
    }

    pub fn get_field(&self, index: u64) -> Field {
        let coordinate: HexCoordinate = CartesianCoordinate::from_index(index).to_hex();
        let penguin: Option<Penguin> = self.get_penguin(&coordinate);
        let fish: i32 = self.get_fish(coordinate.to_cartesian().to_index().unwrap());
        let field: Field = Field::new(coordinate, penguin, fish);
        field
    }

    pub fn get_penguin(&self, coordinate: &HexCoordinate) -> Option<Penguin> {
        let index: u64 = coordinate.to_cartesian().to_index().unwrap();
        if self.one & (1 << index) != 0 {
            Some(Penguin::new(coordinate.clone(), TeamEnum::ONE))
        } else if self.two & (1 << index) != 0 {
            Some(Penguin::new(coordinate.clone(), TeamEnum::TWO))
        } else {
            None
        }
    }

    pub fn get_fish(&self, index: u64) -> i32 {
        if self.fish_0 & (1 << index) != 0 {
            0
        } else if self.fish_1 & (1 << index) != 0 {
            1
        } else if self.fish_2 & (1 << index) != 0 {
            2
        } else if self.fish_3 & (1 << index) != 0 {
            3
        } else if self.fish_4 & (1 << index) != 0 {
            4
        } else {
            0
        }
    }

    pub fn get_empty_bits(&self) -> u64 {
        !(self.one | self.two | self.fish_0 | self.fish_1 | self.fish_2 | self.fish_3 | self.fish_4)
    }

    pub fn is_occupied(&self, index: u64) -> bool {
        self.one & (1 << index) != 0 || self.two & (1 << index) != 0
    }

    pub fn is_valid(&self, index: u64) -> bool {
        index < 64
    }

    pub fn contains_field(&self, index: u64) -> bool {
        self.one & (1 << index) != 0 ||
            self.two & (1 << index) != 0 ||
            self.fish_0 & (1 << index) != 0 ||
            self.fish_1 & (1 << index) != 0 ||
            self.fish_2 & (1 << index) != 0 ||
            self.fish_3 & (1 << index) != 0 ||
            self.fish_4 & (1 << index) != 0
    }

    pub fn contains(&self, indexes: Vec<u64>) -> bool {
        for index in indexes {
            if !self.contains_field(index) {
                return false;
            }
        }
        true
    }

    pub fn is_team(&self, team: TeamEnum, index: u64) -> bool {
        match team {
            TeamEnum::ONE => self.one & (1 << index) != 0,
            TeamEnum::TWO => self.two & (1 << index) != 0,
        }
    }

    pub fn get_coordinates(&self, bitboard: u64) -> Vec<HexCoordinate> {
        let mut coordinates: Vec<HexCoordinate> = Vec::new();
        for index in 0..64 {
            if bitboard & (1 << index) != 0 {
                coordinates.push(CartesianCoordinate::from_index(index).to_hex());
            }
        }
        coordinates
    }

    fn get_bit_coordinate(&self, field: BitBoard) -> Option<HexCoordinate> {
        let mut count: i32 = 0;
        let mut index: u64 = 0;
        for i in 0..64 {
            if field.one & (1 << index) != 0 {
                count += 1;
                index = i;
            }
            if field.two & (1 << index) != 0 {
                count += 1;
                index = i;
            }
            if field.fish_0 & (1 << index) != 0 {
                count += 1;
                index = i;
            }
            if field.fish_1 & (1 << index) != 0 {
                count += 1;
                index = i;
            }
            if field.fish_2 & (1 << index) != 0 {
                count += 1;
                index = i;
            }
            if field.fish_3 & (1 << index) != 0 {
                count += 1;
                index = i;
            }
            if field.fish_4 & (1 << index) != 0 {
                count += 1;
                index = i;
            }
        }
        if count == 1 {
            Some(CartesianCoordinate::from_index(index).to_hex())
        } else {
            panic!("More than one bit set in bitboards");
        }
    }

    pub fn get_directive_moves(&self, index: u64, direction: Vector, team: TeamEnum) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let origin: HexCoordinate = CartesianCoordinate::from_index(index).to_hex();
        if self.is_team(team.clone(), index) {
            let mut new_index: Option<u64> = CartesianCoordinate::
            from_index(index).to_hex().add_vector(&direction).to_cartesian().to_index();
            while new_index.is_some() &&
                self.is_valid(new_index.unwrap()) &&
                !self.is_occupied(new_index.unwrap()) &&
                self.get_fish(new_index.unwrap()) > 0 {
                moves.push(Move::new(Some(origin.clone()),
                                     CartesianCoordinate::from_index(new_index.unwrap()).to_hex(), team.clone()));
                new_index = CartesianCoordinate::
                from_index(new_index.unwrap()).to_hex().add_vector(&direction).to_cartesian().to_index();
            }
        }
        moves
    }

    pub fn possible_moves_from(&self, index: u64, team: TeamEnum) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        for direction in Vector::neighbours() {
            moves.append(&mut self.get_directive_moves(index, direction.clone(), team.clone()));
        }
        moves
    }

    fn r#move(&self, r#move: Move) -> BitBoard {
        let mut new_board: BitBoard = self.clone();
        let origin: HexCoordinate = r#move._from.unwrap();
        let destination: HexCoordinate = r#move.to;
        let origin_index: u64 = origin.to_cartesian().to_index().unwrap();
        let destination_index: u64 = destination.to_cartesian().to_index().unwrap();
        let origin_field: Field = self.get_field(origin_index);
        let destination_field: Field = self.get_field(destination_index);
        new_board.set_field(&origin_field);
        new_board.set_field(&destination_field);

        new_board
    }

    pub fn __repr__(&self) -> String {
        let mut string: String = String::new();
        string.push_str(&format!("\n"));
        for index in 0..64 {
            if self.one & (1 << index) != 0 {
                string.push_str(&format!("□"));
            } else if self.two & (1 << index) != 0 {
                string.push_str(&format!("■"));
            } else if self.fish_0 & (1 << index) != 0 {
                string.push_str(&format!("0"));
            } else if self.fish_1 & (1 << index) != 0 {
                string.push_str(&format!("1"));
            } else if self.fish_2 & (1 << index) != 0 {
                string.push_str(&format!("2"));
            } else if self.fish_3 & (1 << index) != 0 {
                string.push_str(&format!("3"));
            } else if self.fish_4 & (1 << index) != 0 {
                string.push_str(&format!("4"));
            } else {
                string.push_str(&format!("."));
            }
            string.push_str(&format!("  "));
            if index % 8 == 7 {
                string.push_str(&format!("\n"));
            }
        }
        string
    }
}

impl Default for BitBoard {
    fn default() -> Self {
        BitBoard {
            one: 0b0000000000000000000000000000000000000000000000000000000000000000,
            two: 0b0000000000000000000000000000000000000000000000000000000000000000,
            fish_0: 0b0000000000000000000000000000000000000000000000000000000000000000,
            fish_1: 0b0000000000000000000000000000000000000000000000000000000000000000,
            fish_2: 0b0000000000000000000000000000000000000000000000000000000000000000,
            fish_3: 0b0000000000000000000000000000000000000000000000000000000000000000,
            fish_4: 0b0000000000000000000000000000000000000000000000000000000000000000,
        }
    }
}

impl std::fmt::Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.__repr__())
        .map_err(|_e| core::fmt::Error)
    }
}