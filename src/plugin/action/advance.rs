use pyo3::{pyclass, pymethods, PyErr};

use crate::plugin::{errors::HUIError, field::Field, game_state::GameState, hare::Hare};

use super::card::Card;

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct Advance {
    #[pyo3(get, set)]
    pub distance: usize,
    #[pyo3(get, set)]
    pub cards: Vec<Card>,
}

#[pymethods]
impl Advance {
    #[new]
    #[must_use]
    pub fn new(distance: usize, cards: Vec<Card>) -> Self {
        Self { distance, cards }
    }

    pub fn perform(&self, state: &mut GameState) -> Result<(), PyErr> {
        let mut player = state.clone_current_player();

        player.advance_by(state, self.distance, self.cards.clone())?;

        let current_field = state.board.get_field(player.position).unwrap();
        if self.cards.is_empty() {
            return self.handle_empty_cards(current_field, state, player);
        }

        self.handle_cards(current_field, state, player)
    }

    fn handle_empty_cards(
        &self,
        current_field: Field,
        state: &mut GameState,
        player: Hare,
    ) -> Result<(), PyErr> {
        match current_field {
            Field::Market | Field::Hare => {
                Err(HUIError::new_err("Cannot enter field without any cards"))
            }
            _ => {
                state.update_player(player);
                Ok(())
            }
        }
    }

    fn handle_cards(
        &self,
        mut current_field: Field,
        state: &mut GameState,
        mut player: Hare,
    ) -> Result<(), PyErr> {
        let mut last_card: Option<&Card> = None;
        let mut card_bought = false;

        for (index, card) in self.cards.iter().enumerate() {
            let remaining_cards = self
                .cards
                .get(index + 1..)
                .map(|slice| slice.to_vec())
                .unwrap_or(Vec::new());
            match current_field {
                Field::Market if card_bought => {
                    return Err(HUIError::new_err("Only one card allowed to buy"));
                }
                Field::Market => {
                    player.consume_carrots(state, 10)?;
                    card_bought = true;
                    player.cards.push(*card);
                }
                Field::Hare => {
                    if let Some(last) = last_card {
                        if !last.moves() {
                            return Err(HUIError::new_err("Card cannot be played"));
                        }
                    }

                    last_card = Some(card);

                    card.perform(state, remaining_cards.clone(), self.distance)?;
                    player = state.clone_current_player();
                }
                _ => Err(HUIError::new_err("Card cannot be played on this field"))?,
            }

            current_field = state.board.get_field(player.position).unwrap();
            if current_field == Field::Hare && remaining_cards.is_empty() && last_card.is_none() {
                return Err(HUIError::new_err("Cannot enter field without any cards"));
            }
            if current_field == Field::Market && remaining_cards.is_empty() && !card_bought {
                return Err(HUIError::new_err("Cannot enter field without any cards"));
            }
        }

        state.update_player(player);
        Ok(())
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
}

impl std::fmt::Display for Advance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Advance(distance={}, cards={:?})",
            self.distance, self.cards
        )
    }
}
