use pyo3::prelude::*;

use crate::plugins::penguins::coordinate::{HexCoordinate, CartesianCoordinate};
use crate::plugins::penguins::field::Field;
use crate::plugins::penguins::penguin::Penguin;
use crate::plugins::penguins::r#move::Move;
use crate::plugins::penguins::team::{Team, TeamEnum};
use crate::plugins::penguins::vector::Vector;

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct BitBoard(pub u64);

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct Board {
    #[pyo3(get, set)]
    pub one: BitBoard,
    #[pyo3(get, set)]
    pub two: BitBoard,
    #[pyo3(get, set)]
    pub fish_0: BitBoard,
    #[pyo3(get, set)]
    pub fish_1: BitBoard,
    #[pyo3(get, set)]
    pub fish_2: BitBoard,
    #[pyo3(get, set)]
    pub fish_3: BitBoard,
    #[pyo3(get, set)]
    pub fish_4: BitBoard,
}

#[pymethods]
impl Board {
    #[new]
    pub fn new(board: Vec<Vec<Field>>) -> Self {
        let mut your_penguins = BitBoard(0);
        let mut opponent_penguins = BitBoard(0);
        let mut fish_0 = BitBoard(0);
        let mut fish_1 = BitBoard(0);
        let mut fish_2 = BitBoard(0);
        let mut fish_3 = BitBoard(0);
        let mut fish_4 = BitBoard(0);

        for row in board {
            for field in row {
                if field.penguin.is_some() {
                    if field.penguin.as_ref().unwrap().team.name == TeamEnum::ONE {
                        your_penguins.0 |= 1 << field.coordinate.to_cartesian().to_index();
                    } else if field.penguin.as_ref().unwrap().team.name == TeamEnum::TWO {
                        opponent_penguins.0 |= 1 << field.coordinate.to_cartesian().to_index();
                    } else {
                        panic!("Unknown Type of Team");
                    }
                }
                match field.fish {
                    0 => fish_0.0 |= 1 << field.coordinate.to_cartesian().to_index(),
                    1 => fish_1.0 |= 1 << field.coordinate.to_cartesian().to_index(),
                    2 => fish_2.0 |= 1 << field.coordinate.to_cartesian().to_index(),
                    3 => fish_3.0 |= 1 << field.coordinate.to_cartesian().to_index(),
                    4 => fish_4.0 |= 1 << field.coordinate.to_cartesian().to_index(),
                    _ => panic!("Fish must be between 0 and 4"),
                }
            }
        }

        Board {
            one: your_penguins,
            two: opponent_penguins,
            fish_0,
            fish_1,
            fish_2,
            fish_3,
            fish_4,
        }
    }

    pub fn equivalence(&self, other: &Board) -> bool {
        self.one == other.one
            && self.two == other.two
            && self.fish_0 == other.fish_0
            && self.fish_1 == other.fish_1
            && self.fish_2 == other.fish_2
            && self.fish_3 == other.fish_3
            && self.fish_4 == other.fish_4
    }

    pub fn empty(&self) -> bool {
        self.one.0 == 0
            && self.two.0 == 0
            && self.fish_0.0 == 0
            && self.fish_1.0 == 0
            && self.fish_2.0 == 0
            && self.fish_3.0 == 0
            && self.fish_4.0 == 0
    }

    pub fn intersection(&self, other: &Board) -> Board {
        Board {
            one: BitBoard(self.one.0 & other.one.0),
            two: BitBoard(self.two.0 & other.two.0),
            fish_0: BitBoard(self.fish_0.0 & other.fish_0.0),
            fish_1: BitBoard(self.fish_1.0 & other.fish_1.0),
            fish_2: BitBoard(self.fish_2.0 & other.fish_2.0),
            fish_3: BitBoard(self.fish_3.0 & other.fish_3.0),
            fish_4: BitBoard(self.fish_4.0 & other.fish_4.0),
        }
    }

