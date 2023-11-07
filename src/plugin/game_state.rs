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

    pub fn advances(&self, distance: usize) -> Vec<Advance> {
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

    /// Performs the sequence of actions specified in the provided `Move` object.
    ///
    /// This method applies each `Action` in the `Move` to the current game state,
    /// updating the state as it goes. The method ensures that the sequence of actions
    /// adheres to the game rules by checking requirements and constraints specific to each `Action`.
    /// If an action doesn't comply with the game rules, an error is returned.
    ///
    /// # Args
    ///
    /// * `move_: Move` - a sequence of actions to be performed.
    ///
    /// # Returns
    ///
    /// * `Ok(GameState)` - a new game state after successfully performing all actions.
    /// * `Err(ActionProblem)` - an error occurred either because an action was unlawful
    /// or because there was a problem applying the action to the game state.
    ///
    /// # Examples
    ///
    /// ```Python
    /// move = Move([Accelerate(1), Advance(1), Turn(CubeDirection.Right)])
    /// new_state = game_state.perform_move(move)
    /// print(new_state)
    /// ```
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

    pub fn get_simple_moves(&self) -> Vec<Move> {
        let actions = self.get_actions(0, self.current_ship.coal);

        let moves = actions
            .into_iter()
            .map(|action| Move::new(vec![action]))
            .collect();
        moves
    }

    pub fn get_actions(&self, rank: i32, max_coal: i32) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::new();
        if rank == 0 {
            actions.extend(self.get_accelerations(max_coal).into_iter().map(Action::Accelerate));
        }
        actions.extend(self.get_turns(max_coal).into_iter().map(Action::Turn));
        actions.extend(self.get_advances().into_iter().map(Action::Advance));
        if rank != 0 {
            actions.extend(self.get_pushes().into_iter().map(Action::Push));
        }
        actions
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

    pub fn get_pushes(&self) -> Vec<Push> {
        if
            self.board
                .get(&self.current_ship.position)
                .unwrap_or_else(|| {
                    panic!(
                        "[get_pushes] Field at position {} does not exist",
                        self.current_ship.position
                    )
                }).field_type == FieldType::Sandbank ||
            !self.must_push() ||
            self.current_ship.movement < 1
        {
            Vec::new()
        } else {
            self.get_pushes_from(&self.current_ship.position, &self.current_ship.direction)
        }
    }

    pub fn get_pushes_from(
        &self,
        position: &CubeCoordinates,
        incoming_direction: &CubeDirection
    ) -> Vec<Push> {
        CubeDirection::VALUES.iter()
            .filter(|dir| {
                *dir != &incoming_direction.opposite() &&
                    self.board
                        .get_field_in_direction(*dir, position)
                        .map_or(false, |field| field.is_empty())
            })
            .map(|dir| Push::new(*dir))
            .collect()
    }

    pub fn get_turns(&self, max_coal: i32) -> Vec<Turn> {
        if
            self.board
                .get(&self.current_ship.position)
                .unwrap_or_else(|| {
                    panic!(
                        "[get_turns] Field at position {} does not exist",
                        self.current_ship.position
                    )
                }).field_type == FieldType::Sandbank ||
            self.must_push()
        {
            Vec::new()
        } else {
            let max_turn_count = (max_coal + self.current_ship.free_turns).min(3).max(0);

            (1..=max_turn_count)
                .flat_map(|i| {
                    vec![
                        Turn::new(self.current_ship.direction.rotated_by(i)),
                        Turn::new(self.current_ship.direction.rotated_by(-i))
                    ]
                })
                .take(5)
                .collect()
        }
    }

    pub fn get_advances(&self) -> Vec<Advance> {
        if self.current_ship.movement < 1 || self.must_push() {
            return Vec::new();
        }

        let sandbank: Vec<Advance> = self.check_sandbank_advances(&self.current_ship);

        if !sandbank.is_empty() {
            return sandbank;
        }
        let ship = self.current_ship;
        return self.check_ship_advance_limit(&ship).advances(ship.movement.try_into().unwrap());
    }

    pub fn check_sandbank_advances(&self, ship: &Ship) -> Vec<Advance> {
        let mut advances = Vec::new();
        if
            self.board
                .get(&ship.position)
                .unwrap_or_else(|| {
                    panic!(
                        "[check_sandbank_advances] Field at position {} does not exist",
                        ship.position
                    )
                }).field_type == FieldType::Sandbank
        {
            if self.check_ship_advance_limit(ship).distance() > 1 {
                let advance1 = Advance::new(1);
                advances.push(advance1);
            }

            if self.check_ship_advance_limit(ship).distance() > 1 {
                let advance_minus1 = Advance::new(-1);
                advances.push(advance_minus1);
            }
        }
        advances
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
            current_position = current_position + direction.vector();
            total_cost += 1;
            let current_field_option: Option<Field> = self.board.get(&current_position);

            if current_field_option.is_none() {
                return result!(AdvanceProblem::FieldIsBlocked);
            }

            if !has_current && self.board.does_field_have_stream(&current_position) {
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

    /// Returns a vector of possible accelerations regards to the current ship and plugin constants.
    ///
    /// The function calculates the lower and upper bound of possible accelerations, taking into account
    /// the ship's current speed and free accelerations, as well as the maximum allowable speed provided by
    /// `PluginConstants`. It then generates a set of accelerations within this range.
    ///
    /// # Arguments
    ///
    /// * `max_coal` - The maximum amount of coal that can be consumed to produce an acceleration.
    ///
    /// # Returns
    ///
    /// * `Vec<Accelerate>` - A vector of `Accelerate` objects representing the possible accelerations.
    ///   If the ship must push (determined by `self.must_push()`), it will return an empty vector.
    pub fn get_accelerations(&self, max_coal: i32) -> Vec<Accelerate> {
        if self.must_push() {
            return Vec::new();
        }

        let current_ship: Ship = self.current_ship;

        (1..=max_coal + current_ship.free_acc)
            .flat_map(|i|
                vec![
                    if PluginConstants::MAX_SPEED >= current_ship.speed + i {
                        Some(Accelerate::new(i))
                    } else {
                        None
                    },
                    if PluginConstants::MIN_SPEED <= current_ship.speed - i {
                        Some(Accelerate::new(-i))
                    } else {
                        None
                    }
                ]
            )
            .filter_map(|x| x)
            .collect()
    }

    /// This is a function to check the movement possibilities for the current and another ship.
    /// It determines whether either ship can perform any of the following actions:
    /// 1. Advancing
    /// 2. Turning
    /// 3. Accelerating
    ///
    /// # Returns
    ///
    /// `true` if either the current ship or the other ship can do any of the actions.
    /// `false` if neither of the ships can perform any action.
    pub fn can_move(&self) -> bool {
        let current_ship: Ship = self.current_ship;

        let current_ship_can_advance: bool = !self.get_advances().is_empty();

        let current_ship_can_turn: bool = !self.get_turns(current_ship.coal).is_empty();

        let current_ship_can_accelerate: bool = !self
            .get_accelerations(current_ship.coal)
            .is_empty();

        current_ship_can_advance || current_ship_can_turn || current_ship_can_accelerate
    }

    /// Checks if the game is over based on a set of conditions.
    ///
    /// This function checks for the following end game scenarios:
    ///
    /// 1. Condition 1: A ship with 2 passengers reaches a goal field with speed 1
    /// 2. Condition 2: A player makes an invalid move. This is handled by an InvalidMoveException during the game.
    /// 3. Condition 3: At the end of a round, a ship is more than 3 game segments behind
    /// 4. Condition 4: The round limit of 30 rounds is reached
    /// 5. Condition 5: Both players cannot move anymore
    ///
    /// If any of the above conditions are met, the game is considered over.
    ///
    /// # Returns
    /// * `bool` - Returns `true` if any of the conditions are met, signifying the end of the game. Otherwise, it returns `false`.
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

    /// `is_winner` is a function that checks if a specific ship wins the game.
    ///
    /// The function takes a reference to a ship. A ship is considered a winner if:
    /// - it has more than one passenger,
    /// - its effective speed on the board is less than 2,
    /// - it is in a field of type Goal.
    ///
    /// Arguments:
    /// * `ship`: A reference to the `Ship` object.
    ///
    /// Returns:
    /// `bool` - Returns `true` if the ship meets all the win conditions, `false` otherwise.
    ///
    /// Examples:
    /// ```
    /// assert_eq!(game.is_winner(&ship), true);
    /// ```
    pub fn is_winner(&self, ship: &Ship) -> bool {
        ship.passengers > 1 &&
            self.board.effective_speed(ship) < 2 &&
            self.board
                .get(&ship.position)
                .unwrap_or_else(|| {
                    panic!("[is_winner] Field at position {} does not exist", ship.position)
                }).field_type == FieldType::Goal
    }

    /// This function calculates and returns the total points for a specific team.
    /// The `get_points_for_team` function accepts two parameters: a reference to the team's ship structure
    /// and a reference to self.
    ///
    /// It calculates the team's points as follows:
    /// - Ship points which are directly derived from the ship's point field.
    /// - Coal points which are calculated by multiplying the ship's coal field by 2.
    /// - Finish points which are calculated based on if the ship is the winning team or not.
    /// The result is encapsulated into a TeamPoints structure and returned.
    ///
    /// # Arguments
    ///
    /// * `ship` - A reference to a ship object from the team.
    ///
    /// # Returns
    ///
    /// * `TeamPoints` - A structure that contains information about the points of the team.
    ///
    /// # Example
    ///
    /// ```Rust
    /// // Assuming ship1 is an instance of Ship with appropriate values
    /// let team_points = game.get_points_for_team(&ship1);
    /// println!("Ship Points: {}, Coal Points: {}, Finish Points: {}", team_points.ship_points, team_points.coal_points, team_points.finish_points);
    /// ```
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

        let accelerations: Vec<Accelerate> = game_state.get_accelerations(5);
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

        let turns: Vec<Turn> = game_state.get_turns(5);
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

        let advances: Vec<Advance> = game_state.get_advances();
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

        let pushes: Vec<Push> = game_state.get_pushes();
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
