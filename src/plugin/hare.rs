use std::fmt;

use pyo3::*;

use super::{
    action::card::Card,
    constants::PluginConstants,
    errors::{CannotEnterFieldError, MissingCarrotsError},
    field::Field,
    game_state::GameState,
    r#move::Move,
    rules_engine::RulesEngine,
};

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash, Copy)]
pub enum TeamEnum {
    One,
    Two,
}

impl TeamEnum {
    pub fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
}

impl fmt::Display for TeamEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TeamEnum::One => write!(f, "Team One"),
            TeamEnum::Two => write!(f, "Team Two"),
        }
    }
}

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct Hare {
    #[pyo3(get, set)]
    pub team: TeamEnum,
    #[pyo3(get, set)]
    pub position: usize,
    #[pyo3(get, set)]
    pub salads: i32,
    #[pyo3(get, set)]
    pub carrots: i32,
    #[pyo3(get, set)]
    pub last_move: Option<Move>,
    #[pyo3(get)]
    pub cards: Vec<Card>,
}

#[pymethods]
impl Hare {
    #[new]
    #[must_use]
    pub fn new(
        team: TeamEnum,
        cards: Option<Vec<Card>>,
        carrots: Option<i32>,
        salads: Option<i32>,
        last_move: Option<Move>,
        position: Option<usize>,
    ) -> Self {
        Self {
            team,
            cards: cards.unwrap_or_default(),
            carrots: carrots.unwrap_or(PluginConstants::INITIAL_CARROTS),
            salads: salads.unwrap_or(PluginConstants::INITIAL_SALADS),
            last_move,
            position: position.unwrap_or(0),
        }
    }

    pub fn is_in_goal(&self) -> bool {
        self.position == PluginConstants::NUM_FIELDS - 1
    }

    pub fn can_enter_goal(&self) -> bool {
        self.carrots <= 10 && self.salads == 0
    }

    pub fn advance_by(&mut self, state: &mut GameState, distance: usize) -> Result<(), PyErr> {
        let new_position = self.position + distance;
        RulesEngine::can_advance_to(
            &state.board,
            new_position,
            self,
            &state.clone_other_player(),
        )?;

        let needed_carrots = RulesEngine::calculates_carrots(distance);

        if self.carrots - needed_carrots < 0 {
            return Err(MissingCarrotsError::new_err("Not enough carrots"));
        }

        self.carrots -= needed_carrots;
        self.position = new_position;

        state.update_player(self.clone());
        Ok(())
    }

    pub fn exchange_carrots(&mut self, state: &mut GameState, carrots: i32) -> Result<(), PyErr> {
        RulesEngine::can_exchange_carrots(&state.board, self, carrots)?;
        self.carrots += carrots;

        state.update_player(self.clone());
        Ok(())
    }

    pub fn consume_carrots(&mut self, state: &mut GameState, carrots: i32) -> Result<(), PyErr> {
        if self.carrots - carrots >= 0 {
            self.carrots -= carrots;

            state.update_player(self.clone());
            Ok(())
        } else {
            Err(MissingCarrotsError::new_err("Not enough carrots"))
        }
    }

    pub fn eat_salad(&mut self, state: &mut GameState) -> Result<(), PyErr> {
        self.salads -= 1;
        self.carrots += if self.is_ahead(state) { 10 } else { 30 };

        state.update_player(self.clone());
        Ok(())
    }

    pub fn move_to_field(
        &mut self,
        state: &mut GameState,
        new_position: usize,
    ) -> Result<(), PyErr> {
        RulesEngine::can_advance_to(
            &state.board,
            new_position,
            self,
            &state.clone_other_player(),
        )?;
        self.position = new_position;

        state.update_player(self.clone());
        Ok(())
    }

    pub fn get_fall_back(&self, state: &GameState) -> Option<usize> {
        match state
            .board
            .get_previous_field(Field::Hedgehog, self.position)
        {
            Some(i) if state.clone_other_player().position != i => Some(i),
            Some(_) => None,
            None => None,
        }
    }

    pub fn fall_back(&mut self, state: &mut GameState) -> Result<(), PyErr> {
        match self.get_fall_back(state) {
            Some(i) => {
                RulesEngine::has_to_eat_salad(&state.board, self)?;

                self.carrots += 10 * ((self.position - i) as i32);
                self.position = i;

                state.update_player(self.clone());
                Ok(())
            }
            None => Err(CannotEnterFieldError::new_err("Field not found")),
        }
    }

    pub fn is_ahead(&self, state: &GameState) -> bool {
        self.position > state.clone_other_player().position
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
}

impl fmt::Display for Hare {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Hare(team={}, position={}, salads={}, carrots={}, last_move={:?}, cards={:?})",
            self.team, self.position, self.salads, self.carrots, self.last_move, self.cards
        )
    }
}
