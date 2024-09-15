use pyo3::{pyclass, pymethods, PyErr};

use crate::plugin::{errors::HUIError, field::Field, game_state::GameState};

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
            match current_field {
                Field::Market | Field::Hare => {
                    return Err(HUIError::new_err("Cannot enter field without any cards"));
                }
                _ => {
                    state.update_player(player);
                    return Ok(());
                }
            }
        }

        let mut last_card: Option<&Card> = None;
        let mut card_bought = false;

        for card in &self.cards {
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
                    let mut remaining_cards = self.cards.clone();

                    if let Some(position) = remaining_cards.iter().position(|c| c == card) {
                        remaining_cards.remove(position);
                    } else {
                        return Err(HUIError::new_err("Card not in list of cards"))?;
                    }

                    card.perform(state, remaining_cards)?;
                }
                _ => Err(HUIError::new_err("Card cannot be played on this field"))?,
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
