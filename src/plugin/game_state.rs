use std::cmp::Ordering;
use std::mem::swap;

use log::{ debug, info };
use pyo3::exceptions::PyBaseException;
use pyo3::prelude::*;

use crate::plugin::actions::accelerate::Accelerate;
use crate::plugin::actions::advance::Advance;
use crate::plugin::actions::push::Push;
use crate::plugin::actions::turn::Turn;
use crate::plugin::actions::Action;
use crate::plugin::board::Board;
use crate::plugin::constants::PluginConstants;
use crate::plugin::coordinate::{ CubeCoordinates, CubeDirection };
use crate::plugin::errors::advance_errors::AdvanceProblem;
use crate::plugin::errors::movement_error::MoveMistake;
use crate::plugin::field::FieldType;
use crate::plugin::r#move::Move;
use crate::plugin::ship::Ship;

use super::field::{ Field, Passenger };
use super::ship::TeamEnum;

#[pyclass]
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TeamPoints {
    #[pyo3(get, set)]
    pub ship_points: i32,
    #[pyo3(get, set)]
    pub coal_points: i32,
    #[pyo3(get, set)]
    pub finish_points: i32,
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct AdvanceInfo {
    #[pyo3(get, set)]
    pub costs: Vec<i32>,
    #[pyo3(get, set)]
    pub problem: AdvanceProblem,
}

#[pymethods]
impl AdvanceInfo {
    #[must_use]
    pub fn cost_until(&self, distance: usize) -> i32 {
        let cost: i32 = self.costs[distance - 1];
        cost
    }

    #[must_use]
    pub fn advances(&self, distance: Option<usize>) -> Vec<Advance> {
        let distance = distance.unwrap_or(self.costs.len());
        (1..=distance)
            .map(|it| Advance {
                distance: it as i32,
            })
            .collect()
    }

    #[must_use]
    pub fn distance(&self) -> usize {
        self.costs.len()
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("AdvanceInfo(costs={:?}, problem={:?})", self.costs, self.problem))
    }
}

#[cfg(test)]
mod advance_info_tests {
    use super::*;

    #[test]
    fn test_cost_until() {
        let advance_info: AdvanceInfo = AdvanceInfo {
            costs: vec![1, 2, 3, 4, 5],
            problem: AdvanceProblem::FieldIsBlocked,
        };
        assert_eq!(advance_info.cost_until(3), 3);
    }

    #[test]
    fn test_advances() {
        let advance_info: AdvanceInfo = AdvanceInfo {
            costs: vec![1, 2, 3, 4, 5],
            problem: AdvanceProblem::FieldIsBlocked,
        };
        assert_eq!(
            advance_info.advances(None),
            vec![
                Advance { distance: 1 },
                Advance { distance: 2 },
                Advance { distance: 3 },
                Advance { distance: 4 },
                Advance { distance: 5 }
            ]
        );
    }

    #[test]
    fn test_distance() {
        let advance_info: AdvanceInfo = AdvanceInfo {
            costs: vec![1, 2, 3, 4, 5],
            problem: AdvanceProblem::FieldIsBlocked,
        };
        assert_eq!(advance_info.distance(), 5);
    }
}

/// # `GameState`
///
/// A `GameState` contains all information required to describe the game state at any given time, that is, in between two game moves.
///
/// This includes:
/// - `board`: The current board configuration.
/// - `turn`: The consecutive turn number, indicating which team's turn it is.
/// - `team_one` & `team_two`: Information about the teams, including details of their ships.
/// - `last_move`: The last move that was made in the game.
///
/// The GameState object provides essential information about the ongoing game and offers various helper methods for managing the game flow. These methods include:
///
/// - `new` : Method to create and initialize a new game state.
/// - `current_ship` and `other_ship` : Methods to fetch details about the current and the opposing team's ships.
/// - `determine_ahead_team` : A method to determine the leading team.
/// - `ship_advance_points`, `calculate_points` : Methods to calculate points based on various parameters.
/// - `is_current_ship_on_current`, `can_move`, `is_over`, `is_winner` : Methods to get game state.
/// - `perform_move` : A method to perform game moves.
/// - `advance_turn` : Lets the game advance to the next turn.
/// - `get_simple_moves`, `get_actions`, `get_accelerations` : Methods to fetch possible moves and actions.
/// - `check_sandbank_advances`, `check_advance_limit` : Methods to validate possible moves and actions.
/// - `get_pushes`, `get_pushes_from`, `get_turns`, `get_advances` : Methods to generate valid game actions.
/// - `must_push` : Method to check if a push action is needed.
/// - `get_points_for_team` : A method to calculate total points for a team.
///
/// The game server sends a copy of the GameState object to both participating teams after every completed move, providing an updated snapshot of the current game state.
#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct GameState {
    #[pyo3(get, set)]
    pub board: Board,
    #[pyo3(get, set)]
    pub turn: i32,
    #[pyo3(get, set)]
    pub current_ship: Ship,
    #[pyo3(get, set)]
    pub other_ship: Ship,
    #[pyo3(get, set)]
    pub last_move: Option<Move>,
}

