use itertools::Itertools;
use pyo3::*;

use super::action::advance::Advance;
use super::action::eat_salad::EatSalad;
use super::action::exchange_carrots::ExchangeCarrots;
use super::action::fall_back::FallBack;
use super::action::card::Card;
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
    #[pyo3(get, set)]
    pub last_move: Option<Move>
}

#[pymethods]
impl GameState {
    #[new]
    pub fn new(board: Board, turn: usize, player_one: Hare, player_two: Hare, last_move: Option<Move>) -> Self {
        Self {
            board,
            turn,
            player_one,
            player_two,
            last_move,
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
                Some(Field::Position2) if player.position < opponent_position => {
                    player.carrots += 30
                }
                _ => {}
            }
        }

        if new_state.turn % 2 == 0 {
            update_carrots(
                &mut new_state.player_one,
                new_state.player_two.position,
                &new_state.board,
            );
        } else {
            update_carrots(
                &mut new_state.player_two,
                new_state.player_one.position,
                &new_state.board,
            );
        }

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

    pub fn possible_moves_old(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        moves.append(&mut self.possible_advance_moves_old());
        moves.append(&mut self.possible_eat_salad_moves());
        moves.append(&mut self.possible_exchange_carrots_moves());
        moves.append(&mut self.possible_fall_back_moves());

        moves
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

    fn possible_advance_moves_old(&self) -> Vec<Move> {

        let current_player = self.clone_current_player();
        let max_distance =
            (((-1.0 + (1 + 8 * current_player.carrots) as f64).sqrt()) / 2.0) as usize;

        let mut moves = Vec::new();

        for distance in 1..=max_distance {
            for card in PluginConstants::MARKET_SELECTION {
                moves.push(Move::new(Action::Advance(Advance::new(
                    distance,
                    vec![card],
                ))));
            }

            for k in 0..=current_player.cards.len() {
                for permutation in current_player.cards.iter().permutations(k).unique() {
                    moves.push(Move::new(Action::Advance(Advance::new(
                        distance,
                        permutation.iter().map(|&c| *c).collect(),
                    ))));

                    for card in PluginConstants::MARKET_SELECTION {
                        let mut extended_permutaion = permutation.clone();
                        extended_permutaion.push(&card);
                        moves.push(Move::new(Action::Advance(Advance::new(
                            distance,
                            extended_permutaion.iter().map(|&c| *c).collect(),
                        ))));
                    }
                }
            }

            moves.push(Move::new(Action::Advance(Advance::new(distance, vec![]))));
        }

        moves
            .into_iter()
            .unique()
            .filter(|m| m.perform(&mut self.clone()).is_ok())
            .collect()
    }

    fn possible_advance_moves(&self) -> Vec<Move> {

        let current_player = self.clone_current_player();
        let max_distance =
            (((-1.0 + (1 + 8 * current_player.carrots) as f64).sqrt()) / 2.0) as usize;
        

        let mut card_permutations = Vec::new();

        for k in 0..=current_player.cards.len() {
            for permutation in current_player.cards.iter().permutations(k).unique() {

                // change permutation cards to owned
                let owned_permutation: Vec<Card> = permutation.iter().map(|&card| *card).collect();

                // if minimum one card in permutation, save permutation and add all market cards to it
                if !owned_permutation.is_empty() {
                    card_permutations.push(owned_permutation.clone());

                    for card in PluginConstants::MARKET_SELECTION {
                        let mut extended_permutation = owned_permutation.clone();
                        extended_permutation.push(card);  // card is already owned
                        card_permutations.push(extended_permutation);
                    }
                }
            }
        }

        let mut moves: Vec<Move> = Vec::new();

        for distance in 1..=max_distance {
            // destination of advance
            let target_pos: usize = current_player.position + distance;

            // out of range, skip
            if target_pos > self.board.track.len() - 1 {
                continue;
            }

            // destination field of advance
            let target_field: Field = self.board.track[target_pos];

            // add card / no card advances for each field type
            match target_field {
                Field::Hare => {
                    for permutation in &card_permutations {
                        moves.push(Move::new(Action::Advance(Advance::new(
                            distance,
                            permutation.iter().copied().collect(),
                        ))));
                    }
                },
                Field::Market => {
                    for card in PluginConstants::MARKET_SELECTION {
                        moves.push(Move::new(Action::Advance(Advance::new(
                            distance,
                            vec![card],
                        ))));
                    }
                },
                _ => {
                    moves.push(Move::new(Action::Advance(Advance::new(distance, vec![]))));
                }
            }
        }

        moves
            .into_iter()
            .unique()
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
            "GameState(board={}, turn={}, player_one={}, player_two={}, last_move={:?})",
            self.board, self.turn, self.player_one, self.player_two, self.last_move
        )
    }
}
