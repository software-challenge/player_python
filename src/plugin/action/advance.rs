use pyo3::{ pyclass, pymethods, PyErr };

use crate::plugin::{
    errors::{ CannotPlayCardError, MustBuyOneCardError },
    field::Field,
    game_state::GameState,
};

use super::card::Card;

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct Advance {
    #[pyo3(get, set)]
    distance: usize,
    #[pyo3(get, set)]
    cards: Vec<Card>,
}

#[pymethods]
impl Advance {
    #[new]
    #[must_use]
    pub fn new(distance: usize, cards: Vec<Card>) -> Self {
        Self { distance, cards }
    }

    pub fn perform(&self, state: &mut GameState) -> Result<(), PyErr> {
        let mut player = state.get_current_player();

        state.can_advance_to(self.distance, &player)?;
        player.advance_by(self.distance)?;

        let mut last_card: Option<&Card> = None;
        let mut card_bought = false;

        for card in &self.cards {
            match state.board.get_field(player.position).unwrap() {
                Field::Market => {
                    if card_bought {
                        return Err(MustBuyOneCardError::new_err("Only one card allowed to buy"));
                    }
                    player.consume_carrots(10)?;
                    card_bought = true;
                    player.cards.push(*card);
                }
                Field::Hare => {
                    if let Some(last) = last_card {
                        if !last.moves() {
                            return Err(CannotPlayCardError::new_err("Card cannot be played"));
                        }
                    }
                }
                _ => {}
            }
            last_card = Some(card);
            card.perform(state)?;
        }
        state.set_current_player(player);
        Ok(())
    }
}
