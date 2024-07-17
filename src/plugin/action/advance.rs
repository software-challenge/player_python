use pyo3::{pyclass, pymethods, PyErr};

use crate::plugin::{
    errors::{CannotPlayCardError, MustBuyOneCardError, MustPlayCardError},
    field::Field,
    game_state::GameState,
};

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

        player.advance_by(state, self.distance)?;

        let current_field = state.board.get_field(player.position).unwrap();
        if self.cards.is_empty() {
            match current_field {
                Field::Market | Field::Hare => {
                    return Err(MustPlayCardError::new_err(
                        "Cannot enter field without any cards",
                    ));
                }
                _ => {}
            }
        }

        let mut last_card: Option<&Card> = None;
        let mut card_bought = false;

        for card in &self.cards {
            match current_field {
                Field::Market if card_bought => {
                    return Err(MustBuyOneCardError::new_err("Only one card allowed to buy"));
                }
                Field::Market => {
                    player.consume_carrots(state, 10)?;
                    card_bought = true;
                    player.cards.push(*card);
                }
                Field::Hare => {
                    if let Some(last) = last_card {
                        if !last.moves() {
                            return Err(CannotPlayCardError::new_err("Card cannot be played"));
                        }
                    }

                    last_card = Some(card);
                    card.perform(state)?;
                }
                _ => {}
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
