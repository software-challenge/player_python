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
use crate::plugin::field::{ FieldType, Field };
use crate::plugin::r#move::Move;
use crate::plugin::ship::Ship;
use crate::plugin::errors::advance_errors::AdvanceProblem;

use super::field::Passenger;

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
        debug!("Costs: {:?} at distance {}", cost, distance);
        cost
    }

    pub fn advances(&self, distance: Option<usize>) -> Vec<Advance> {
        let distance = distance.unwrap_or(self.costs.len().saturating_sub(1));
        (1..=distance).map(|it| Advance { distance: it as i32 }).collect()
    }

    pub fn distance(&self) -> usize {
        self.costs.len()
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("AdvanceInfo(costs={:?}, problem={:?})", self.costs, self.problem))
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
        vec![self.other_ship, self.current_ship]
            .into_iter()
            .max_by_key(|s| (s.points, s.speed, s.coal))
            .unwrap()
    }

    pub fn is_current_ship_on_current(&self) -> bool {
        self.board.does_field_have_stream(&self.current_ship.position)
    }

    fn perform_move(&self, move_: Move) -> Result<GameState, PyErr> {
        let mut new_state: GameState = self.clone();

        debug!("Current ship before move: {:?}", new_state.current_ship);
        debug!("Other ship before move: {:?}", new_state.other_ship);

        let actions: &Vec<Action> = &move_.actions;
        if actions.is_empty() {
            return Err(PyBaseException::new_err(MoveMistake::NoActions.message()));
        }

        debug!("Actions: {:?}", actions);

        for (i, action) in actions.iter().enumerate() {
            match action {
                Action::Push(_) if !new_state.must_push() => {
                    return Err(PyBaseException::new_err(MoveMistake::PushActionRequired.message()));
                }
                Action::Accelerate(_) if i != 0 => {
                    return Err(
                        PyBaseException::new_err(MoveMistake::FirstActionAccelerate.message())
                    );
                }
                Action::Advance(ad) => {
                    let future_field = new_state.board.get(
                        &(
                            new_state.current_ship.position +
                            new_state.current_ship.direction.vector() * ad.distance
                        )
                    );
                    if
                        future_field.is_some() &&
                        future_field.unwrap().field_type == FieldType::Sandbank
                    {
                        return Err(PyBaseException::new_err(MoveMistake::SandBankEnd.message()));
                    }
                }
                _ => {}
            }

            let mut new_current_ship: Result<Ship, PyErr> = Ok(new_state.current_ship);
            let mut new_other_ship: Result<Ship, PyErr> = Ok(new_state.other_ship);

            match action {
                Action::Accelerate(accelerate) => {
                    new_current_ship = accelerate.perform(&new_state);
                }
                Action::Advance(advance) => {
                    new_current_ship = advance.perform(&new_state);
                }
                Action::Push(push) => {
                    new_other_ship = push.perform(&new_state);
                }
                Action::Turn(turn) => {
                    new_current_ship = turn.perform(&new_state);
                }
            }

            new_state.current_ship = new_current_ship?;
            new_state.other_ship = new_other_ship?;
        }

        match new_state.current_ship.movement {
            p if p > 0 => {
                return Err(PyBaseException::new_err(MoveMistake::MovementPointsLeft.message()));
            }
            p if p < 0 => {
                return Err(PyBaseException::new_err(MoveMistake::MovementPointsMissing.message()));
            }
            _ => {}
        }

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
        let current_ship: &mut Ship = &mut self.current_ship;

        current_ship.free_acc = 1;
        current_ship.free_turns = 1;
        current_ship.movement = current_ship.speed;

        self.turn += 1;

        if self.turn % 2 == 0 {
            if self.determine_ahead_team() != self.current_ship {
                swap(&mut self.current_ship, &mut self.other_ship);
            }
        } else {
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

    fn remove_passenger_at(&mut self, coord: CubeCoordinates) -> bool {
        let mut passenger_removed = false;
        for d in CubeDirection::VALUES {
            if let Some(mut field) = self.board.get_field_in_direction(&d, &coord) {
                if let Some(Passenger { passenger, direction }) = &mut field.passenger {
                    if *passenger > 0 && *direction == d.opposite() {
                        *passenger -= 1;
                        passenger_removed = true;
                    }
                }
            }
        }
        passenger_removed
    }

    pub fn pick_up_passenger_current_ship(&mut self) {
        if self.effective_speed(self.current_ship) < 2 {
            if self.remove_passenger_at(self.current_ship.position) {
                self.current_ship.passengers += 1;
            }
        }
    }

    pub fn pick_up_passenger_other_ship(&mut self) {
        if self.effective_speed(self.other_ship) < 2 {
            if self.remove_passenger_at(self.other_ship.position) {
                self.other_ship.passengers += 1;
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
        let mut current_position: CubeCoordinates = *start;
        let mut total_cost: i32 = 0;
        let mut has_current: bool = false;
        let max_movement: i32 = max_movement_points.clamp(0, PluginConstants::MAX_SPEED);
        let mut costs: Vec<i32> = Vec::new();

        macro_rules! result {
            ($problem:expr) => {
                AdvanceInfo { costs, problem: $problem }
            };
        }

        while total_cost < max_movement {
            current_position += direction.vector();
            debug!("Current Position: {:?}", current_position);
            debug!("Vector: {:?}", direction.vector());
            total_cost += 1;
            let current_field_option: Option<Field> = self.board.get(&current_position);

            if current_field_option.is_none() {
                debug!("Current Position: {:?}", current_position);
                return result!(AdvanceProblem::FieldIsBlocked);
            }

            if !has_current && self.board.does_field_have_stream(&current_position) {
                has_current = true;
                if total_cost < max_movement {
                    total_cost += 1;
                } else {
                    debug!("Movement points missing due to stream");
                    break;
                }
            }

            if
                self.current_ship.position == current_position ||
                self.other_ship.position == current_position
            {
                if total_cost < max_movement {
                    costs.push(total_cost);
                    return result!(AdvanceProblem::ShipAlreadyInTarget);
                }
                return result!(AdvanceProblem::InsufficientPush);
            }

            if let FieldType::Sandbank = current_field_option.unwrap().field_type {
                return result!(AdvanceProblem::MoveEndOnSandbank);
            }

            costs.push(total_cost);
        }

        result!(AdvanceProblem::MovementPointsMissing)
    }

    pub fn possible_accelerations(&self) -> Vec<Accelerate> {
        if self.must_push() {
            return Vec::new();
        }

        let ship = self.current_ship;
        return (1..=self.current_ship.coal + ship.free_acc)
            .flat_map(|i| [i, -i])
            .filter(|&i| (
                if i > 0 {
                    PluginConstants::MAX_SPEED >= ship.speed + i
                } else {
                    PluginConstants::MIN_SPEED <= ship.speed - i
                }
            ))
            .map(Accelerate::new)
            .collect();
    }

    pub fn possible_pushes(&self) -> Vec<Push> {
        let ship = self.current_ship;
        if !self.must_push() || self.board.is_sandbank(&ship.position) || ship.movement < 1 {
            return Vec::new();
        }

        CubeDirection::VALUES.into_iter()
            .filter(
                |&d|
                    d != ship.direction.opposite() &&
                    self.board.get(&(ship.position + d.vector())).is_some() &&
                    self.board
                        .get(&(ship.position + d.vector()))
                        .unwrap()
                        .is_empty()
            )
            .map(Push::new)
            .collect()
    }

    pub fn possible_turns(&self) -> Vec<Turn> {
        let ship = self.current_ship;
        if self.must_push() || self.board.is_sandbank(&ship.position) {
            return Vec::new();
        }
        let max_turn_count = (ship.coal + ship.free_turns).min(3) as i32;
        (1..=max_turn_count)
            .flat_map(|i| [i, -i])
            .map(|turns| Turn::new(ship.direction.rotated_by(turns)))
            .take(5)
            .collect()
    }

    pub fn possible_advances(&self) -> Vec<Advance> {
        let ship = self.current_ship;
        if ship.movement < 1 || self.must_push() {
            return Vec::new();
        }

        self.sandbank_advances_for(&ship).unwrap_or_else(||
            self.check_ship_advance_limit(&ship).advances(None)
        )
    }

    pub fn sandbank_advances_for(&self, ship: &Ship) -> Option<Vec<Advance>> {
        if self.board.is_sandbank(&ship.position) {
            Some(
                [-1, 1]
                    .into_iter()
                    .map(Advance::new)
                    .filter(|a| {
                        let direction = if a.distance < 0 {
                            ship.direction.opposite()
                        } else {
                            ship.direction
                        };

                        let advanced_ship: Ship = Ship::new(
                            ship.position,
                            ship.team,
                            Some(direction),
                            Some(1),
                            Some(1),
                            Some(ship.coal),
                            Some(ship.points),
                            Some(ship.passengers),
                            Some(ship.free_turns)
                        );
                        self.check_ship_advance_limit(&advanced_ship).distance() > 1
                    })
                    .collect()
            )
        } else {
            None
        }
    }

    pub fn possible_actions(&self, rank: i32) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::new();

        if rank == 0 {
            actions.extend(self.possible_accelerations().into_iter().map(Action::Accelerate));
        }
        actions.extend(self.possible_turns().into_iter().map(Action::Turn));
        actions.extend(self.possible_advances().into_iter().map(Action::Advance));
        if rank != 0 {
            actions.extend(self.possible_pushes().into_iter().map(Action::Push));
        }

        actions
    }

    pub fn can_move(&self) -> bool {
        let current_ship_can_advance: bool = !self.possible_advances().is_empty();

        let current_ship_can_turn: bool = !self.possible_turns().is_empty();

        let current_ship_can_accelerate: bool = !self.possible_accelerations().is_empty();

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

    #[test]
    fn test_check_advance_limit() {
        let segment: Vec<Segment> = vec![Segment {
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
        }];
        let board: Board = Board::new(segment, CubeDirection::Right);
        let team_one: &mut Ship = &mut Ship::new(
            CubeCoordinates::new(0, -1),
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
        let team_two: &mut Ship = &mut Ship::new(
            CubeCoordinates::new(-1, 1),
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
        let game_state: GameState = GameState::new(
            board,
            0,
            team_one.clone(),
            team_two.clone(),
            None
        );

        let advances: AdvanceInfo = game_state.check_ship_advance_limit(&team_one);

        assert_eq!(advances.costs, vec![1, 2, 3, 4, 5]);
        assert_eq!(advances.problem, AdvanceProblem::MovementPointsMissing);
        assert_eq!(advances.distance(), 5);
    }

    #[test]
    fn test_get_accelerations() {
        let segment: Vec<Segment> = vec![Segment {
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
        }];
        let board: Board = Board::new(segment, CubeDirection::Right);
        let team_one: &mut Ship = &mut Ship::new(
            CubeCoordinates::new(0, -1),
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
        let team_two: &mut Ship = &mut Ship::new(
            CubeCoordinates::new(-1, 1),
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
        let game_state: GameState = GameState::new(
            board,
            0,
            team_one.clone(),
            team_two.clone(),
            None
        );

        let accelerations: Vec<Accelerate> = game_state.possible_accelerations();
        assert_eq!(accelerations.len(), 5);
        assert_eq!(accelerations[4].acc, -4);
    }

    #[test]
    fn test_get_turns() {
        let segment: Vec<Segment> = vec![Segment {
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
        }];
        let board: Board = Board::new(segment, CubeDirection::Right);
        let team_one: &mut Ship = &mut Ship::new(
            CubeCoordinates::new(0, -1),
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
        let team_two: &mut Ship = &mut Ship::new(
            CubeCoordinates::new(-1, 1),
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
        let game_state: GameState = GameState::new(
            board,
            0,
            team_one.clone(),
            team_two.clone(),
            None
        );

        let turns: Vec<Turn> = game_state.possible_turns();
        assert_eq!(turns.len(), 5);
        assert_eq!(turns[4].direction, CubeDirection::Left);
    }

    #[test]
    fn test_get_advances() {
        let segment: Vec<Segment> = vec![Segment {
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
        }];
        let board: Board = Board::new(segment, CubeDirection::Right);
        let team_one: &mut Ship = &mut Ship::new(
            CubeCoordinates::new(0, -1),
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
        let team_two: &mut Ship = &mut Ship::new(
            CubeCoordinates::new(-1, 1),
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
        let game_state: GameState = GameState::new(
            board,
            0,
            team_one.clone(),
            team_two.clone(),
            None
        );

        let advances: Vec<Advance> = game_state.possible_advances();
        assert_eq!(advances.len(), 3);
        assert_eq!(advances[2].distance, 2);
    }

    #[test]
    fn test_get_pushes() {
        let segment: Vec<Segment> = vec![Segment {
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
            None
        );
        team_one.speed = 5;
        team_one.movement = 5;
        let team_two: &mut Ship = &mut Ship::new(
            CubeCoordinates::new(0, 0),
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
        let game_state: GameState = GameState::new(
            board,
            0,
            team_one.clone(),
            team_two.clone(),
            None
        );

        let pushes: Vec<Push> = game_state.possible_pushes();
        assert_eq!(pushes.len(), 5);
        assert_eq!(pushes[0].direction, CubeDirection::Right);
    }

    #[test]
    fn test_performance_move() {
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
            vec![
                Action::Accelerate(Accelerate::new(1)),
                Action::Advance(Advance::new(1)),
                Action::Turn(Turn::new(CubeDirection::UpRight)),
                Action::Advance(Advance::new(1))
            ]
        );

        assert_eq!(game_state.current_ship.team, TeamEnum::One);
        assert_eq!(game_state.current_ship.position, CubeCoordinates::new(-1, -1));

        let new_state: GameState = game_state.perform_move(move_).unwrap();
        assert_eq!(new_state.other_ship.team, TeamEnum::One);
        assert_eq!(new_state.other_ship.position, CubeCoordinates::new(1, -2));

        assert_eq!(new_state.current_ship.team, TeamEnum::Two);
        assert_eq!(new_state.current_ship.position, CubeCoordinates::new(-2, 1));

        let second_move_: Move = Move::new(
            vec![
                Action::Accelerate(Accelerate::new(1)),
                Action::Advance(Advance::new(1)),
                Action::Turn(Turn::new(CubeDirection::DownRight)),
                Action::Advance(Advance::new(1))
            ]
        );

        let second_new_state: GameState = new_state.perform_move(second_move_).unwrap();
        assert_eq!(second_new_state.other_ship.team, TeamEnum::Two);
        assert_eq!(second_new_state.other_ship.position, CubeCoordinates::new(-1, 2));
    }

    #[test]
    fn test_advance_turn() {
        let segment: Vec<Segment> = vec![Segment {
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
        }];
        let board: Board = Board::new(segment, CubeDirection::Right);
        let team_one: &mut Ship = &mut Ship::new(
            CubeCoordinates::new(0, -1),
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
        let team_two: &mut Ship = &mut Ship::new(
            CubeCoordinates::new(-1, 1),
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
        let game_state: &mut GameState = &mut GameState::new(
            board,
            0,
            team_one.clone(),
            team_two.clone(),
            None
        );

        assert_eq!(game_state.current_ship.team, TeamEnum::One);
        assert_eq!(game_state.other_ship.team, TeamEnum::Two);

        game_state.advance_turn();

        assert_eq!(game_state.current_ship.team, TeamEnum::Two);
        assert_eq!(game_state.other_ship.team, TeamEnum::One);

        game_state.other_ship.speed += 1;
        game_state.advance_turn();

        assert_eq!(game_state.current_ship.team, TeamEnum::One);
        assert_eq!(game_state.other_ship.team, TeamEnum::Two);
    }

    #[test]
    fn test_team_ahead() {
        let segment: Vec<Segment> = vec![Segment {
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
        }];
        let board: Board = Board::new(segment, CubeDirection::Right);
        let team_one: &mut Ship = &mut Ship::new(
            CubeCoordinates::new(0, -1),
            TeamEnum::One,
            None,
            None,
            None,
            None,
            None,
            None,
            None
        );
        let team_two: &mut Ship = &mut Ship::new(
            CubeCoordinates::new(-1, 1),
            TeamEnum::Two,
            None,
            None,
            None,
            None,
            None,
            None,
            None
        );
        let game_state: &mut GameState = &mut GameState::new(
            board,
            0,
            team_one.clone(),
            team_two.clone(),
            None
        );

        assert_eq!(game_state.determine_ahead_team().team, TeamEnum::One);

        game_state.other_ship.speed += 1;

        assert_eq!(game_state.determine_ahead_team().team, TeamEnum::Two);
    }

    #[test]
    fn test_is_winner() {}

    #[test]
    fn test_get_points_for_team() {}
}
