use pyo3::prelude::*;

use crate::plugins::penguins::coordinate::{HexCoordinate, CartesianCoordinate};
use crate::plugins::penguins::field::Field;
use crate::plugins::penguins::penguin::Penguin;
use crate::plugins::penguins::r#move::Move;
use crate::plugins::penguins::team::{Team, TeamEnum};
use crate::plugins::penguins::vector::Vector;


#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
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

    pub fn empty(&self) -> bool {
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
        self.intersection(other).empty()
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

    pub fn update(&mut self, move_: &Move) {
        if move_.from != None {
            let from = move_.from.clone().unwrap().to_cartesian().to_index();
            let to = move_.to.to_cartesian().to_index();
            match move_.team.name {
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
            let to = move_.to.to_cartesian().to_index();
            match move_.team.name {
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
            let index: u64 = penguin.position.to_cartesian().to_index();
            match penguin.team.name {
                TeamEnum::ONE => {
                    self.one |= 1 << index;
                }
                TeamEnum::TWO => {
                    self.two |= 1 << index;
                }
            }
        } else {
            let fish: i32 = field.fish.clone();
            let index = field.coordinate.to_cartesian().to_index();
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
                _ => {}
            }
        }
    }

    pub fn get_field(&self, index: u64) -> BitBoard {
        let mut board: BitBoard = self.clone();
        board.one = if (self.one & (1 << index)) != 0 { 1 << index } else { 0 };
        board.two = if (self.two & (1 << index)) != 0 { 1 << index } else { 0 };
        board.fish_0 = if (self.fish_0 & (1 << index)) != 0 { 1 << index } else { 0 };
        board.fish_1 = if (self.fish_1 & (1 << index)) != 0 { 1 << index } else { 0 };
        board.fish_2 = if (self.fish_2 & (1 << index)) != 0 { 1 << index } else { 0 };
        board.fish_3 = if (self.fish_3 & (1 << index)) != 0 { 1 << index } else { 0 };
        board.fish_4 = if (self.fish_4 & (1 << index)) != 0 { 1 << index } else { 0 };
        board
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

    pub fn is_team(&self, team: Team, index: u64) -> bool {
        match team.name {
            TeamEnum::ONE => self.one & (1 << index) != 0,
            TeamEnum::TWO => self.two & (1 << index) != 0,
        }
    }

    pub fn get_coordinates(&self, bitboard: u64) -> Vec<HexCoordinate> {
        let mut coordinates: Vec<HexCoordinate> = Vec::new();
        for i in 0..64 {
            if bitboard & (1 << i) != 0 {
                coordinates.push(CartesianCoordinate::from_index(i).to_hex());
            }
        }
        coordinates
    }

    fn get_bit_coordinate(&self, field: BitBoard) -> Option<HexCoordinate> {
        let mut count: i32 = 0;
        let mut index: u64 = 0;
        for i in 0..63 {
            if field.one & (1 << i) != 0 {
                count += 1;
                index = i;
            }
            if field.two & (1 << i) != 0 {
                count += 1;
                index = i;
            }
            if field.fish_0 & (1 << i) != 0 {
                count += 1;
                index = i;
            }
            if field.fish_1 & (1 << i) != 0 {
                count += 1;
                index = i;
            }
            if field.fish_2 & (1 << i) != 0 {
                count += 1;
                index = i;
            }
            if field.fish_3 & (1 << i) != 0 {
                count += 1;
                index = i;
            }
            if field.fish_4 & (1 << i) != 0 {
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

    pub fn get_directive_moves(&self, index: u64, direction: Vector, team: Team) -> Vec<Move> {
        if self.is_team(team.clone(), index) {
            let mut moves: Vec<Move> = Vec::new();
            let mut current_index: u64 = index;
            let mut current_field: BitBoard = self.get_field(current_index);
            let mut next_index: u64 = current_index;
            let mut next_field: BitBoard = current_field;
            let mut next_fish: i32 = current_field.get_fish(current_index);
            while next_fish != 0 {
                current_index = next_index;
                current_field = next_field;
                next_index = CartesianCoordinate::from_index(current_index).add_vector(&direction).to_index();
                next_field = self.get_field(next_index);
                next_fish = next_field.get_fish(next_index);
                let next_move: Move = Move::new(self.get_bit_coordinate(current_field),
                                                self.get_bit_coordinate(next_field).unwrap(),
                                                team.clone());
                moves.push(next_move);
            }
            return moves;
        }
        Vec::new()
    }

    pub fn get_moves_from(&self, index: u64, team: Team) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let directions: Vec<Vector> = CartesianCoordinate::from_index(index).to_vector().neighbours();
        for direction in directions {
            moves.append(&mut self.get_directive_moves(index, direction.clone(), team.clone()));
        }
        moves
    }

    pub fn __repr__(&self) -> String {
        let mut string: String = String::new();
        for i in 0..63 {
            if self.one & (1 << i) != 0 {
                string.push_str("ONE");
            } else if self.two & (1 << i) != 0 {
                string.push_str("TWO");
            } else if self.fish_0 & (1 << i) != 0 {
                string.push_str("0");
            } else if self.fish_1 & (1 << i) != 0 {
                string.push_str("1");
            } else if self.fish_2 & (1 << i) != 0 {
                string.push_str("2");
            } else if self.fish_3 & (1 << i) != 0 {
                string.push_str("3");
            } else if self.fish_4 & (1 << i) != 0 {
                string.push_str("4");
            } else {
                string.push_str(" ");
            }
            if i % 8 == 7 {
                string.push_str("\n");
            }
        }
        string
    }
}

impl std::fmt::Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.__repr__())
    }
}