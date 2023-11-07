use pyo3::{ prelude::*, exceptions::PyBaseException };

use crate::plugin::{
    coordinate::CubeDirection,
    game_state::GameState,
    errors::turn_error::TurnProblem,
    field::FieldType,
    ship::Ship,
};

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash, Copy)]
pub struct Turn {
    #[pyo3(get, set)]
    pub direction: CubeDirection,
}

#[pymethods]
impl Turn {
    #[new]
    pub fn new(direction: CubeDirection) -> Self {
        Turn { direction }
    }

    pub fn perform(&self, state: &GameState) -> Result<Ship, PyErr> {
        let mut current_ship: Ship = state.current_ship.clone();

        let turn_count: i32 = current_ship.direction.turn_count_to(self.direction.clone());

        let abs_turn_count: i32 = turn_count.abs();
        let used_coal: i32 = abs_turn_count - current_ship.free_turns;

        current_ship.free_turns = std::cmp::max(current_ship.free_turns - abs_turn_count, 0);

        if state.board.get(&current_ship.position).unwrap().field_type == FieldType::Sandbank {
            return Err(
                PyBaseException::new_err(TurnProblem::RotationOnSandbankNotAllowed.message())
            );
        }
        if current_ship.coal < used_coal {
            return Err(PyBaseException::new_err(TurnProblem::NotEnoughCoalForRotation.message()));
        }

        if used_coal > 0 {
            current_ship.coal -= used_coal;
        }

        current_ship.direction = self.direction.clone();

        Ok(current_ship)
    }

    pub fn coal_cost(&self, ship: &Ship) -> i32 {
        self.direction.turn_count_to(self.direction.clone()).abs().saturating_sub(ship.free_turns)
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Turn({})", self.direction))
    }
}

#[cfg(test)]
mod tests {
    use crate::plugin::board::Board;
    use crate::plugin::coordinate::{ CubeCoordinates, CubeDirection };
    use crate::plugin::field::{ Field, FieldType };
    use crate::plugin::game_state::GameState;
    use crate::plugin::segment::Segment;
    use crate::plugin::ship::{ Ship, TeamEnum };

    use super::*;

    #[test]
    fn test_turn_new() {
        let direction = CubeDirection::Right;
        let turn = Turn::new(direction);
        assert_eq!(turn.direction, direction);
    }

    fn setup(coal: i32) -> GameState {
        let segment: Vec<Segment> = vec![Segment {
            direction: CubeDirection::Right,
            center: CubeCoordinates::new(0, 0),
            fields: vec![vec![Field::new(FieldType::Water, None); 4]; 5],
        }];
        let board: Board = Board::new(segment, CubeDirection::Right);
        let team_one: &mut Ship = &mut Ship::new(
            CubeCoordinates::new(0, 0),
            TeamEnum::One,
            None,
            None,
            None,
            None,
            None,
            None,
            None);
        team_one.speed = 5;
        team_one.movement = 5;
        team_one.coal = coal;
        let team_two: &mut Ship = &mut Ship::new(
            CubeCoordinates::new(-1, 0),
            TeamEnum::Two,
            None,
            None,
            None,
            None,
            None,
            None,
            None);
        team_two.speed = 5;
        team_two.movement = 5;
        team_two.coal = coal;
        let game_state: GameState = GameState::new(
            board,
            0,
            team_one.clone(),
            team_two.clone(), 
            None);
        game_state
    }

    #[test]
    fn test_turn_perform() {
        let state: GameState = setup(5);
        let turn: Turn = Turn::new(CubeDirection::Left);
        let result: Result<Ship, PyErr> = turn.perform(&state);

        assert!(result.is_ok());

        let new_ship: Ship = result.unwrap();
        assert_eq!(new_ship.direction, CubeDirection::Left);
    }

    #[test]
    fn test_turn_perform_not_enough_coal() {
        let state: GameState = setup(0);
        let turn: Turn = Turn::new(CubeDirection::Left);
        let result: Result<Ship, PyErr> = turn.perform(&state);

        assert!(result.is_err());

        Python::with_gil(|py| {
            assert_eq!(
                result.unwrap_err().value(py).to_string(),
                TurnProblem::NotEnoughCoalForRotation.message()
            );
        });
    }
}
