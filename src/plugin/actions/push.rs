use pyo3::exceptions::PyBaseException;
use pyo3::prelude::*;

use crate::plugin::coordinate::{ CubeCoordinates, CubeDirection };
use crate::plugin::errors::push_error::PushProblem;
use crate::plugin::field::FieldType;
use crate::plugin::game_state::GameState;
use crate::plugin::ship::Ship;

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash, Copy)]
pub struct Push {
    #[pyo3(get, set)]
    pub direction: CubeDirection,
}

#[pymethods]
impl Push {
    #[new]
    pub fn new(direction: CubeDirection) -> Self {
        Push { direction }
    }

    pub fn perform(&self, state: &GameState) -> Result<Ship, PyErr> {
        let current_ship: Ship = state.current_ship.clone();
        let mut other_ship: Ship = state.other_ship.clone();

        if current_ship.movement == 0 {
            return Err(PyBaseException::new_err(PushProblem::MovementPointsMissing.message()));
        }

        let push_from: CubeCoordinates = current_ship.position;
        let push_to: CubeCoordinates = push_from + self.direction.vector();

        let shift_to_field = match state.board.get(&push_to) {
            Some(value) => value,
            None => {
                return Err(PyBaseException::new_err(PushProblem::InvalidFieldPush.message()));
            }
        };

        if !shift_to_field.is_empty() {
            return Err(PyBaseException::new_err(PushProblem::BlockedFieldPush.message()));
        }

        if push_from != other_ship.position {
            return Err(PyBaseException::new_err(PushProblem::SameFieldPush.message()));
        }

        if state.board.get(&push_from).unwrap().field_type == FieldType::Sandbank {
            return Err(PyBaseException::new_err(PushProblem::SandbankPush.message()));
        }

        if self.direction == current_ship.direction.opposite() {
            return Err(PyBaseException::new_err(PushProblem::BackwardPushingRestricted.message()));
        }

        if shift_to_field.field_type == FieldType::Sandbank {
            other_ship.speed = 1;
            other_ship.movement = 1;
        }

        other_ship.position = push_to;
        other_ship.free_turns += 1;

        Ok(other_ship)
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Push({})", self.direction))
    }
}

#[cfg(test)]
mod tests {
    use crate::plugin::board::Board;
    use crate::plugin::coordinate::{ CubeCoordinates, CubeDirection };
    use crate::plugin::field::Field;
    use crate::plugin::game_state::GameState;
    use crate::plugin::segment::Segment;
    use crate::plugin::ship::{ Ship, TeamEnum };

    use super::*;

    fn setup(
        c1: CubeCoordinates,
        c2: CubeCoordinates,
        fields: Vec<Vec<Field>>,
        dir: CubeDirection
    ) -> (GameState, Push) {
        let segment: Vec<Segment> = vec![Segment {
            direction: CubeDirection::Right,
            center: CubeCoordinates::new(0, 0),
            fields,
        }];
        let board: Board = Board::new(segment, CubeDirection::Right);
        let mut team_one: Ship = Ship::new(
            c1,
            TeamEnum::One,
            None,
            None,
            None,
            None,
            None,
            None,
            None
        );
        team_one.speed = 5;
        team_one.movement = 5;
        let mut team_two: Ship = Ship::new(
            c2,
            TeamEnum::Two,
            None,
            None,
            None,
            None,
            None,
            None,
            None
        );
        team_two.speed = 5;
        team_two.movement = 5;
        let state: GameState = GameState::new(board, 0, team_one, team_two, None);
        let push: Push = Push::new(dir);
        (state, push)
    }

    #[test]
    fn test_push_perform() {
        let (state, push) = setup(
            CubeCoordinates::new(0, 0),
            CubeCoordinates::new(0, 0),
            vec![vec![Field::new(FieldType::Water, None); 4]; 5],
            CubeDirection::Right
        );
        let result: Result<Ship, PyErr> = push.perform(&state);

        assert!(result.is_ok());

        let new_ship: Ship = result.unwrap();

        assert_eq!(new_ship.position, CubeCoordinates::new(1, 0));
        assert_eq!(new_ship.free_turns, 1);
    }

    #[test]
    fn test_push_perform_blocked_field() {
        let fields: Vec<Vec<Field>> = vec![
            vec![
                Field::new(FieldType::Water, None),
                Field::new(FieldType::Water, None),
                Field::new(FieldType::Water, None),
                Field::new(FieldType::Water, None)
            ],
            vec![
                Field::new(FieldType::Water, None),
                Field::new(FieldType::Water, None),
                Field::new(FieldType::Water, None),
                Field::new(FieldType::Water, None)
            ],
            vec![
                Field::new(FieldType::Water, None),
                Field::new(FieldType::Water, None),
                Field::new(FieldType::Island, None),
                Field::new(FieldType::Water, None)
            ],
            vec![
                Field::new(FieldType::Water, None),
                Field::new(FieldType::Water, None),
                Field::new(FieldType::Water, None),
                Field::new(FieldType::Water, None)
            ],
            vec![
                Field::new(FieldType::Water, None),
                Field::new(FieldType::Water, None),
                Field::new(FieldType::Water, None),
                Field::new(FieldType::Water, None)
            ]
        ];

        let (state, push) = setup(
            CubeCoordinates::new(0, 0),
            CubeCoordinates::new(0, 0),
            fields,
            CubeDirection::Right
        );
        let result: Result<Ship, PyErr> = push.perform(&state);

        assert!(result.is_err());

        Python::with_gil(|py| {
            assert_eq!(
                result.unwrap_err().value(py).to_string(),
                PushProblem::BlockedFieldPush.message()
            );
        });
    }

    #[test]
    fn test_push_perform_same_field() {
        let (state, push) = setup(
            CubeCoordinates::new(0, 0),
            CubeCoordinates::new(1, 0),
            vec![vec![Field::new(FieldType::Water, None); 4]; 5],
            CubeDirection::Right
        );
        let result: Result<Ship, PyErr> = push.perform(&state);

        assert!(result.is_err());

        Python::with_gil(|py| {
            assert_eq!(
                result.unwrap_err().value(py).to_string(),
                PushProblem::SameFieldPush.message()
            );
        });
    }

    #[test]
    fn test_push_perform_backward_pushing_restricted() {
        let (state, push) = setup(
            CubeCoordinates::new(0, 0),
            CubeCoordinates::new(0, 0),
            vec![vec![Field::new(FieldType::Water, None); 4]; 5],
            CubeDirection::Left
        );
        let result: Result<Ship, PyErr> = push.perform(&state);

        assert!(result.is_err());

        Python::with_gil(|py| {
            assert_eq!(
                result.unwrap_err().value(py).to_string(),
                PushProblem::BackwardPushingRestricted.message()
            );
        });
    }
}