#[pymethods]
impl GameState {
    #[new]
    #[must_use]
    pub fn new(
        board: Board,
        turn: i32,
        current_ship: Ship,
        other_ship: Ship,
        last_move: Option<Move>
    ) -> Self {
        Self {
            board,
            turn,
            current_ship,
            other_ship,
            last_move,
        }
    }

    #[must_use]
    pub fn determine_ahead_team(&self) -> Ship {
        let calculate_points = |ship: Ship| -> i32 {
            self.ship_advance_points(ship).unwrap() * 100 + ship.speed * 10 + ship.coal
        };

        let current_points = calculate_points(self.current_ship);
        let other_points = calculate_points(self.other_ship);

        match (current_points.cmp(&other_points), &self.current_ship.team) {
            (Ordering::Greater, _) => self.current_ship,
            (Ordering::Less, _) => self.other_ship,
            (_, TeamEnum::One) => self.current_ship,
            _ => self.other_ship,
        }
    }

    #[must_use]
    pub fn is_current_ship_on_current(&self) -> bool {
        self.board.does_field_have_stream(&self.current_ship.position)
    }

    pub fn perform_action(&self, action: Action) -> Result<Self, PyErr> {
        let mut new_state = self.clone();

        match action {
            Action::Accelerate(accelerate) =>
                match accelerate.perform(&new_state) {
                    Ok(updated_ship) => {
                        new_state.current_ship = updated_ship;
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            Action::Advance(advance) =>
                match advance.perform(&new_state) {
                    Ok(updated_ship) => {
                        new_state.current_ship = updated_ship;
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            Action::Turn(turn) =>
                match turn.perform(&new_state) {
                    Ok(updated_ship) => {
                        new_state.current_ship = updated_ship;
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            Action::Push(push) =>
                match push.perform(&new_state) {
                    Ok((updated_current_ship, updated_other_ship)) => {
                        new_state.current_ship = updated_current_ship;
                        new_state.other_ship = updated_other_ship;
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
        }

        Ok(new_state)
    }

    fn move_pre_check(&self, action: Action, action_idx: usize, ship: Ship) -> Result<(), PyErr> {
        match action {
            Action::Push(_) if !self.must_push() => {
                return Err(PyBaseException::new_err(MoveMistake::PushActionRequired.message()));
            }
            Action::Accelerate(_) if action_idx != 0 => {
                return Err(PyBaseException::new_err(MoveMistake::FirstActionAccelerate.message()));
            }
            Action::Advance(ad) if
                self.board
                    .get(&(ship.position + ship.direction.vector() * ad.distance))
                    .map_or(false, |f| f.field_type == FieldType::Sandbank)
            => {
                return Err(PyBaseException::new_err(MoveMistake::SandBankEnd.message()));
            }
            _ => {}
        }

        Ok(())
    }

    fn move_after_check(&self, ship: &Ship) -> Result<(), PyErr> {
        if ship.movement != 0 {
            return Err(
                PyBaseException::new_err(
                    if ship.movement > 0 {
                        MoveMistake::MovementPointsLeft.message()
                    } else {
                        MoveMistake::MovementPointsMissing.message()
                    }
                )
            );
        }

        Ok(())
    }

    pub fn perform_move(&self, move_: Move) -> Result<Self, PyErr> {
        debug!("Current ship before move: {:?}", self.current_ship);
        debug!("Other ship before move: {:?}", self.other_ship);

        if move_.actions.is_empty() {
            return Err(PyBaseException::new_err(MoveMistake::NoActions.message()));
        }

        let mut new_state = self.clone();
        debug!("Actions: {:?}", move_.actions);

        for (i, action) in move_.actions.iter().enumerate() {
            new_state.move_pre_check(*action, i, self.current_ship)?;
            match new_state.perform_action(*action) {
                Ok(state) => {
                    new_state = state;
                }
                Err(e) => {
                    return Err(PyBaseException::new_err(e));
                }
            }
        }

        new_state.move_after_check(&new_state.current_ship)?;

        new_state.pick_up_passenger_current_ship();

        if move_.actions.iter().any(|a| matches!(a, Action::Push(_))) {
            new_state.pick_up_passenger_other_ship();
        }

        new_state.last_move = Some(move_);
        new_state.advance_turn();

        debug!("Current ship after move: {:?}", new_state.current_ship);
        debug!("Other ship after move: {:?}", new_state.other_ship);

        Ok(new_state)
    }

    #[allow(clippy::nonminimal_bool)]
    pub fn advance_turn(&mut self) {
        self.current_ship.free_acc = 1;
        self.current_ship.free_turns = 1;
        self.current_ship.movement = self.current_ship.speed;

        self.turn += 1;

        if
            (self.turn % 2 == 0 && self.determine_ahead_team() != self.current_ship) ||
            self.turn % 2 != 0
        {
            swap(&mut self.current_ship, &mut self.other_ship);
        }

        if !self.can_move() && !self.is_over() {
            self.last_move = None;
            self.advance_turn();
        }
    }

    #[must_use]
    pub fn effective_speed(&self, ship: &Ship) -> i32 {
        ship.speed - i32::from(self.board.does_field_have_stream(&ship.position))
    }

    pub fn remove_passenger_at(&mut self, coord: CubeCoordinates) -> bool {
        for &d in &CubeDirection::VALUES {
            if let Some(field) = self.board.get_field_in_direction(&d, &coord) {
                if let Some(passenger) = field.passenger {
                    if passenger.passenger > 0 && passenger.direction == d.opposite() {
                        let updated_passenger = Passenger {
                            passenger: passenger.passenger - 1,
                            direction: passenger.direction,
                        };
                        self.board.set_field_in_direction(
                            &d,
                            &coord,
                            Field::new(field.field_type, Some(updated_passenger))
                        );
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn pick_up_passenger_current_ship(&mut self) {
        if self.effective_speed(&self.current_ship) < 2 {
            let has_removed = self.remove_passenger_at(self.current_ship.position);
            if has_removed {
                self.current_ship.passengers += 1;
                self.current_ship.points = self
                    .ship_points(self.other_ship)
                    .expect("Could not calculate other ship's points");
            }
        }
    }

    pub fn pick_up_passenger_other_ship(&mut self) {
        if self.effective_speed(&self.other_ship) < 2 {
            let has_removed = self.remove_passenger_at(self.other_ship.position);
            if has_removed {
                self.other_ship.passengers += 1;
                self.other_ship.points = self
                    .ship_points(self.other_ship)
                    .expect("Could not calculate other ship's points");
            }
        }
    }

    #[must_use]
    pub fn ship_advance_points(&self, ship: Ship) -> Option<i32> {
        let (i, segment) = self.board.segment_with_index_at(ship.position)?;
        Some(
            (i as i32) * PluginConstants::POINTS_PER_SEGMENT +
                segment.global_to_local(ship.position).array_x() +
                1
        )
    }

    #[must_use]
    pub fn ship_points(&self, ship: Ship) -> Option<i32> {
        Some(
            self.ship_advance_points(ship)? +
                ship.passengers * PluginConstants::POINTS_PER_PASSENGER
        )
    }

    #[must_use]
    pub fn must_push(&self) -> bool {
        self.current_ship.position == self.other_ship.position
    }

    #[must_use]
    pub fn check_ship_advance_limit(&self, ship: &Ship) -> AdvanceInfo {
        self.calculate_advance_info(&ship.position, &ship.direction, ship.movement)
    }

    #[must_use]
    pub fn calculate_advance_info(
        &self,
        start: &CubeCoordinates,
        direction: &CubeDirection,
        max_movement_points: i32
    ) -> AdvanceInfo {
        let max_movement = max_movement_points.clamp(0, PluginConstants::MAX_SPEED);
        let mut current_position = *start;
        let mut total_cost = 0;
        let mut costs: Vec<i32> = Vec::new();
        let mut has_current = false;

        while total_cost < max_movement {
            current_position += direction.vector();
            total_cost += 1;

            match self.board.get(&current_position) {
                Some(field) if field.is_empty() => {
                    if self.board.does_field_have_stream(&current_position) && !has_current {
                        has_current = true;
                        if total_cost < max_movement {
                            total_cost += 1;
                        } else {
                            break;
                        }
                    }

                    if
                        self.current_ship.position == current_position ||
                        self.other_ship.position == current_position
                    {
                        if total_cost < max_movement {
                            costs.push(total_cost);
                            return AdvanceInfo {
                                costs,
                                problem: AdvanceProblem::ShipAlreadyInTarget,
                            };
                        }
                        return AdvanceInfo {
                            costs,
                            problem: AdvanceProblem::InsufficientPush,
                        };
                    }

                    if field.field_type == FieldType::Sandbank {
                        return AdvanceInfo {
                            costs,
                            problem: AdvanceProblem::MoveEndOnSandbank,
                        };
                    }
                    costs.push(total_cost);
                }
                _ => {
                    return AdvanceInfo {
                        costs,
                        problem: AdvanceProblem::FieldIsBlocked,
                    };
                }
            }
        }

        AdvanceInfo {
            costs,
            problem: AdvanceProblem::MovementPointsMissing,
        }
    }

    fn merge_consecutive_advances(&self, actions: Vec<Action>) -> Vec<Action> {
        let mut merged_actions = vec![];
        let mut iter = actions.into_iter().peekable();

        while let Some(action) = iter.next() {
            match action {
                Action::Advance(advance) => {
                    let mut total_distance = advance.distance;
                    while matches!(iter.peek(), Some(Action::Advance(_))) {
                        if let Some(Action::Advance(a)) = iter.next() {
                            total_distance += a.distance;
                        }
                    }
                    merged_actions.push(
                        Action::Advance(Advance {
                            distance: total_distance,
                        })
                    );
                }
                _ => merged_actions.push(action),
            }
        }

        merged_actions
    }

    #[must_use]
    pub fn possible_moves(&self, depth: Option<usize>) -> Vec<Move> {
        self.possible_action_comb(self, vec![], 0, depth.unwrap_or(PluginConstants::MAX_DEPTH))
            .into_iter()
            .map(|actions| Move { actions })
            .collect()
    }

    #[allow(clippy::only_used_in_recursion)]
    #[must_use]
    pub fn possible_action_comb(
        &self,
        current_state: &Self,
        current_actions: Vec<Action>,
        depth: usize,
        max_depth: usize
    ) -> Vec<Vec<Action>> {
        if depth > max_depth || (!current_state.can_move() && !current_state.must_push()) {
            return current_state
                .move_after_check(&current_state.current_ship)
                .map_or(vec![], |()| {
                    vec![current_state.merge_consecutive_advances(current_actions)]
                });
        }

        current_state
            .possible_actions(depth, None)
            .iter()
            .filter_map(|&action| {
                current_state
                    .perform_action(action)
                    .ok()
                    .map(|new_state| {
                        let mut new_actions = current_actions.clone();
                        new_actions.push(action);
                        self.possible_action_comb(&new_state, new_actions, depth + 1, max_depth)
                    })
            })
            .flatten()
            .collect()
    }

    #[must_use]
    pub fn possible_accelerations(&self, max_coal: Option<usize>) -> Vec<Accelerate> {
        if self.must_push() {
            return Vec::new();
        }

        let ship = self.current_ship;
        let max_coal = max_coal.unwrap_or_else(|| ship.coal.try_into().unwrap());
        let max_possible_acceleration = max_coal + (ship.free_acc as usize);

        (1..=max_possible_acceleration as i32)
            .flat_map(|i| {
                let positive = if PluginConstants::MAX_SPEED >= ship.speed + i {
                    Some(Accelerate { acc: i })
                } else {
                    None
                };
                let negative = if PluginConstants::MIN_SPEED <= ship.speed - i {
                    Some(Accelerate { acc: -i })
                } else {
                    None
                };
                positive.into_iter().chain(negative)
            })
            .collect()
    }

    #[must_use]
    pub fn possible_pushes(&self) -> Vec<Push> {
        if
            self.board.get_field_in_direction(
                &self.current_ship.direction,
                &self.current_ship.position
            ) == Some(Field::new(FieldType::Sandbank, None)) ||
            !self.must_push() ||
            self.current_ship.movement < 1
        {
            return Vec::new();
        }

        CubeDirection::VALUES.into_iter()
            .filter(|&dir| {
                dir != self.current_ship.direction.opposite() &&
                    self.board
                        .get_field_in_direction(&dir, &self.current_ship.position)
                        .map_or(false, |f| f.is_empty())
            })
            .map(|dir| Push { direction: dir })
            .collect()
    }

    #[must_use]
    pub fn possible_turns(&self, max_coal: Option<usize>) -> Vec<Turn> {
        if
            self.board.get(&self.current_ship.position) ==
                Some(Field::new(FieldType::Sandbank, None)) ||
            self.must_push()
        {
            return Vec::new();
        }

        let max_coal = max_coal.unwrap_or(self.current_ship.coal as usize);
        let max_turn_count = std::cmp::min(max_coal + (self.current_ship.free_turns as usize), 3);

        (1..=max_turn_count)
            .flat_map(|i| {
                vec![
                    Turn::new(self.current_ship.direction.rotated_by(i as i32)),
                    Turn::new(self.current_ship.direction.rotated_by(-(i as i32)))
                ]
            })
            .take(5)
            .collect()
    }

    #[must_use]
    pub fn possible_advances(&self) -> Vec<Advance> {
        if self.current_ship.movement < 1 || self.must_push() {
            return Vec::new();
        }

        self.check_ship_advance_limit(&self.current_ship).advances(None)
    }

    #[allow(unused_variables)]
    #[must_use]
    pub fn sandbank_advances_for(&self, ship: &Ship) -> Option<Vec<Advance>> {
        panic!(
            "Sandbanks will not be included in the Software-Challenge 2024. 
        Consequently, this particular method will not be implemented for the duration of this season."
        );
    }

    pub fn possible_actions(&self, rank: usize, max_coal: Option<usize>) -> Vec<Action> {
        let max_coal = max_coal.unwrap_or_else(|| self.current_ship.coal.try_into().unwrap());
        let mut actions: Vec<Action> = Vec::new();

        if rank == 0 {
            actions.extend(
                self.possible_accelerations(Some(max_coal)).into_iter().map(Action::Accelerate)
            );
        }
        actions.extend(self.possible_turns(Some(max_coal)).into_iter().map(Action::Turn));
        actions.extend(self.possible_advances().into_iter().map(Action::Advance));
        if rank != 0 {
            actions.extend(self.possible_pushes().into_iter().map(Action::Push));
        }

        actions
    }

    #[must_use]
    pub fn coal_for_action(&self, action: Action) -> usize {
        match action {
            Action::Accelerate(acc) => {
                (acc.acc.unsigned_abs() as usize) - (self.current_ship.free_acc as usize)
            }
            Action::Turn(dir) => {
                let turn_count: i32 = self.current_ship.direction.turn_count_to(dir.direction);
                (turn_count.unsigned_abs() as usize) - (self.current_ship.free_turns as usize)
            }
            Action::Push(_) | Action::Advance(_) => 0,
        }
    }

    #[must_use]
    pub fn can_move(&self) -> bool {
        let current_ship_can_advance: bool = !self.possible_advances().is_empty();

        let current_ship_can_turn: bool = !self.possible_turns(None).is_empty();

        let current_ship_can_accelerate: bool = !self.possible_accelerations(None).is_empty();

        current_ship_can_advance || current_ship_can_turn || current_ship_can_accelerate
    }

    #[must_use]
    pub fn is_over(&self) -> bool {
        // Bedingung 1: ein Dampfer mit 2 Passagieren erreicht ein Zielfeld mit Geschwindigkeit 1
        let condition1 =
            self.turn % 2 == 0 &&
            (self.is_winner(&self.current_ship) || self.is_winner(&self.other_ship));

        // Bedingung 2: ein Spieler macht einen ungültigen Zug.
        // Das wird durch eine InvalidMoveException während des Spiels behandelt.

        // Bedingung 3: am Ende einer Runde liegt ein Dampfer mehr als 3 Spielsegmente zurück
        let condition3 =
            self.board
                .segment_distance(&self.current_ship.position, &self.other_ship.position)
                .abs() > 3;

        // Bedingung 4: das Rundenlimit von 30 Runden ist erreicht
        let condition4 = self.turn / 2 >= PluginConstants::ROUND_LIMIT;

        // Bedingung 5: beide Spieler können sich nicht mehr bewegen
        let condition5 = self.last_move.is_none() && !self.can_move();

        condition1 || condition3 || condition4 || condition5
    }

    #[must_use]
    pub fn is_winner(&self, ship: &Ship) -> bool {
        ship.passengers > 1 &&
            self.board.effective_speed(ship) < 2 &&
            self.board
                .get(&ship.position)
                .unwrap_or_else(|| {
                    panic!("[is_winner] Field at position {} does not exist", ship.position)
                }).field_type == FieldType::Goal
    }

    #[must_use]
    pub fn get_points_for_team(&self, ship: &Ship) -> TeamPoints {
        let finish_points = PluginConstants::FINISH_POINTS * i32::from(self.is_winner(ship));
        TeamPoints {
            ship_points: ship.points,
            coal_points: ship.coal * 2,
            finish_points,
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(
            format!(
                "GameState(board={:?} segments, turn={}, team_one={:?}, team_two={:?}, last_move={:?})",
                self.board.segments.len(),
                self.turn,
                self.current_ship,
                self.other_ship,
                self.last_move
            )
        )
    }
}
