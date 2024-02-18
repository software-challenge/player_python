use std::cmp::Ordering;
use std::mem::swap;

use log::debug;
use pyo3::exceptions::PyBaseException;
use pyo3::prelude::*;

use crate::plugin::actions::accelerate::Accelerate;
use crate::plugin::actions::Action;
use crate::plugin::actions::advance::Advance;
use crate::plugin::actions::push::Push;
use crate::plugin::actions::turn::Turn;
use crate::plugin::board::Board;
use crate::plugin::constants::PluginConstants;
use crate::plugin::coordinate::{ CubeCoordinates, CubeDirection };
use crate::plugin::errors::movement_error::MoveMistake;
use crate::plugin::field::FieldType;
use crate::plugin::r#move::Move;
use crate::plugin::ship::Ship;
use crate::plugin::errors::advance_errors::AdvanceProblem;

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
    pub fn cost_until(&self, distance: usize) -> i32 {
        let cost: i32 = self.costs[distance - 1];
        cost
    }

    pub fn advances(&self, distance: Option<usize>) -> Vec<Advance> {
        let distance = distance.unwrap_or(self.costs.len());
        (1..=distance).map(|it| Advance { distance: it as i32 }).collect()
    }

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
    pub fn new(
        board: Board,
        turn: i32,
        current_ship: Ship,
        other_ship: Ship,
        last_move: Option<Move>
    ) -> GameState {
        GameState {
            board,
            turn,
            current_ship,
            other_ship,
            last_move,
        }
    }

    pub fn determine_ahead_team(&self) -> Ship {
        let calculate_points = |ship: Ship| -> i32 {
            self.ship_advance_points(ship).unwrap() * 100 + ship.speed * 10 + ship.coal
        };

        let current_points = calculate_points(self.current_ship);
        let other_points = calculate_points(self.other_ship);

        match (current_points.cmp(&other_points), &self.current_ship.team) {
            (Ordering::Greater, _) => self.current_ship.clone(),
            (Ordering::Less, _) => self.other_ship.clone(),
            (_, TeamEnum::One) => self.current_ship.clone(),
            _ => self.other_ship.clone(),
        }
    }

    pub fn is_current_ship_on_current(&self) -> bool {
        self.board.does_field_have_stream(&self.current_ship.position)
    }

    pub fn perform_action(&self, action: Action) -> Result<GameState, PyErr> {
        let mut new_state = self.clone();

        match action {
            Action::Accelerate(accelerate) => {
                match accelerate.perform(&new_state) {
                    Ok(updated_ship) => {
                        new_state.current_ship = updated_ship;
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            Action::Advance(advance) => {
                match advance.perform(&new_state) {
                    Ok(updated_ship) => {
                        new_state.current_ship = updated_ship;
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            Action::Turn(turn) => {
                match turn.perform(&new_state) {
                    Ok(updated_ship) => {
                        new_state.current_ship = updated_ship;
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            Action::Push(push) => {
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

    fn move_after_check(&self, ship: Ship) -> Result<(), PyErr> {
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

    pub fn perform_move(&self, move_: Move) -> Result<GameState, PyErr> {
        debug!("Current ship before move: {:?}", self.current_ship);
        debug!("Other ship before move: {:?}", self.other_ship);

        if move_.actions.is_empty() {
            return Err(PyBaseException::new_err(MoveMistake::NoActions.message()));
        }

        let mut new_state = self.clone();
        debug!("Actions: {:?}", move_.actions);

        for (i, action) in move_.actions.iter().enumerate() {
            new_state.move_pre_check(*action, i, self.current_ship).map_err(|e| {
                return e;
            })?;
            match new_state.perform_action(*action) {
                Ok(state) => {
                    new_state = state;
                }
                Err(e) => {
                    return Err(PyBaseException::new_err(e));
                }
            }
        }

        new_state.move_after_check(new_state.current_ship).map_err(|e| {
            return e;
        })?;

        new_state.pick_up_passenger_current_ship();
        new_state.current_ship.points = new_state
            .ship_points(new_state.current_ship)
            .expect("Could not calculate ship points");

        if move_.actions.iter().any(|a| matches!(a, Action::Push(_))) {
            if new_state.other_ship.speed == 1 {
                new_state.pick_up_passenger_other_ship();
            }
            new_state.other_ship.points = new_state
                .ship_points(new_state.other_ship)
                .expect("Could not calculate other ship's points");
        }

        new_state.last_move = Some(move_);
        new_state.advance_turn();

        debug!("Current ship after move: {:?}", new_state.current_ship);
        debug!("Other ship after move: {:?}", new_state.other_ship);

        Ok(new_state)
    }

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

    pub fn effective_speed(&self, ship: Ship) -> i32 {
        ship.speed - (self.board.does_field_have_stream(&ship.position) as i32)
    }

    pub fn remove_passenger_at(&mut self, coord: CubeCoordinates) -> bool {
        for &d in CubeDirection::VALUES.iter() {
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
        if self.effective_speed(self.current_ship) < 2 {
            if self.remove_passenger_at(self.current_ship.position) {
                self.current_ship.passengers += 1;
                self.current_ship.points = self
                    .ship_points(self.current_ship)
                    .expect("Could not calculate ship points");
            }
        }
    }

    pub fn pick_up_passenger_other_ship(&mut self) {
        if self.effective_speed(self.other_ship) < 2 {
            if self.remove_passenger_at(self.other_ship.position) {
                self.other_ship.passengers += 1;
                self.other_ship.points = self
                    .ship_points(self.other_ship)
                    .expect("Could not calculate other ship's points");
            }
        }
    }

    pub fn ship_advance_points(&self, ship: Ship) -> Option<i32> {
        let (i, segment) = self.board.segment_with_index_at(ship.position)?;
        Some(
            (i as i32) * PluginConstants::POINTS_PER_SEGMENT +
                segment.global_to_local(ship.position).array_x() +
                1
        )
    }

    pub fn ship_points(&self, ship: Ship) -> Option<i32> {
        Some(
            self.ship_advance_points(ship.clone())? +
                (ship.passengers as i32) * PluginConstants::POINTS_PER_PASSENGER
        )
    }

    pub fn must_push(&self) -> bool {
        &self.current_ship.position == &self.other_ship.position
    }

    pub fn check_ship_advance_limit(&self, ship: &Ship) -> AdvanceInfo {
        self.calculate_advance_info(&ship.position, &ship.direction, ship.movement)
    }

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
                        return AdvanceInfo { costs, problem: AdvanceProblem::InsufficientPush };
                    }

                    if let FieldType::Sandbank = field.field_type {
                        return AdvanceInfo { costs, problem: AdvanceProblem::MoveEndOnSandbank };
                    }
                    costs.push(total_cost);
                }
                _ => {
                    return AdvanceInfo { costs, problem: AdvanceProblem::FieldIsBlocked };
                }
            }
        }

        return AdvanceInfo { costs, problem: AdvanceProblem::MovementPointsMissing };
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
                    merged_actions.push(Action::Advance(Advance { distance: total_distance }));
                }
                _ => merged_actions.push(action),
            }
        }

        merged_actions
    }

    pub fn possible_moves(&self, depth: Option<usize>) -> Vec<Move> {
        self.possible_action_comb(&self, vec![], 0, depth.unwrap_or(PluginConstants::MAX_DEPTH))
            .into_iter()
            .map(|actions| Move { actions })
            .collect()
    }

    pub fn possible_action_comb(
        &self,
        current_state: &GameState,
        current_actions: Vec<Action>,
        depth: usize,
        max_depth: usize
    ) -> Vec<Vec<Action>> {
        if depth > max_depth || (!current_state.can_move() && !current_state.must_push()) {
            return current_state
                .move_after_check(current_state.current_ship)
                .map_or(vec![], |_|
                    vec![current_state.merge_consecutive_advances(current_actions)]
                );
        }

        current_state
            .possible_actions(depth, None)
            .iter()
            .filter_map(|&action| {
                current_state
                    .perform_action(action)
                    .ok()
                    .and_then(|new_state| {
                        let mut new_actions = current_actions.clone();
                        new_actions.push(action);
                        Some(
                            self.possible_action_comb(&new_state, new_actions, depth + 1, max_depth)
                        )
                    })
            })
            .flatten()
            .collect()
    }

    pub fn possible_accelerations(&self, max_coal: Option<usize>) -> Vec<Accelerate> {
        if self.must_push() {
            return Vec::new();
        }

        let ship = self.current_ship;
        let max_coal = max_coal.unwrap_or(ship.coal.try_into().unwrap());
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

    pub fn possible_advances(&self) -> Vec<Advance> {
        if self.current_ship.movement < 1 || self.must_push() {
            return Vec::new();
        }

        self.check_ship_advance_limit(&self.current_ship).advances(None)
    }

    #[allow(unused_variables)]
    pub fn sandbank_advances_for(&self, ship: &Ship) -> Option<Vec<Advance>> {
        panic!(
            "Sandbanks will not be included in the Software-Challenge 2024. 
        Consequently, this particular method will not be implemented for the duration of this season."
        );
    }

    pub fn possible_actions(&self, rank: usize, max_coal: Option<usize>) -> Vec<Action> {
        let max_coal = max_coal.unwrap_or(self.current_ship.coal.try_into().unwrap());
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

    pub fn coal_for_action(&self, action: Action) -> usize {
        match action {
            Action::Accelerate(acc) => {
                (acc.acc.abs() as usize) - (self.current_ship.free_acc as usize)
            }
            Action::Turn(dir) => {
                let turn_count: i32 = self.current_ship.direction.turn_count_to(dir.direction);
                (turn_count.abs() as usize) - (self.current_ship.free_turns as usize)
            }
            Action::Push(_) | Action::Advance(_) => { 0 }
        }
    }

    pub fn can_move(&self) -> bool {
        let current_ship_can_advance: bool = !self.possible_advances().is_empty();

        let current_ship_can_turn: bool = !self.possible_turns(None).is_empty();

        let current_ship_can_accelerate: bool = !self.possible_accelerations(None).is_empty();

        current_ship_can_advance || current_ship_can_turn || current_ship_can_accelerate
    }

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

    pub fn is_winner(&self, ship: &Ship) -> bool {
        ship.passengers > 1 &&
            self.board.effective_speed(ship) < 2 &&
            self.board
                .get(&ship.position)
                .unwrap_or_else(|| {
                    panic!("[is_winner] Field at position {} does not exist", ship.position)
                }).field_type == FieldType::Goal
    }

    pub fn get_points_for_team(&self, ship: &Ship) -> TeamPoints {
        let finish_points = PluginConstants::FINISH_POINTS * (self.is_winner(ship) as i32);
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

#[cfg(test)]
mod tests {
    use crate::plugin::{ segment::Segment, field::{ Field, Passenger }, ship::TeamEnum };

    use super::*;

    fn create_water_segment(center: CubeCoordinates, direction: CubeDirection) -> Segment {
        Segment {
            direction,
            center,
            fields: vec![
                vec![Field::new(FieldType::Water, None); 4];
                5
            ],
        }
    }

    fn create_ship(position: CubeCoordinates, team: TeamEnum) -> Ship {
        Ship::new(position, team, None, None, None, None, None, None, None)
    }

    fn create_game_state(segment: Vec<Segment>, team_one: Ship, team_two: Ship) -> GameState {
        GameState::new(Board::new(segment, CubeDirection::Right), 0, team_one, team_two, None)
    }

    #[test]
    fn test_remove_passenger_at() {
        let mut segment = vec![
            create_water_segment(CubeCoordinates::new(0, 0), CubeDirection::Right)
        ];
        let team_one = create_ship(CubeCoordinates::new(0, -1), TeamEnum::One);
        let team_two = create_ship(CubeCoordinates::new(-1, 1), TeamEnum::Two);
        segment[0].set(
            CubeCoordinates::new(0, 0),
            Field::new(FieldType::Passenger, Some(Passenger::new(CubeDirection::UpLeft, 1)))
        );
        let mut game_state = create_game_state(segment, team_one, team_two);

        assert_eq!(game_state.current_ship.passengers, 0);
        assert_eq!(game_state.current_ship.points, 0);
        game_state.pick_up_passenger_current_ship();
        assert_eq!(game_state.current_ship.passengers, 1);
        assert_eq!(game_state.current_ship.points, 6);
        game_state.pick_up_passenger_current_ship();
        assert_eq!(game_state.current_ship.passengers, 1);
        assert_eq!(game_state.current_ship.points, 6);
    }

    #[test]
    fn find_possible_moves_returns_correct_count() {
        let segment = vec![create_water_segment(CubeCoordinates::new(0, 0), CubeDirection::Right)];
        let team_one = create_ship(CubeCoordinates::new(0, -1), TeamEnum::One);
        let team_two = create_ship(CubeCoordinates::new(-1, 1), TeamEnum::Two);
        let game_state = create_game_state(segment, team_one, team_two);

        let possible_moves = game_state.possible_action_comb(&game_state, vec![], 0, 5);
        assert_eq!(possible_moves.len(), 6725);
    }

    #[test]
    fn test_check_advance_limit() {
        let segment = vec![create_water_segment(CubeCoordinates::new(0, 0), CubeDirection::Right)];
        let mut team_one = create_ship(CubeCoordinates::new(0, 0), TeamEnum::One);
        team_one.speed = 5;
        team_one.movement = 5;
        let mut team_two = create_ship(CubeCoordinates::new(-1, 1), TeamEnum::Two);
        team_two.speed = 5;
        team_two.movement = 5;
        let game_state = create_game_state(segment, team_one, team_two);

        let advances: AdvanceInfo = game_state.check_ship_advance_limit(&team_one);

        assert_eq!(advances.costs, vec![2, 3, 4]);
        assert_eq!(advances.problem, AdvanceProblem::FieldIsBlocked);
        assert_eq!(advances.distance(), 3);
    }

    #[test]
    fn test_check_advance_limit_to_upperleft_end() {
        let segment = vec![create_water_segment(CubeCoordinates::new(0, 0), CubeDirection::Right)];
        let mut team_one = create_ship(CubeCoordinates::new(0, -1), TeamEnum::One);
        team_one.speed = 5;
        team_one.movement = 5;
        team_one.direction = CubeDirection::Left;
        let mut team_two = create_ship(CubeCoordinates::new(-1, 1), TeamEnum::Two);
        team_two.speed = 5;
        team_two.movement = 5;
        let game_state = create_game_state(segment, team_one, team_two);

        let advances: AdvanceInfo = game_state.check_ship_advance_limit(&team_one);

        assert_eq!(advances.costs, vec![1]);
        assert_eq!(advances.problem, AdvanceProblem::FieldIsBlocked);
        assert_eq!(advances.distance(), 1);
    }

    #[test]
    fn test_get_accelerations() {
        let segment = vec![create_water_segment(CubeCoordinates::new(0, 0), CubeDirection::Right)];
        let mut team_one = create_ship(CubeCoordinates::new(0, 0), TeamEnum::One);
        team_one.speed = 5;
        team_one.movement = 5;
        let mut team_two = create_ship(CubeCoordinates::new(-1, 1), TeamEnum::Two);
        team_two.speed = 5;
        team_two.movement = 5;
        let game_state = create_game_state(segment, team_one, team_two);

        let accelerations: Vec<Accelerate> = game_state.possible_accelerations(None);
        assert_eq!(accelerations.len(), 5);
        assert_eq!(accelerations[4].acc, -4);
    }

    #[test]
    fn test_get_turns() {
        let segment = vec![create_water_segment(CubeCoordinates::new(0, 0), CubeDirection::Right)];
        let mut team_one = create_ship(CubeCoordinates::new(0, 0), TeamEnum::One);
        team_one.speed = 5;
        team_one.movement = 5;
        let mut team_two = create_ship(CubeCoordinates::new(-1, 1), TeamEnum::Two);
        team_two.speed = 5;
        team_two.movement = 5;
        let game_state = create_game_state(segment, team_one, team_two);

        let turns: Vec<Turn> = game_state.possible_turns(None);
        assert_eq!(turns.len(), 5);
        assert_eq!(turns[4].direction, CubeDirection::Left);
    }

    #[test]
    fn test_get_advances() {
        let segment = vec![create_water_segment(CubeCoordinates::new(0, 0), CubeDirection::Right)];
        let mut team_one = create_ship(CubeCoordinates::new(0, 0), TeamEnum::One);
        team_one.speed = 5;
        team_one.movement = 5;
        let mut team_two = create_ship(CubeCoordinates::new(-1, 1), TeamEnum::Two);
        team_two.speed = 5;
        team_two.movement = 5;
        let game_state = create_game_state(segment, team_one, team_two);

        let advances: Vec<Advance> = game_state.possible_advances();
        assert_eq!(advances.len(), 3);
        assert_eq!(advances[1].distance, 2);
    }

    #[test]
    fn test_get_pushes() {
        let segment = vec![create_water_segment(CubeCoordinates::new(0, 0), CubeDirection::Right)];
        let mut team_one = create_ship(CubeCoordinates::new(0, 0), TeamEnum::One);
        team_one.speed = 5;
        team_one.movement = 5;
        let mut team_two = create_ship(CubeCoordinates::new(0, 0), TeamEnum::Two);
        team_two.speed = 5;
        team_two.movement = 5;
        let game_state = create_game_state(segment, team_one, team_two);

        let pushes: Vec<Push> = game_state.possible_pushes();
        assert_eq!(pushes.len(), 5);
        assert_eq!(pushes[0].direction, CubeDirection::Right);
    }

    #[test]
    fn test_only_pushes_if_must_push() {
        let segment = vec![create_water_segment(CubeCoordinates::new(0, 0), CubeDirection::Right)];
        let mut team_one = create_ship(CubeCoordinates::new(0, 0), TeamEnum::One);
        team_one.speed = 5;
        team_one.movement = 5;
        let mut team_two = create_ship(CubeCoordinates::new(0, 0), TeamEnum::Two);
        team_two.speed = 5;
        team_two.movement = 5;
        let game_state = create_game_state(segment, team_one, team_two);

        let actions: Vec<Action> = game_state.possible_actions(1, None);
        assert_eq!(actions.len(), 5);
        assert!(actions.iter().all(|a| matches!(a, Action::Push(_))));
    }

    #[test]
    fn test_performe_move() {
        let segment: Vec<Segment> = vec![
            Segment {
                direction: CubeDirection::Right,
                center: CubeCoordinates::new(0, 0),
                fields: vec![
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
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Water, None)
                    ]
                ],
            },
            Segment {
                direction: CubeDirection::Right,
                center: CubeCoordinates::new(3, 0),
                fields: vec![
                    vec![
                        Field::new(FieldType::Water, None),
                        Field::new(
                            FieldType::Passenger,
                            Some(Passenger::new(CubeDirection::DownLeft, 1))
                        ),
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
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Water, None)
                    ],
                    vec![
                        Field::new(FieldType::Island, None),
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Water, None)
                    ],
                    vec![
                        Field::new(FieldType::Island, None),
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Water, None)
                    ]
                ],
            }
        ];
        let board: Board = Board::new(segment, CubeDirection::Right);
        let team_one: &mut Ship = &mut Ship::new(
            CubeCoordinates::new(-1, -1),
            TeamEnum::One,
            Some(CubeDirection::Right),
            Some(1),
            Some(6),
            Some(0),
            Some(0),
            Some(0),
            Some(1)
        );
        let team_two: &mut Ship = &mut Ship::new(
            CubeCoordinates::new(-2, 1),
            TeamEnum::Two,
            Some(CubeDirection::Right),
            Some(1),
            Some(6),
            Some(0),
            Some(0),
            Some(0),
            Some(1)
        );
        let game_state: GameState = GameState::new(
            board,
            0,
            team_one.clone(),
            team_two.clone(),
            None
        );

        let move_: Move = Move::new(
            vec![Action::Accelerate(Accelerate::new(1)), Action::Advance(Advance::new(2))]
        );

        assert_eq!(game_state.current_ship.team, TeamEnum::One);
        assert_eq!(game_state.current_ship.position, CubeCoordinates::new(-1, -1));

        let new_state: GameState = game_state.perform_move(move_).unwrap();
        assert_eq!(new_state.other_ship.team, TeamEnum::One);
        assert_eq!(new_state.other_ship.position, CubeCoordinates::new(1, -1));

        assert_eq!(new_state.current_ship.team, TeamEnum::Two);
        assert_eq!(new_state.current_ship.position, CubeCoordinates::new(-2, 1));

        let second_move_: Move = Move::new(
            vec![Action::Accelerate(Accelerate::new(1)), Action::Advance(Advance::new(2))]
        );

        let second_new_state: GameState = new_state.perform_move(second_move_).unwrap();
        assert_eq!(second_new_state.current_ship.team, TeamEnum::One);
        assert_eq!(second_new_state.current_ship.position, CubeCoordinates::new(1, -1));
        assert_eq!(second_new_state.other_ship.team, TeamEnum::Two);
        assert_eq!(second_new_state.other_ship.position, CubeCoordinates::new(0, 1));
    }

    #[test]
    fn test_advance_turn() {
        let segment = vec![create_water_segment(CubeCoordinates::new(0, 0), CubeDirection::Right)];
        let team_one = create_ship(CubeCoordinates::new(0, -1), TeamEnum::One);
        let team_two = create_ship(CubeCoordinates::new(-1, 1), TeamEnum::Two);
        let mut game_state = create_game_state(segment, team_one, team_two);

        assert_eq!(game_state.current_ship.team, TeamEnum::One);
        assert_eq!(game_state.other_ship.team, TeamEnum::Two);

        game_state.advance_turn();

        assert_eq!(game_state.current_ship.team, TeamEnum::Two);
        assert_eq!(game_state.other_ship.team, TeamEnum::One);

        game_state.advance_turn();

        assert_eq!(game_state.current_ship.team, TeamEnum::One);
        assert_eq!(game_state.other_ship.team, TeamEnum::Two);
    }

    #[test]
    fn test_team_ahead() {
        let segment = vec![create_water_segment(CubeCoordinates::new(0, 0), CubeDirection::Right)];
        let team_one = create_ship(CubeCoordinates::new(0, -1), TeamEnum::One);
        let team_two = create_ship(CubeCoordinates::new(-1, 1), TeamEnum::Two);
        let game_state = create_game_state(segment, team_one, team_two);

        assert_eq!(game_state.determine_ahead_team().team, TeamEnum::One);

        let _move: Move = Move::new(vec![Action::Advance(Advance::new(1))]);

        let new_state: GameState = game_state.perform_move(_move).unwrap();

        assert_eq!(new_state.determine_ahead_team().team, TeamEnum::One);

        let second_move: Move = Move::new(
            vec![Action::Accelerate(Accelerate::new(1)), Action::Advance(Advance::new(2))]
        );

        let second_new_state: GameState = new_state.perform_move(second_move).unwrap();

        assert_eq!(second_new_state.determine_ahead_team().team, TeamEnum::Two);
    }
}
