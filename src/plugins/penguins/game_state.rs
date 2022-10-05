/*
 A `GameState` contains all information, that describes the game state at a given time, that is, between two game
       moves.

       This includes:
            - the board
            - a consecutive turn number (round & turn) and who's turn it is
            - the team that has started the game
            - the number of fishes each player has
            - the last move made

       The `GameState` is thus the central object through which all essential information of the current game can be
       accessed.

       Therefore, for easier handling, it offers further aids, such as:
            - a method to calculate available moves
            - a method to perform a move for simulating future game states

       The game server sends a new copy of the `GameState` to both participating players after each completed move,
       describing the then current state.
       */

use pyo3::prelude::*;

use crate::plugins::penguins::board::Board;
use crate::plugins::penguins::r#move::Move;
use crate::plugins::penguins::team::Team;

use super::coordinate::HexCoordinate;
use super::team::TeamEnum;

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct WelcomeMessage {
    #[pyo3(get)]
    pub team: TeamEnum,
}

#[pymethods]
impl WelcomeMessage {
    #[new]
    pub fn new(team: TeamEnum) -> Self {
        WelcomeMessage { team }
    }
}

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct Progress {
    #[pyo3(get, set)]
    pub round: i32,
    #[pyo3(get, set)]
    pub turn: i32,
}

#[pymethods]
impl Progress {
    #[new]
    pub fn new(round: i32, turn: i32) -> Self {
        Progress {
            round,
            turn,
        }
    }
}

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct Score {
    #[pyo3(get, set)]
    pub team_one: Team,
    #[pyo3(get, set)]
    pub team_two: Team,
}

#[pymethods]
impl Score {
    #[new]
    pub fn new(team_one: Team, team_two: Team) -> Self {
        Score {
            team_one,
            team_two,
        }
    }
}

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct GameState {
    #[pyo3(get, set)]
    pub welcome_message: WelcomeMessage,
    #[pyo3(get, set)]
    pub start_team: Team,
    #[pyo3(get, set)]
    pub board: Board,
    #[pyo3(get, set)]
    pub round: Progress,
    #[pyo3(get, set)]
    pub score: Score,
    #[pyo3(get, set)]
    pub last_move: Option<Move>,
}

#[pymethods]
impl GameState {
    #[new]
    pub(crate) fn new(welcome_message: WelcomeMessage, start_team: Team, board: Board,
                      round: Progress, score: Score, last_move: Option<Move>) -> Self {
        GameState {
            welcome_message,
            start_team,
            board,
            round,
            score,
            last_move,
        }
    }

    fn get_team(&self) -> Team {
        let team_one_moves = self.possible_moves(TeamEnum::ONE);
        let team_two_moves = self.possible_moves(TeamEnum::TWO);
        if team_one_moves.is_empty() && !team_two_moves.is_empty() {
            match self.start_team.name {
                TeamEnum::ONE => self.start_team.opponent(),
                TeamEnum::TWO => self.start_team.clone(),
            }
        } else if team_two_moves.is_empty() && !team_one_moves.is_empty() {
            match self.start_team.name {
                TeamEnum::ONE => self.start_team.clone(),
                TeamEnum::TWO => self.start_team.opponent(),
            }
        } else {
            match self.round.turn % 2 {
                0 => self.start_team.clone(),
                1 => self.start_team.opponent(),
                _ => panic!("Invalid turn number"),
            }
        }
    }

    fn get_opponent(&self) -> Team {
        self.get_team().opponent()
    }

    fn possible_moves(&self, team: TeamEnum) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let penguins: u64 = if team == TeamEnum::ONE { self.board.board.one } else { self.board.board.two };
        if penguins.count_ones() < 4 {
            let destinations: Vec<HexCoordinate> = self.board.board.get_coordinates(self.board.board.fish_1);
            moves.extend(destinations.iter().map(|c| Move::new(None, c.clone(), team.clone())));
        } else {
            if team == TeamEnum::ONE {
                let from: Vec<HexCoordinate> = self.board.board.get_coordinates(self.board.board.one);
                for coordinate in from {
                    for possible_moves in self.board.get_moves_from(coordinate, team.clone()) {
                        moves.push(possible_moves);
                    }
                }
            } else {
                let from: Vec<HexCoordinate> = self.board.board.get_coordinates(self.board.board.two);
                for coordinate in from {
                    for possible_moves in self.board.get_moves_from(coordinate, team.clone()) {
                        moves.push(possible_moves);
                    }
                }
            }
        }
        moves
    }
}