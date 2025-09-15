use pyo3::*;

use crate::plugin2026::{
    board::Board, errors::PiranhasError, field_type::FieldType, r#move::Move, rules_engine::RulesEngine, utils::{
        coordinate::Coordinate,
        direction::Direction
    }
};

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    fn __str__(&self) -> String {self.to_string()}
    fn __repr__(&self) -> String {format!("{:?}", self)}
    fn __eq__(&self, other: &GameState) -> bool {self == other}
    fn __ne__(&self, other: &GameState) -> bool {self != other}
    fn deepcopy(&self) -> GameState {self.clone()}

    pub fn set_board_field(&mut self, position: &Coordinate, field: FieldType) -> Result<(), PyErr> {
        let x = position.x as usize;
        let y = position.y as usize;

        if y >= self.board.map.len() || x >= self.board.map[0].len() {
            return Err(PiranhasError::new_err("Position not in bounds of map"));
        }

        self.board.map[y][x] = field;

        Ok(())
    }

    pub fn possible_moves_for(&self, start: &Coordinate) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for d in Direction::all_directions() {
            moves.push(Move { start: start.to_owned(), direction: d });
        }

        moves
            .into_iter()
            .filter(|m| RulesEngine::can_execute_move(&self.board, m).is_ok())
            .collect()
    }

    pub fn possible_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let mut fish: Vec<Coordinate> = Vec::new();

        for f in RulesEngine::get_team_on_turn(self.turn).get_fish_types() {
            fish.append(&mut self.board.get_fields_by_type(f));
        }
        
        for f in fish {
            moves.extend(self.possible_moves_for(&f));
        }

        moves
    }

    pub fn perform_move(&self, move_: &Move) -> Result<GameState, PyErr> {

        let mut new_game_state = self.clone();
        new_game_state.perform_move_mut(move_)?;

        Ok(new_game_state)
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