    pub fn union(&self, other: &Board) -> Board {
        Board {
            one: BitBoard(self.one.0 | other.one.0),
            two: BitBoard(self.two.0 | other.two.0),
            fish_0: BitBoard(self.fish_0.0 | other.fish_0.0),
            fish_1: BitBoard(self.fish_1.0 | other.fish_1.0),
            fish_2: BitBoard(self.fish_2.0 | other.fish_2.0),
            fish_3: BitBoard(self.fish_3.0 | other.fish_3.0),
            fish_4: BitBoard(self.fish_4.0 | other.fish_4.0),
        }
    }

    pub fn difference(&self, other: &Board) -> Board {
        Board {
            one: BitBoard(self.one.0 & !other.one.0),
            two: BitBoard(self.two.0 & !other.two.0),
            fish_0: BitBoard(self.fish_0.0 & !other.fish_0.0),
            fish_1: BitBoard(self.fish_1.0 & !other.fish_1.0),
            fish_2: BitBoard(self.fish_2.0 & !other.fish_2.0),
            fish_3: BitBoard(self.fish_3.0 & !other.fish_3.0),
            fish_4: BitBoard(self.fish_4.0 & !other.fish_4.0),
        }
    }

    pub fn disjoint(&self, other: &Board) -> bool {
        self.intersection(other).empty()
    }

    pub fn complement(&self, other: &Board) -> Board {
        Board {
            one: BitBoard(!self.one.0 & other.one.0),
            two: BitBoard(!self.two.0 & other.two.0),
            fish_0: BitBoard(!self.fish_0.0 & other.fish_0.0),
            fish_1: BitBoard(!self.fish_1.0 & other.fish_1.0),
            fish_2: BitBoard(!self.fish_2.0 & other.fish_2.0),
            fish_3: BitBoard(!self.fish_3.0 & other.fish_3.0),
            fish_4: BitBoard(!self.fish_4.0 & other.fish_4.0),
        }
    }

    pub fn implication(&self, other: &Board) -> Board {
        Board {
            one: BitBoard(!self.one.0 | other.one.0),
            two: BitBoard(!self.two.0 | other.two.0),
            fish_0: BitBoard(!self.fish_0.0 | other.fish_0.0),
            fish_1: BitBoard(!self.fish_1.0 | other.fish_1.0),
            fish_2: BitBoard(!self.fish_2.0 | other.fish_2.0),
            fish_3: BitBoard(!self.fish_3.0 | other.fish_3.0),
            fish_4: BitBoard(!self.fish_4.0 | other.fish_4.0),
        }
    }

    pub fn exclusive_or(&self, other: &Board) -> Board {
        Board {
            one: BitBoard(self.one.0 ^ other.one.0),
            two: BitBoard(self.two.0 ^ other.two.0),
            fish_0: BitBoard(self.fish_0.0 ^ other.fish_0.0),
            fish_1: BitBoard(self.fish_1.0 ^ other.fish_1.0),
            fish_2: BitBoard(self.fish_2.0 ^ other.fish_2.0),
            fish_3: BitBoard(self.fish_3.0 ^ other.fish_3.0),
            fish_4: BitBoard(self.fish_4.0 ^ other.fish_4.0),
        }
    }

    pub fn update(&mut self, move_: &Move) {
        if move_.from != None {
            let from = move_.from.clone().unwrap().to_cartesian().to_index();
            let to = move_.to.to_cartesian().to_index();
            match move_.team.name {
                TeamEnum::ONE => {
                    self.one.0 ^= 1 << from;
                    self.one.0 |= 1 << to;
                }
                TeamEnum::TWO => {
                    self.two.0 ^= 1 << from;
                    self.two.0 |= 1 << to;
                }
            }
        } else {
            let to = move_.to.to_cartesian().to_index();
            match move_.team.name {
                TeamEnum::ONE => {
                    self.one.0 |= 1 << to;
                }
                TeamEnum::TWO => {
                    self.two.0 |= 1 << to;
                }
            }
        }
    }

