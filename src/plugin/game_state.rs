use itertools::Itertools;
use pyo3::*;

use super::action::advance::Advance;
use super::action::card::Card;
use super::action::eat_salad::EatSalad;
use super::action::exchange_carrots::ExchangeCarrots;
use super::action::fall_back::FallBack;
use super::action::Action;
use super::board::Board;
use super::constants::PluginConstants;
use super::field::Field;
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

        fn update_carrots(player: &mut Hare, opponent_position: usize, board: &Board) {
            match board.get_field(player.position) {
                Some(Field::Position1) if player.position > opponent_position => {
                    player.carrots += 10
                }
                Some(Field::Position2) if player.position > opponent_position => {
                    player.carrots += 30
                }
                _ => {}
            }
        }

        update_carrots(
            &mut new_state.player_one,
            new_state.player_two.position,
            &new_state.board,
        );
        update_carrots(
            &mut new_state.player_two,
            new_state.player_one.position,
            &new_state.board,
        );

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

    pub fn is_over(&self) -> bool {
        let player_one_in_goal = self.player_one.is_in_goal();
        let player_two_in_goal = self.player_two.is_in_goal();
        let both_had_last_chance = self.turn % 2 == 0;
        let rounds_exceeded = self.turn / 2 == PluginConstants::ROUND_LIMIT;

        player_one_in_goal || player_two_in_goal && both_had_last_chance || rounds_exceeded
    }

    pub fn possible_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        moves.append(&mut self.possible_advance_moves());
        moves.append(&mut self.possible_eat_salad_moves());
        moves.append(&mut self.possible_exchange_carrots_moves());
        moves.append(&mut self.possible_fall_back_moves());

        moves
    }

    fn possible_exchange_carrots_moves(&self) -> Vec<Move> {
        let moves: Vec<Move> = vec![
            Move::new(Action::ExchangeCarrots(ExchangeCarrots::new(-10))),
            Move::new(Action::ExchangeCarrots(ExchangeCarrots::new(10))),
        ];

        moves
            .into_iter()
            .filter(|m| m.perform(&mut self.clone()).is_ok())
            .collect()
    }

    fn possible_fall_back_moves(&self) -> Vec<Move> {
        let moves: Vec<Move> = vec![Move::new(Action::FallBack(FallBack::new()))];

        moves
            .into_iter()
            .filter(|m| m.perform(&mut self.clone()).is_ok())
            .collect()
    }

    fn possible_eat_salad_moves(&self) -> Vec<Move> {
        let moves: Vec<Move> = vec![Move::new(Action::EatSalad(EatSalad::new()))];

        moves
            .into_iter()
            .filter(|m| m.perform(&mut self.clone()).is_ok())
            .collect()
    }

    fn possible_advance_moves(&self) -> Vec<Move> {
        let current_player = self.clone_current_player();
        let max_distance =
            (((-1.0 + (1 + 8 * current_player.carrots) as f64).sqrt()) / 2.0) as usize;

        let mut moves = Vec::new();

        for distance in 1..=max_distance {
            if let Some(Field::Hare) = self.board.get_field(current_player.position + distance) {
                for k in 0..current_player.cards.len() {
                    for combination in current_player.cards.iter().combinations(k) {
                        moves.push(Move::new(Action::Advance(Advance::new(
                            distance,
                            combination.iter().map(|&c| *c).collect(),
                        ))));
                    }
                }
            }

            if self.board.get_field(current_player.position + distance) == Some(Field::Market) {
                let cards = vec![
                    Card::FallBack,
                    Card::HurryAhead,
                    Card::EatSalad,
                    Card::SwapCarrots,
                ];
                for card in cards {
                    moves.push(Move::new(Action::Advance(Advance::new(
                        distance,
                        vec![card],
                    ))));
                }
            }

            moves.push(Move::new(Action::Advance(Advance::new(distance, vec![]))));
        }

        moves
            .into_iter()
            .filter(|m| m.perform(&mut self.clone()).is_ok())
            .collect()
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
