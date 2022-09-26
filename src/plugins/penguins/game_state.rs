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

use crate::plugins::penguins::board::{BitBoard, Board};
use crate::plugins::penguins::r#move::Move;
use crate::plugins::penguins::team::Team;

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct Round {
    #[pyo3(get, set)]
    pub round: i32,
    #[pyo3(get, set)]
    pub turn: i32,
}

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct Score {
    #[pyo3(get, set)]
    pub team_one: Team,
    #[pyo3(get, set)]
    pub team_two: Team,
}

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct GameState {
    #[pyo3(get, set)]
    pub start_team: Team,
    #[pyo3(get, set)]
    pub board: Board,
    #[pyo3(get, set)]
    pub round: Round,
    #[pyo3(get, set)]
    pub score: Score,
    #[pyo3(get, set)]
    pub last_move: Option<Move>,

}

#[pymethods]
impl GameState {
    #[new]
    pub(crate) fn new(start_team: Team, board: Board, round: Round, score: Score, last_move: Option<Move>) -> Self {
        GameState {
            start_team,
            board,
            round,
            score,
            last_move,
        }
    }

    fn get_team(&self) -> Team {
        match self.round.turn % 2 {
            0 => self.start_team.clone(),
            1 => self.start_team.opponent(),
            _ => panic!("Invalid turn number"),
        }
    }

    fn get_opponent(&self) -> Team {
        self.get_team().opponent()
    }

    fn get_possible_moves(&self) -> Vec<Move> {
        let team = self.get_team();
        let mut moves = Vec::new();
        if self.board.get_team_penguins(team.clone()).len() < 4 {
            let fish_1 = self.board.fish_1.clone();
            let possible_fields: BitBoard = BitBoard(fish_1.0 &
                !self.board.fish_0.0 &
                !self.board.fish_2.0 &
                !self.board.fish_3.0 &
                !self.board.fish_4.0 &
                !self.board.one.0 &
                !self.board.two.0);
            for coordinates in self.board.get_fields(possible_fields) {
                moves.push(Move::new(None, coordinates, team.clone()));
            }
        } else {
            for penguin in self.board.get_team_penguins(team.clone()) {
                let mut moves_from = self.board.get_moves_from(penguin.coordinate,
                                                               team.clone());
                for move_ in moves_from {
                    moves.push(move_);
                }
            }
        }
        moves
    }
}