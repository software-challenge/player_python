use pyo3::{pyclass, pymethods};

use crate::plugin::actions::accelerate::Accelerate;
use crate::plugin::actions::Action;
use crate::plugin::actions::advance::Advance;
use crate::plugin::actions::push::Push;
use crate::plugin::actions::turn::Turn;
use crate::plugin::board::Board;
use crate::plugin::constants::PluginConstants;
use crate::plugin::coordinate::{CubeCoordinates, CubeDirection};
use crate::plugin::errors::ActionProblem;
use crate::plugin::errors::movement_error::MoveMistake;
use crate::plugin::field::FieldType;
use crate::plugin::r#move::Move;
use crate::plugin::ship::Ship;

#[pyclass]
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TeamPoints {
    ship_points: i32,
    coal_points: i32,
    finish_points: i32,
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
#[derive(Debug, Clone)]
pub struct GameState {
    pub board: Board,
    pub turn: i32,
    pub team_one: Ship,
    pub team_two: Ship,
    pub last_move: Option<Move>,
}

#[pymethods]
impl GameState {
    #[new]
    pub fn new(board: Board, turn: i32, team_one: Ship, team_two: Ship, last_move: Option<Move>) -> GameState {
        GameState {
            board,
            turn,
            team_one,
            team_two,
            last_move,
        }
    }

    pub fn current_ship(&self) -> Ship {
        if self.turn % 2 == 0 {
            self.determine_ahead_team().clone()
        } else {
            match &self.determine_ahead_team().opponent {
                Some(ship_arc_mutex) => {
                    let ship_mutex = ship_arc_mutex.lock().unwrap();
                    ship_mutex.clone()
                }
                None => panic!("No opponent ship found"),
            }
        }
    }

    pub fn other_ship(&self) -> Ship {
        match &self.current_ship().opponent {
            Some(ship_arc_mutex) => {
                let ship_mutex = ship_arc_mutex.lock().unwrap();
                ship_mutex.clone()
            }
            None => panic!("No other ship found"),
        }
    }

    pub fn determine_ahead_team(&self) -> Ship {
        let team_one: i32 =
            self.ship_advance_points(&self.team_one) * 100
                + self.team_one.speed
                + self.team_one.coal;

        let team_two: i32 =
            self.ship_advance_points(&self.team_two) * 100
                + self.team_two.speed
                + self.team_two.coal;

        if team_one >= team_two {
            self.team_one.clone()
        } else {
            self.team_two.clone()
        }
    }

    pub fn ship_advance_points(&self, ship: &Ship) -> i32 {
        let segment_index = self.board.segment_index(&ship.position).unwrap();
        let segment = &self.board.segments[segment_index];

        segment.global_to_local(ship.position.clone()).x() + 1
            + segment_index as i32 * PluginConstants::POINTS_PER_SEGMENT
    }

    pub fn calculate_points(&self, ship: &Ship) -> i32 {
        self.ship_advance_points(ship) + ship.passengers * PluginConstants::POINTS_PER_PASSENGER
    }

    pub fn is_current_ship_on_current(&self) -> bool {
        self.board.does_field_have_current(&self.current_ship().position)
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
    /// ```Rust
    /// let game_state = ...;  // Suppose we have a game state
    /// let move_ = ...;  // and a valid move
    /// match game_state.perform_move(move_) {
    ///    Ok(new_state) => {
    ///         println!("Move successful, new state: {:?}", new_state);
    ///    },
    ///    Err(e) => {
    ///         println!("Error performing move: {}", e);
    ///    },
    /// }
    /// ```
    fn perform_move(&self, move_: Move) -> Result<GameState, ActionProblem> {
        let mut new_state: GameState = self.clone();

        let actions: &Vec<Action> = &move_.actions;
        if actions.is_empty() {
            return Err(ActionProblem::MoveMistake(MoveMistake::NoActions));
        }

        for (i, action) in actions.iter().enumerate() {
            match action {
                Action::Push(_) if !new_state.must_push() => {
                    return Err(ActionProblem::MoveMistake(MoveMistake::PushActionRequired));
                }
                Action::Accelerate(_) if i != 0 => {
                    return Err(ActionProblem::MoveMistake(MoveMistake::FirstActionAccelerate));
                }
                Action::Advance(ad) => {
                    if new_state.board.get(&(new_state.current_ship().position * ad.distance)).unwrap().field_type == FieldType::Sandbank && i != 0 {
                        return Err(ActionProblem::MoveMistake(MoveMistake::SandBankEnd));
                    }
                }
                _ => {}
            }

            let result: Result<GameState, ActionProblem> = match action {
                Action::Accelerate(accelerate) => accelerate.perform(&new_state),
                Action::Advance(advance) => advance.perform(&new_state),
                Action::Push(push) => push.perform(&new_state),
                Action::Turn(turn) => turn.perform(&new_state),
            };

            match result {
                Ok(game_state) => new_state = game_state,
                Err(e) => return Err(e),
            }
        }

        let movement_points = self.current_ship().movement;

        if movement_points > 0 {
            return Err(ActionProblem::MoveMistake(MoveMistake::MovementPointsLeft));
        }
        if movement_points < 0 {
            return Err(ActionProblem::MoveMistake(MoveMistake::MovementPointsMissing));
        }

        new_state.last_move = Some(move_);
        new_state = new_state.advance_turn(&new_state);

        Ok(new_state.clone())
    }


    pub fn advance_turn(&self, state: &GameState) -> GameState {
        let mut new_state: GameState = state.clone();
        new_state.current_ship().free_acc = 1;
        new_state.current_ship().free_turns = 1;
        new_state.current_ship().movement = new_state.current_ship().speed;
        new_state.turn += 1;

        if !new_state.can_move() && !new_state.is_over() {
            new_state.last_move = None;
            return new_state.advance_turn(&new_state);
        }

        return new_state;
    }

    pub fn get_simple_moves(&self, max_coal: i32) -> Vec<Move> {
        let actions = self.get_actions(0, max_coal);

        let moves = actions.into_iter().map(|action| Move::new(vec![action])).collect();
        moves
    }

    pub fn get_actions(&self, rank: i32, max_coal: i32) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::new();
        if rank == 0 {
            actions.extend(
                self.get_accelerations(max_coal)
                    .into_iter()
                    .map(Action::Accelerate),
            );
        }
        actions.extend(
            self.get_turns(max_coal)
                .into_iter()
                .map(Action::Turn),
        );
        actions.extend(
            self.get_advances()
                .into_iter()
                .map(Action::Advance),
        );
        if rank != 0 {
            actions.extend(
                self.get_pushes()
                    .into_iter()
                    .map(Action::Push),
            );
        }
        actions
    }

    pub fn must_push(&self) -> bool {
        &self.current_ship().position == &self.other_ship().position
    }

    pub fn get_pushes(&self) -> Vec<Push> {
        if self.board.get(&self.current_ship().position).unwrap().field_type == FieldType::Sandbank
            || !self.must_push()
            || self.current_ship().movement < 1
        {
            Vec::new()
        } else {
            self.get_pushes_from(&self.current_ship().position, &self.current_ship().direction)
        }
    }

    pub fn get_pushes_from(
        &self,
        position: &CubeCoordinates,
        incoming_direction: &CubeDirection,
    ) -> Vec<Push> {
        CubeDirection::VALUES.iter().filter(|dir| {
            *dir != &incoming_direction.opposite()
                && self.board
                .get_field_in_direction(*dir, position)
                .map_or(false, |field| field.is_empty())
        })
            .map(|dir| Push::new((*dir).clone()))
            .collect()
    }

    pub fn get_turns(&self, max_coal: i32) -> Vec<Turn> {
        if self.board.get(&self.current_ship().position).unwrap().field_type == FieldType::Sandbank || self.must_push() {
            Vec::new()
        } else {
            let max_turn_count = (max_coal + self.current_ship().free_turns)
                .min(3)
                .max(0);

            (1..=max_turn_count)
                .flat_map(|i| {
                    vec![Turn::new(self.current_ship().direction.rotated_by(i)), Turn::new(self.current_ship().direction.rotated_by(-i))]
                })
                .take(5)
                .collect()
        }
    }

    pub fn get_advances(&self) -> Vec<Advance> {
        if self.current_ship().movement < 1 || self.must_push() {
            return Vec::new();
        }

        let sandbank: Vec<Advance> = self.check_sandbank_advances(&self.current_ship());

        if !sandbank.is_empty() {
            return sandbank;
        }
        let ship = self.current_ship();
        return self.check_advance_limit(&ship.position, &ship.direction, ship.movement);
    }

    pub fn check_sandbank_advances(&self, ship: &Ship) -> Vec<Advance> {
        let mut advances = Vec::new();
        if self.board.get(&ship.position).unwrap().field_type == FieldType::Sandbank {
            if self.check_advance_limit(&ship.position, &ship.direction, 1).len() > 1 {
                let advance1 = Advance::new(1);
                advances.push(advance1);
            }

            if self.check_advance_limit(&ship.position, &ship.direction.opposite(), 1).len() > 1 {
                let advance_minus1 = Advance::new(-1);
                advances.push(advance_minus1);
            }
        }
        advances
    }

    pub fn check_advance_limit(&self, start: &CubeCoordinates, direction: &CubeDirection, max_movement_points: i32) -> Vec<Advance> {
        let mut current_position = start.clone();
        let mut total_cost = 0;

        let max_movement = max_movement_points.min(PluginConstants::MAX_SPEED).max(0);

        let mut advances: Vec<Advance> = Vec::new();

        while total_cost < max_movement {
            current_position = current_position.clone() + direction.vector();
            let current_field = self.board.get(&current_position).unwrap();
            if current_field.is_empty() || current_field.field_type == FieldType::Sandbank {
                break;
            }
            if self.board.does_field_have_current(&current_position) && total_cost + 1 >= max_movement {
                break;
            }
            if self.team_one.position == current_position || self.team_two.position == current_position {
                total_cost += 1;
                advances.push(Advance::new(total_cost));
                break;
            }
            advances.push(Advance::new(total_cost));
            total_cost += 1;
        }
        advances
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

        let current_ship = self.current_ship();
        let lower = PluginConstants::MIN_SPEED - current_ship.speed;
        let upper = PluginConstants::MAX_SPEED - current_ship.speed;
        (1..=max_coal + current_ship.free_acc)
            .filter(|&i| i >= lower && i <= upper)
            .map(Accelerate::new)
            .collect()
    }

    pub fn can_move(&self) -> bool {
        true
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
        let condition1 = self.turn % 2 == 0 && (self.is_winner(&self.team_one) || self.is_winner(&self.team_two));

        // Bedingung 2: ein Spieler macht einen ungültigen Zug.
        // Das wird durch eine InvalidMoveException während des Spiels behandelt.

        // Bedingung 3: am Ende einer Runde liegt ein Dampfer mehr als 3 Spielsegmente zurück
        let condition3 = self.board.segment_distance(&self.team_one.position, &self.team_two.position).abs() > 3;

        // Bedingung 4: das Rundenlimit von 30 Runden ist erreicht
        let condition4 = self.turn / 2 >= PluginConstants::ROUND_LIMIT;

        // Bedingung 5: beide Spieler können sich nicht mehr bewegen
        let condition5 = self.last_move.is_none() && !self.can_move();

        condition1 || condition3 || condition4 || condition5
    }

    pub fn is_winner(&self, ship: &Ship) -> bool {
        ship.passengers > 1
            && self.board.effective_speed(ship) < 2
            && self.board.get(&ship.position).unwrap().field_type == FieldType::Goal
    }

    pub fn get_points_for_team(&self, ship: &Ship) -> TeamPoints {
        let finish_points = PluginConstants::FINISH_POINTS * (self.is_winner(ship) as i32);

        TeamPoints {
            ship_points: ship.points,
            coal_points: ship.coal * 2,
            finish_points,
        }
    }
}