    pub fn get_fields(&self) -> Vec<Vec<Field>> {
        let mut fields = Vec::new();
        for i in 0..7 {
            let mut row = Vec::new();
            for j in 0..7 {
                let index = i * 5 + j;
                let coordinate: HexCoordinate = CartesianCoordinate::new(i, j).to_hex();
                if self.one.0 & (1 << index) != 0 {
                    let team: Team = Team::new("ONE".to_string());
                    let penguin: Penguin = Penguin::new(coordinate, team);
                    let field: Field = Field::new(coordinate, Some(penguin), 0);
                    row.push(field);
                } else if self.two.0 & (1 << index) != 0 {
                    let team = Team::new("TWO".to_string());
                    let penguin: Penguin = Penguin::new(coordinate, team);
                    let field: Field = Field::new(coordinate, Some(penguin), 0);
                    row.push(field);
                } else {
                    let fish: i32 = self.get_fish(coordinate);
                    let field: Field = Field::new(coordinate, None, fish);
                    row.push(field);
                }
            }
            fields.push(row);
        }
        fields
    }

    fn get_fish(&self, coordinate: HexCoordinate) -> i32 {
        let index = coordinate.to_cartesian().to_index();
        if self.fish_0.0 & (1 << index) != 0 {
            0
        } else if self.fish_1.0 & (1 << index) != 0 {
            1
        } else if self.fish_2.0 & (1 << index) != 0 {
            2
        } else if self.fish_3.0 & (1 << index) != 0 {
            3
        } else if self.fish_4.0 & (1 << index) != 0 {
            4
        } else {
            0
        }
    }

    fn get_empty_fields(&self) -> Vec<Field> {
        let mut empty_fields = Vec::new();
        for i in 0..64 {
            if self.one.0 & (1 << i) == 0 && self.two.0 & (1 << i) == 0
                && self.fish_0.0 & (1 << i) == 1 {
                let coordinate: HexCoordinate = CartesianCoordinate::from_index(i).to_hex();
                empty_fields.push(Field {
                    coordinate,
                    fish: 0,
                    penguin: None,
                });
            }
        }
        empty_fields
    }

    fn is_occupied(&self, coordinate: HexCoordinate) -> bool {
        self.one.0 & (1 << coordinate.to_cartesian().to_index()) != 0
            || self.two.0 & (1 << coordinate.to_cartesian().to_index()) != 0
    }

    pub fn is_valid(&self, coordinate: HexCoordinate) -> bool {
        coordinate.to_cartesian().to_index() < 64
    }

    pub fn get_all_fields(&self) -> Vec<Vec<Field>> {
        let mut board = Vec::new();
        for i in 0..8 {
            let mut row = Vec::new();
            for j in 0..8 {
                let coordinate = HexCoordinate::new(i, j);
                row.push(self.get_field(coordinate));
            }
            board.push(row);
        }
        board
    }


    pub fn get_field(&self, coordinate: HexCoordinate) -> Field {
        if coordinate.to_cartesian().to_index() >= 64 {
            panic!("Coordinate is not valid");
        } else {
            let fish = match coordinate.to_cartesian().to_index() {
                x if self.fish_0.0 & (1 << x) != 0 => 0,
                x if self.fish_1.0 & (1 << x) != 0 => 1,
                x if self.fish_2.0 & (1 << x) != 0 => 2,
                x if self.fish_3.0 & (1 << x) != 0 => 3,
                x if self.fish_4.0 & (1 << x) != 0 => 4,
                _ => panic!("Fish must be between 0 and 4"),
            };
            let penguin: Option<Penguin> = match coordinate.to_cartesian().to_index() {
                x if self.one.0 & (1 << x) != 0 => Some(Penguin {
                    position: coordinate.clone(),
                    team: Team::new("ONE".to_string()),
                }),
                x if self.two.0 & (1 << x) != 0 => Some(Penguin {
                    position: coordinate.clone(),
                    team: Team::new("TWO".to_string()),
                }),
                _ => None,
            };
            Field {
                coordinate,
                fish,
                penguin,
            }
        }
    }

