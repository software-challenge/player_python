use pyo3::prelude::*;
use std::collections::HashSet;

use crate::plugin2027::{
    board::Board, color::Color, r#move::Move, piece::Piece, piece_shape::PieceShape,
    field::Field, game_state::GameState,
    utils::{constants::Constants, coordinate::Coordinate, coordinate_helper::CoordinateSetExt},
    errors::BlokusMoveMistake,
};

#[allow(clippy::identity_op)]
pub const SUM_MAX_SQUARES: usize = 1 * 1 + 1 * 2 + 2 * 3 + 5 * 4 + 12 * 5; // = 89

#[pyclass]
pub struct GameRuleLogic;

#[pymethods]
impl GameRuleLogic {

    #[staticmethod]
    pub fn round_from_turn(turn: usize) -> usize {
        1 + turn / Constants::COLORS
    }

    #[staticmethod]
    pub fn get_points_from_undeployed(undeployed: Vec<PieceShape>, mono_last: bool) -> usize {
        if undeployed.is_empty() {
            SUM_MAX_SQUARES + 15 + if mono_last { 5 } else { 0 }
        } else {
            SUM_MAX_SQUARES
                - undeployed
                    .iter()
                    .map(|shape| shape.coordinates().len())
                    .sum::<usize>()
        }
    }

    #[staticmethod]
    pub fn perform_move(game_state: &mut GameState, mv: &Move) -> PyResult<()> {
        if Constants::VALIDATE_MOVE {
            Self::validate_move_color(game_state, mv)?;
        }
        match mv {
            Move::SkipMove { .. } => Self::perform_skip_move(game_state)?,
            Move::SetMove { piece } => Self::perform_set_move(game_state, piece)?,
        }
        game_state.advance(1);
        game_state.last_move = Some(mv.clone());
        Ok(())
    }

    #[staticmethod]
    pub fn validate_move_color(game_state: &GameState, mv: &Move) -> PyResult<()> {
        if mv.get_color() != game_state.current_color() {
            return Err(BlokusMoveMistake::WrongColor.to_py_err());
        }
        Ok(())
    }

    #[staticmethod]
    pub fn validate_set_move(game_state: &GameState, piece: &Piece) -> PyResult<()> {
        Self::validate_move_color(game_state, &Move::SetMove { piece: piece.clone() })?;
        Self::validate_shape(game_state, piece.kind, piece.color)?;
        Self::validate_set_move_on_board(&game_state.board, piece)?;

        if Self::is_first_move(game_state) {
            if !piece.coordinates().iter().any(|c| Self::is_on_border(*c)) {
                return Err(BlokusMoveMistake::NotOnBorder.to_py_err());
            }
        } else {
            let touches_corner = piece.coordinates().iter().any(|c| {
                Self::corners_on_color(&game_state.board, &Field { coordinate: *c, content: piece.color.to_field_content() })
            });
            if !touches_corner {
                return Err(BlokusMoveMistake::NoSharedCorner.to_py_err());
            }
        }
        Ok(())
    }

    #[staticmethod]
    pub fn perform_set_move(game_state: &mut GameState, piece: &Piece) -> PyResult<()> {
        if Constants::VALIDATE_MOVE {
            Self::validate_set_move(game_state, piece)?;
        }
        Self::perform_set_move_on_board(&mut game_state.board, piece);
        game_state.remove_undeployed_piece(piece.color, piece.kind);

        if game_state.undeployed_piece_shapes(piece.color).is_empty() {
            game_state
                .last_move_mono
                .insert(piece.color, piece.kind == PieceShape::Mono);
        }
        Ok(())
    }

    #[staticmethod]
    pub fn validate_shape(game_state: &GameState, shape: PieceShape, color: Color) -> PyResult<()> {
        if Self::is_first_move(game_state) {
            if shape != game_state.start_piece {
                return Err(BlokusMoveMistake::WrongShape.to_py_err());
            }
        } else if !game_state.undeployed_piece_shapes(color).contains(&shape) {
            return Err(BlokusMoveMistake::DuplicateShape.to_py_err());
        }
        Ok(())
    }

    #[staticmethod]
    pub fn is_valid_set_move(game_state: &GameState, piece: &Piece) -> bool {
        Self::validate_set_move(game_state, piece).is_ok()
    }

