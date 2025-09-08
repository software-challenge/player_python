use pyo3::*;

use crate::plugin2026::{
    board::Board, errors::PiranhasError, field_type::FieldType, r#move::Move, rules_engine::RulesEngine, utils::{
        coordinate::Coordinate,
        direction::Direction
    }
};

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct GameState {
    #[pyo3(get, set)]
    pub board: Board,
    #[pyo3(get, set)]
    pub turn: usize,
    #[pyo3(get, set)]
    pub last_move: Option<Move>
}

#[pymethods]
impl GameState {
    #[new]
    pub fn new(board: Board, turn: usize, last_move: Option<Move>) -> Self {
        Self {
            board,
            turn,
            last_move,
        }
    }

    pub fn __str__(&self) -> String {self.to_string()}
    pub fn __repr__(&self) -> String {format!("{:?}", self)}

    pub fn possible_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let mut fish: Vec<Coordinate> = Vec::new();

        for f in RulesEngine::get_team_on_turn(self.turn).get_fish_types() {
            fish.append(&mut self.board.get_fields_by_type(f));
        }
        
        for f in fish {
            for d in Direction::all_directions() {
                moves.push(Move { start: f.clone(), direction: d });
            }
        }

        moves
            .into_iter()
            .filter(|m| RulesEngine::can_execute_move(&self.board, m).is_ok())
            .collect()
    }

    pub fn perform_move(&self, move_: &Move) -> Result<GameState, PyErr> {

        RulesEngine::can_execute_move(&self.board, move_)
            .map_err(|e| {
                let full_error = e.to_string();
                let clean_error = full_error.strip_prefix("PiranhasError:").unwrap_or(&full_error).trim();
                PiranhasError::new_err(format!("Cannot execute move: {}", clean_error))
            })?;
    
        let target = RulesEngine::target_position(&self.board, move_);
        let mut new_board = self.board.clone();
        new_board.map[target.y as usize][target.x as usize] = self.board.get_field(&move_.start).unwrap();
        new_board.map[move_.start.y as usize][move_.start.x as usize] = FieldType::Empty;

        let new_state = GameState {
            board: new_board,
            turn: self.turn + 1,
            last_move: Some(move_.clone())
        };

        Ok(new_state)
    }

    pub fn perform_move_mut(&mut self, move_: &Move) -> Result<(), PyErr> {

        RulesEngine::can_execute_move(&self.board, move_)
            .map_err(|e| {
                let full_error = e.to_string();
                let clean_error = full_error.strip_prefix("PiranhasError:").unwrap_or(&full_error).trim();
                PiranhasError::new_err(format!("Cannot execute move: {}", clean_error))
            })?;
    
        let target = RulesEngine::target_position(&self.board, move_);
        let mut new_board = self.board.clone();
        new_board.map[target.y as usize][target.x as usize] = self.board.get_field(&move_.start).unwrap();
        new_board.map[move_.start.y as usize][move_.start.x as usize] = FieldType::Empty;

        self.board = new_board;
        self.turn += 1;
        self.last_move = Some(move_.clone());

        Ok(())
    }
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Zug: {}\nSpielbrett:{}Letzter Zug: {:?}",
            self.turn,
            self.board,
            self.last_move
        )
    }
}