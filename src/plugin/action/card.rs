use std::mem::swap;

use pyo3::*;

use crate::plugin::{
    constants::PluginConstants, errors::HUIError, field::Field, game_state::GameState, hare::Hare,
    rules_engine::RulesEngine,
};

use super::Action;

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Copy)]
pub enum Card {
    FallBack,
    HurryAhead,
    EatSalad,
    SwapCarrots,
}

#[pymethods]
impl Card {
    pub fn moves(&self) -> bool {
        match self {
            Card::FallBack | Card::HurryAhead => true,
            Card::EatSalad | Card::SwapCarrots => false,
        }
    }

    fn move_to_field(
        &self,
        player: &mut Hare,
        state: &mut GameState,
        target_position: usize,
        cards: Vec<Card>,
    ) -> Result<(), PyErr> {
        let distance = target_position as isize - player.position as isize;
        RulesEngine::can_move_to(
            &state.board,
            distance,
            player,
            &state.clone_other_player(),
            cards,
        )?;

        player.position = (player.position as isize + distance) as usize;

        state.update_player(player.clone());
        Ok(())
    }

    fn play(
        &self,
        state: &mut GameState,
        current: &mut Hare,
        other: &mut Hare,
        remaining_cards: Vec<Card>,
        advance_distance: usize,
    ) -> Result<(), PyErr> {
        match self {
            Card::FallBack => {
                println!("trest");
                if current.position < other.position {
                    return Err(HUIError::new_err(
                        "You can only play this card if you are ahead of the other player",
                    ));
                }
                self.move_to_field(
                    current,
                    state,
                    other.position.saturating_sub(1),
                    remaining_cards,
                )?;
            }
            Card::HurryAhead => {
                if current.position > other.position {
                    return Err(HUIError::new_err(
                        "You can only play this card if you are behind the other player",
                    ));
                }
                // saturating add is here unnecessary because the board is finite and never larger than usize::MAX
                self.move_to_field(current, state, other.position + 1, remaining_cards)?;
            }
            Card::EatSalad => {
                if current.salads == 0 {
                    return Err(HUIError::new_err(
                        "You can only play this card if you have lettuce left",
                    ));
                }

                current.eat_salad(state)?
            }
            Card::SwapCarrots => {
                if current.position >= PluginConstants::LAST_LETTUCE_POSITION
                    || other.position >= PluginConstants::LAST_LETTUCE_POSITION
                {
                    return Err(HUIError::new_err(
                    "You can only play this card if both players are before the last lettuce field",
                ));
                }

                let mut current_ok: bool = true;
                if let Some(current_last_move) = &current.last_move
                {
                    if let Action::Advance(current_advance) = &current_last_move.action
                    {   
                        if current_advance.cards.contains(&Card::SwapCarrots) 
                        && state.board.track[current.position - advance_distance] == Field::Hare
                        {
                            current_ok = false;
                        }
                    }
                }

                let mut other_ok: bool = true;
                if let Some(other_last_move) = &other.last_move
                {
                    if let Action::Advance(other_advance) = &other_last_move.action
                    {
                        if other_advance.cards.contains(&Card::SwapCarrots)
                        && state.board.track[other.position] == Field::Hare
                        {
                            other_ok = false;
                        }
                    }
                }

                if !current_ok || !other_ok {
                    return Err(HUIError::new_err(
                        "You can only play this card if the last similar swap card was not used in one of the last two turns",
                    ));
                }

                swap(&mut current.carrots, &mut other.carrots);
            }
        }
        Ok(())
    }

    pub fn perform(&self, state: &mut GameState, remaining_cards: Vec<Card>, advance_distance: usize) -> Result<(), PyErr> {
        let mut current = state.clone_current_player();
        let mut other = state.clone_other_player();

        let field = state
            .board
            .get_field(current.position)
            .ok_or_else(|| HUIError::new_err("Field not found"))?;

        if field != Field::Hare {
            return Err(HUIError::new_err(
                "You can only play cards on the hare field",
            ));
        }

        let index = current
            .cards
            .iter()
            .position(|card| card == self)
            .ok_or_else(|| HUIError::new_err("Card not owned"))?;

        self.play(state, &mut current, &mut other, remaining_cards, advance_distance)?;

        current.cards.remove(index);

        state.update_player(current);
        state.update_player(other);

        Ok(())
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        match self {
            Card::FallBack => write!(f, "Fallback Card"),
            Card::HurryAhead => write!(f, "HurryAhead Card"),
            Card::EatSalad => write!(f, "EatSalad Card"), 
            Card::SwapCarrots => write!(f, "SwapCarrot Card"),
        }
    }
}