    #[staticmethod]
    pub fn validate_set_move_on_board(board: &Board, piece: &Piece) -> PyResult<()> {
        for coord in &piece.coordinates() {
            if !Board::contains(*coord) {
                return Err(BlokusMoveMistake::OutOfBounds.to_py_err());
            }
            if board.is_obstructed(*coord) {
                return Err(BlokusMoveMistake::Obstructed.to_py_err());
            }
            let field = Field { coordinate: *coord, content: piece.color.to_field_content() };
            if Self::borders_on_color(board, &field) {
                return Err(BlokusMoveMistake::TouchesSameColor.to_py_err());
            }
        }
        Ok(())
    }

    #[staticmethod]
    pub fn validate_skip_move(game_state: &GameState) -> PyResult<()> {
        if Self::is_first_move(game_state) {
            return Err(BlokusMoveMistake::SkipFirstTurn.to_py_err());
        }
        Ok(())
    }

    #[staticmethod]
    pub fn perform_skip_move(game_state: &GameState) -> PyResult<()> {
        Self::validate_skip_move(game_state)
    }

    #[staticmethod]
    pub fn borders_on_color(board: &Board, field: &Field) -> bool {
        field
            .coordinate
            .neighbors()
            .iter()
            .any(|n| match board.get_content(*n) {
                Some(content) => content == field.content && !field.content.is_empty(),
                None => false,
            })
    }

    #[staticmethod]
    pub fn corners_on_color(board: &Board, field: &Field) -> bool {
        field
            .coordinate
            .diagonal_neighbors()
            .iter()
            .any(|n| match board.get_content(*n) {
                Some(content) => content == field.content && !field.content.is_empty(),
                None => false,
            })
    }

    #[staticmethod]
    pub fn is_on_border(position: Coordinate) -> bool {
        let max = Constants::BOARD_LENGTH as isize - 1;
        position.x == 0 || position.x == max || position.y == 0 || position.y == max
    }

    #[staticmethod]
    pub fn is_first_move(game_state: &GameState) -> bool {
        game_state.undeployed_piece_shapes(game_state.current_color()).len() == Constants::TOTAL_PIECE_SHAPES
    }

    #[staticmethod]
    pub fn get_random_start_pentomino() -> PieceShape {
        use rand::seq::IndexedRandom;
        let pentominoes: Vec<PieceShape> = PieceShape::all()
            .into_iter()
            .filter(|shape| shape.coordinates().len() == 5)
            .collect();
        *pentominoes.choose(&mut rand::rng()).unwrap()
    }

    #[staticmethod]
    pub fn remove_invalid_colors(game_state: &mut GameState) {
        if !game_state.has_valid_colors() {
            return;
        }
        let no_valid_move = Self::get_all_possible_moves(game_state)
            .iter()
            .all(|piece| !Self::is_valid_set_move(game_state, piece));

        if no_valid_move {
            game_state.remove_active_color();
            Self::remove_invalid_colors(game_state);
        }
    }

    #[staticmethod]
    pub fn get_all_possible_moves(game_state: &GameState) -> Vec<Piece> {
        if Self::is_first_move(game_state) {
            Self::get_possible_start_moves(game_state, false)
        } else {
            Self::get_possible_moves(game_state)
        }
    }

    #[staticmethod]
    pub fn get_filtered_possible_moves(game_state: &GameState) -> Vec<Piece> {
        if Self::is_first_move(game_state) {
            Self::get_possible_start_moves(game_state, true)
        } else if game_state.round() <= 5 {
            Self::get_pentomino_moves(game_state)
        } else {
            Self::get_possible_moves(game_state)
        }
    }

