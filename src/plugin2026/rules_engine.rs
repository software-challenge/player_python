use std::vec;

use pyo3::*;

use crate::plugin2026::{
    board::Board, errors::PiranhasError, field_type::FieldType, r#move::Move,
    utils::{
        constants::PluginConstants,
        coordinate::Coordinate,
        direction::Direction,
        team::TeamEnum
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

    #[staticmethod]
    pub fn swarm_from(board: &Board, position: &Coordinate) -> Vec<Coordinate> {

        if !RulesEngine::is_in_bounds(position) {
            return vec![];
        }
        
        let Some(this_team) = board.get_field(position).unwrap().get_team() else {
            return vec![];
        };

        let mut todo: Vec<Coordinate> = vec![position.to_owned()];
        let mut visited: Vec<Coordinate> = Vec::new();

        while !todo.is_empty() {

            let neighbors = RulesEngine::valid_neighbors(&todo[0]);
            for n in neighbors {
                if visited.contains(&n) || todo.contains(&n) {
                    continue;
                }

                if let Some(team) = board.get_field(&n).unwrap().get_team() {
                    if team == this_team {
                        todo.push(n)
                    }
                }
            }

            visited.push(todo[0]);
            todo.remove(0);
        }

        visited
    }

    #[staticmethod]
    pub fn swarms_of_team(board: &Board, team: &TeamEnum) -> Vec<Vec<Coordinate>> {

        let mut team_fish: Vec<Coordinate> = Vec::new();
        for f in team.get_fish_types() {
            team_fish.extend(board.get_fields_by_type(f));
        }

        let mut swarms: Vec<Vec<Coordinate>> = Vec::new();
        while !team_fish.is_empty() {
            let visited = RulesEngine::swarm_from(board, &team_fish[0]);

            for v in &visited {
                if let Some(index) = team_fish.iter().position(|x| x == v) {
                    team_fish.remove(index);
                }
            }

            swarms.push(visited)
        }

        swarms
    }
}

// rust exclusive methods
impl RulesEngine {
    pub fn valid_neighbors(position: &Coordinate) -> Vec<Coordinate> {

        let mut coordinates: Vec<Coordinate> = Vec::new();

        for d in Direction::all_directions() {
            let neighbor = position.add_vector(&d.to_vector());
            if RulesEngine::is_in_bounds(&neighbor) {
                coordinates.push(neighbor);
            }
        }
        
        coordinates
    }
}