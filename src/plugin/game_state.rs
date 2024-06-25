use pyo3::*;

use super::board::Board;
use super::hare::Hare;
use super::r#move::Move;

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct GameState {
    #[pyo3(get, set)]
    pub board: Board,
    #[pyo3(get, set)]
    pub turn: usize,
    player_one: Hare,
    player_two: Hare,
}

#[pymethods]
impl GameState {
    #[new]
    pub fn new(board: Board, turn: usize, player_one: Hare, player_two: Hare) -> Self {
        Self {
            board,
            turn,
            player_one,
            player_two,
        }
    }

    pub fn perform_move(&self, r#move: &Move) -> Result<GameState, PyErr> {
        let mut new_state = self.clone();
        r#move.perform(&mut new_state)?;
        new_state.turn += 1;
        Ok(new_state)
    }

    pub fn clone_current_player(&self) -> Hare {
        if self.turn % 2 == 0 {
            self.player_one.clone()
        } else {
            self.player_two.clone()
        }
    }

    pub fn clone_other_player(&self) -> Hare {
        if self.turn % 2 != 0 {
            self.player_one.clone()
        } else {
            self.player_two.clone()
        }
    }

    pub fn update_player(&mut self, player: Hare) {
        if player.team == self.player_one.team {
            self.player_one = player;
        } else {
            self.player_two = player;
        }
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GameState(board={}, turn={}, player_one={}, player_two={})",
            self.board, self.turn, self.player_one, self.player_two
        )
    }
}