    #[staticmethod]
    pub fn get_possible_start_moves(game_state: &GameState, filter: bool) -> Vec<Piece> {
        let mut moves = Vec::new();
        let kind = game_state.start_piece;
        let color = game_state.current_color();

        for (shape, rotation, is_flipped) in kind.variants() {
            let area = shape.area();
            let mut border_coords: Vec<Coordinate> = Vec::new();

            let top_left_half = !filter || color == Color::BLUE || color == Color::YELLOW;
            let bottom_right_half = !filter || color == Color::RED || color == Color::GREEN;

            let board_len = Constants::BOARD_LENGTH as isize;

            if top_left_half {
                for x in 0..(board_len - area.delta_x - 1) {
                    border_coords.push(Coordinate::new(x, 0));
                }
                for y in 1..(board_len - area.delta_y) {
                    border_coords.push(Coordinate::new(0, y));
                }
            }
            if bottom_right_half {
                for y in 0..(board_len - area.delta_y - 1) {
                    border_coords.push(Coordinate::new(board_len - area.delta_x - 1, y));
                }
                for x in 1..(board_len - area.delta_x) {
                    border_coords.push(Coordinate::new(x, board_len - area.delta_y - 1));
                }
            }

            for position in border_coords {
                let piece = Piece::new(color, kind, rotation, is_flipped, position);
                if Self::is_valid_set_move(game_state, &piece) {
                    moves.push(piece);
                }
            }
        }
        moves
    }

    #[staticmethod]
    pub fn get_possible_moves(game_state: &GameState) -> Vec<Piece> {
        let color = game_state.current_color();
        let valid_fields = Self::get_valid_fields(&game_state.board, color);
        let mut moves = Vec::new();
        for shape in game_state.undeployed_piece_shapes(color) {
            moves.extend(Self::get_possible_moves_for_shape(game_state, shape, valid_fields.clone()));
        }
        moves
    }

    #[staticmethod]
    pub fn get_pentomino_moves(game_state: &GameState) -> Vec<Piece> {
        let color = game_state.current_color();
        let valid_fields = Self::get_valid_fields(&game_state.board, color);
        let mut moves = Vec::new();
        for shape in game_state.undeployed_piece_shapes(color) {
            if shape.coordinates().len() == 5 {
                moves.extend(Self::get_possible_moves_for_shape(game_state, shape, valid_fields.clone()));
            }
        }
        moves
    }

    #[staticmethod]
    pub fn get_possible_moves_for_shape(
        game_state: &GameState,
        shape: PieceShape,
        valid_fields: HashSet<Coordinate>,
    ) -> Vec<Piece> {
        let mut moves: HashSet<Piece> = HashSet::new();

        if Self::is_first_move(game_state) {
            if shape == game_state.start_piece {
                return Self::get_possible_start_moves(game_state, false);
            } else {
                return Vec::new();
            }
        }

        let color = game_state.current_color();
        for field in &valid_fields {
            for (_variant_shape, rotation, is_flipped) in shape.variants() {
                let area = shape.transform(rotation, is_flipped).area();
                for x in (field.x - area.delta_x)..=field.x {
                    for y in (field.y - area.delta_y)..=field.y {
                        let position = Coordinate::new(x, y);
                        let piece = Piece::new(color, shape, rotation, is_flipped, position);
                        if Self::is_valid_set_move(game_state, &piece) {
                            moves.insert(piece);
                        }
                    }
                }
            }
        }
        moves.into_iter().collect()
    }

    #[staticmethod]
    pub fn get_valid_fields(board: &Board, color: Color) -> HashSet<Coordinate> {
        let colored_fields = Self::get_colored_fields(board, color);
        colored_fields
            .iter()
            .flat_map(|c| c.diagonal_neighbors())
            .filter(|corner| {
                Board::contains(*corner)
                    && board.get_content(*corner).is_some_and(|c| c.is_empty())
                    && corner
                        .neighbors()
                        .iter()
                        .all(|n| {
                            !Board::contains(*n)
                                || board.get_content(*n) != Some(color.to_field_content())
                        })
            })
            .collect()
    }

    #[staticmethod]
    pub fn get_colored_fields(board: &Board, color: Color) -> HashSet<Coordinate> {
        let board_len = Constants::BOARD_LENGTH as isize;
        let mut colored = HashSet::new();
        for x in 0..board_len {
            for y in 0..board_len {
                let pos = Coordinate::new(x, y);
                if board.get_content(pos) == Some(color.to_field_content()) {
                    colored.insert(pos);
                }
            }
        }
        colored
    }
}

impl GameRuleLogic {
    fn perform_set_move_on_board(board: &mut Board, piece: &Piece) {
        for coord in &piece.coordinates() {
            board.set_content(*coord, piece.color.to_field_content());
        }
    }
}