    pub fn contains_field(&self, field: Field) -> bool {
        let coordinate = field.coordinate;
        let fish = field.fish;
        let penguin = field.penguin;
        let mut contains = true;
        if coordinate.to_cartesian().to_index() >= 64 {
            contains = false;
        } else {
            contains = contains && match coordinate.to_cartesian().to_index() {
                x if self.fish_0.0 & (1 << x) != 0 => fish == 0,
                x if self.fish_1.0 & (1 << x) != 0 => fish == 1,
                x if self.fish_2.0 & (1 << x) != 0 => fish == 2,
                x if self.fish_3.0 & (1 << x) != 0 => fish == 3,
                x if self.fish_4.0 & (1 << x) != 0 => fish == 4,
                _ => false,
            };
            contains = contains && match coordinate.to_cartesian().to_index() {
                x if self.one.0 & (1 << x) != 0 => {
                    penguin.is_some() && penguin.as_ref().unwrap().team.name == TeamEnum::ONE
                }
                x if self.two.0 & (1 << x) != 0 => {
                    penguin.is_some() && penguin.as_ref().unwrap().team.name == TeamEnum::TWO
                }
                _ => penguin.is_none(),
            };
        }
        contains
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
        let mut moves = Vec::new();
        let mut current_coordinate = coordinate.clone();
        let mut current_field = self.get_field(current_coordinate.clone());
        while current_field.fish > 0 {
            current_coordinate = current_coordinate.add_vector(&direction);
            current_field = self.get_field(current_coordinate.clone());
            if current_field.fish > 0 {
                moves.push(Move {
                    from: Some(coordinate.clone()),
                    to: current_coordinate.clone(),
                    team: team.clone(),
                });
            }
        }
        moves
    }

    pub fn get_moves_from(&self, coordinate: HexCoordinate, team: Team) -> Vec<Move> {
        let mut moves = Vec::new();
        let directions: Vec<Vector> = coordinate.to_vector().neighbours();
        for direction in directions.iter() {
            moves.append(&mut self.get_directive_moves(coordinate.clone(),
                                                       direction.clone(),
                                                       team.clone()));
        }
        moves
    }

    pub fn get_penguins(&self) -> Vec<Field> {
        let mut penguins = Vec::new();
        for i in 0..64 {
            if self.one.0 & (1 << i) != 0 {
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
            if self.two.0 & (1 << i) != 0 {
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
            if team.name == TeamEnum::ONE && self.one.0 & (1 << i) != 0 {
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
            if team.name == TeamEnum::TWO && self.two.0 & (1 << i) != 0 {
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
        let mut repr = String::new();
        for i in 0..64 {
            if self.one.0 & (1 << i) != 0 {
                repr.push_str("ONE");
            } else if self.two.0 & (1 << i) != 0 {
                repr.push_str("TWO");
            } else if self.fish_0.0 & (1 << i) != 0 {
                repr.push_str("0");
            } else if self.fish_1.0 & (1 << i) != 0 {
                repr.push_str("1");
            } else if self.fish_2.0 & (1 << i) != 0 {
                repr.push_str("2");
            } else if self.fish_3.0 & (1 << i) != 0 {
                repr.push_str("3");
            } else if self.fish_4.0 & (1 << i) != 0 {
                repr.push_str("4");
            } else {
                repr.push_str(" ");
            }
        }
        repr
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut repr = String::new();
        for i in 0..64 {
            if self.one.0 & (1 << i) != 0 {
                repr.push_str("ONE");
            } else if self.two.0 & (1 << i) != 0 {
                repr.push_str("TWO");
            } else if self.fish_0.0 & (1 << i) != 0 {
                repr.push_str("0");
            } else if self.fish_1.0 & (1 << i) != 0 {
                repr.push_str("1");
            } else if self.fish_2.0 & (1 << i) != 0 {
                repr.push_str("2");
            } else if self.fish_3.0 & (1 << i) != 0 {
                repr.push_str("3");
            } else if self.fish_4.0 & (1 << i) != 0 {
                repr.push_str("4");
            } else {
                repr.push_str(" ");
            }
        }
        write!(f, "{}", repr)
    }
}
