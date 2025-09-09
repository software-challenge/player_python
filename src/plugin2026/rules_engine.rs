use pyo3::*;

use crate::plugin2026::{
    board::Board, errors::PiranhasError, field_type::FieldType, r#move::Move, utils::{
        constants::PluginConstants,
        coordinate::Coordinate, team::TeamEnum
    }
};

#[pyclass]
pub struct RulesEngine;

#[pymethods]
impl RulesEngine {

    #[staticmethod]
    pub fn move_distance(board: &Board, move_: &Move) -> usize {
        board.get_fish_on_line(&move_.start, &move_.direction).len()
    }

    #[staticmethod]
    pub fn target_position(board: &Board, move_: &Move) -> Coordinate {
        move_.start.clone().add_vector(&move_.direction.to_vector().scale(Self::move_distance(board, move_) as isize))
    }

    #[staticmethod]
    pub fn is_in_bounds(coordinate: &Coordinate) -> bool {
        coordinate.x >= 0 && coordinate.x < PluginConstants::BOARD_WIDTH as isize
        && coordinate.y >= 0 && coordinate.y < PluginConstants::BOARD_HEIGHT as isize
    }

    #[staticmethod]
    pub fn can_execute_move(board: &Board, move_: &Move) -> Result<(), PyErr> {

        let target_pos = Self::target_position(board, move_);

        Self::is_in_bounds(&move_.start)
            .then_some(())
            .ok_or_else(|| PiranhasError::new_err("Start position is out of bounds"))?;

        Self::is_in_bounds(&target_pos)
            .then_some(())
            .ok_or_else(|| PiranhasError::new_err("Target position is out of bounds"))?;

        let start_field = board.get_field(&move_.start)
            .expect("Already validated in-bounds position");
        let target_field = board.get_field(&target_pos)
            .expect("Already validated in-bounds position");

        let this_team = start_field
            .get_team()
            .ok_or_else(|| PiranhasError::new_err("Start position is not on fish field"))?;

        let mut blocked_fields = this_team.get_fish_types();
        blocked_fields.push(FieldType::Squid);

        if blocked_fields.contains(&target_field) {
            return Err(PiranhasError::new_err("Cannot swim onto field of own team or squid"));
        }

        let distance = Self::move_distance(board, move_);
        let direction_fields = board.get_fields_in_direction(&move_.start, &move_.direction);
        let path_fields: Vec<_> = direction_fields.iter().take(distance - 1).cloned().collect(); // not including start or target

        for d in path_fields {
            let mut blocked_fields = this_team.get_fish_types();
            blocked_fields.push(FieldType::Squid);

            if this_team.opponent().get_fish_types().contains(&d) {
                return Err(PiranhasError::new_err("Cannot swim over other team's fish"));
            }
        }

        Ok(())
    }

    #[staticmethod]
    pub fn get_team_on_turn(turn: usize) -> TeamEnum {
        if turn % 2 == 0 {
            TeamEnum::One
        } else {
            TeamEnum::Two
        }
    }
}