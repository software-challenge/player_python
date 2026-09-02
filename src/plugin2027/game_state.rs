use pyo3::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::plugin2027::{
    board::Board, color::Color, r#move::Move, piece_shape::PieceShape,
    utils::{constants::Constants, game_rule_logic::GameRuleLogic, team::TeamEnum},
};

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    #[pyo3(get, set)]
    pub turn: usize,
    #[pyo3(get, set)]
    pub last_move: Option<Move>,
    #[pyo3(get, set)]
    pub board: Board,
    #[pyo3(get, set)]
    pub start_piece: PieceShape,
    #[pyo3(get, set)]
    pub last_move_mono: HashMap<Color, bool>,
    blue_shapes: HashSet<PieceShape>,
    yellow_shapes: HashSet<PieceShape>,
    red_shapes: HashSet<PieceShape>,
    green_shapes: HashSet<PieceShape>,
    valid_colors: Vec<Color>,
    round: usize,
}

#[pymethods]
impl GameState {
    #[new]
    #[pyo3(signature = (turn=0, last_move=None, board=None, start_piece=PieceShape::Mono, last_move_mono=None, blue_shapes=None, yellow_shapes=None, red_shapes=None, green_shapes=None, valid_colors=None))]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        turn: usize,
        last_move: Option<Move>,
        board: Option<Board>,
        start_piece: PieceShape,
        last_move_mono: Option<HashMap<Color, bool>>,
        blue_shapes: Option<Vec<PieceShape>>,
        yellow_shapes: Option<Vec<PieceShape>>,
        red_shapes: Option<Vec<PieceShape>>,
        green_shapes: Option<Vec<PieceShape>>,
        valid_colors: Option<Vec<Color>>,
    ) -> Self {
        Self {
            turn,
            last_move,
            board: board.unwrap_or_else(|| Board::new(None)),
            start_piece,
            last_move_mono: last_move_mono.unwrap_or_default(),
            blue_shapes: blue_shapes.map(|v| v.into_iter().collect()).unwrap_or_else(|| PieceShape::all().into_iter().collect()),
            yellow_shapes: yellow_shapes.map(|v| v.into_iter().collect()).unwrap_or_else(|| PieceShape::all().into_iter().collect()),
            red_shapes: red_shapes.map(|v| v.into_iter().collect()).unwrap_or_else(|| PieceShape::all().into_iter().collect()),
            green_shapes: green_shapes.map(|v| v.into_iter().collect()).unwrap_or_else(|| PieceShape::all().into_iter().collect()),
            valid_colors: valid_colors.unwrap_or_else(|| vec![Color::BLUE, Color::YELLOW, Color::RED, Color::GREEN]),
            round: GameRuleLogic::round_from_turn(turn),
        }
    }

    fn round_from_turn(&self, turn: usize) -> usize {
        1 + turn / Constants::COLORS
    }

    pub fn round(&self) -> usize {
        self.round
    }

    pub fn undeployed_piece_shapes(&self, color: Color) -> Vec<PieceShape> {
        self.shapes_for(color).iter().copied().collect()
    }

    pub fn remove_undeployed_piece(&mut self, color: Color, shape: PieceShape) -> bool {
        self.shapes_for_mut(color).remove(&shape)
    }

    pub fn current_color(&self) -> Color {
        let ordered = [Color::BLUE, Color::YELLOW, Color::RED, Color::GREEN];
        ordered[self.turn % Constants::COLORS]
    }

    pub fn has_valid_colors(&self) -> bool {
        !self.valid_colors.is_empty()
    }

    pub fn is_valid_color(&self, color: Color) -> bool {
        self.valid_colors.contains(&color)
    }

    pub fn remove_active_color(&mut self) -> bool {
        let color = self.current_color();
        self.valid_colors.retain(|c| *c != color);
        self.advance(1)
    }

    pub fn advance(&mut self, turns: usize) -> bool {
        if !self.has_valid_colors() {
            return false;
        }
        self.turn += turns;
        while !self.is_valid_color(self.current_color()) {
            self.turn += 1;
        }
        self.round = self.round_from_turn(self.turn);
        true
    }

    pub fn is_over(&self) -> bool {
        !self.has_valid_colors() || self.round >= Constants::ROUND_LIMIT
    }

    pub fn possible_moves(&self) -> Vec<Move> {
        let pieces = GameRuleLogic::get_filtered_possible_moves(self);
        if pieces.is_empty() {
            vec![Move::skip_move(self.current_color())]
        } else {
            pieces.into_iter().map(Move::set_move).collect()
        }
    }

    pub fn get_points_for_color(&self, color: Color) -> usize {
        let pieces = self.undeployed_piece_shapes(color);
        let last_mono = *self.last_move_mono.get(&color).unwrap_or(&false);
        GameRuleLogic::get_points_from_undeployed(pieces, last_mono)
    }

    pub fn get_points_for_team(&self, team: TeamEnum) -> usize {
        [Color::BLUE, Color::YELLOW, Color::RED, Color::GREEN]
            .iter()
            .filter(|c| c.team() == team)
            .map(|c| self.get_points_for_color(*c))
            .sum()
    }

    pub fn win_condition(&self) -> Option<TeamEnum> {
        let one = self.get_points_for_team(TeamEnum::One);
        let two = self.get_points_for_team(TeamEnum::Two);
        if one > two {
            Some(TeamEnum::One)
        } else if two > one {
            Some(TeamEnum::Two)
        } else {
            None // Unentschieden
        }
    }
}

impl GameState {
    fn shapes_for(&self, color: Color) -> &HashSet<PieceShape> {
        match color {
            Color::BLUE => &self.blue_shapes,
            Color::YELLOW => &self.yellow_shapes,
            Color::RED => &self.red_shapes,
            Color::GREEN => &self.green_shapes,
        }
    }

    fn shapes_for_mut(&mut self, color: Color) -> &mut HashSet<PieceShape> {
        match color {
            Color::BLUE => &mut self.blue_shapes,
            Color::YELLOW => &mut self.yellow_shapes,
            Color::RED => &mut self.red_shapes,
            Color::GREEN => &mut self.green_shapes,
        }
    }
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GameState(turn={}, currentColor={})",
            self.turn,
            self.current_color()
        )
    }
}