use pyo3::*;

use super::{
    action::card::Card,
    constants::PluginConstants,
    errors::{CannotEnterFieldError, FieldNonexistentError, MissingCarrotsError, NoSaladError},
    field::Field,
    game_state::GameState,
    rules_engine::RulesEngine,
};

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash, Copy)]
pub enum TeamEnum {
    One,
    Two,
}

impl TeamEnum {
    pub fn __repr__(&self) -> PyResult<String> {
        Ok(match self {
            Self::One => "TeamEnum.One".to_string(),
            Self::Two => "TeamEnum.Two".to_string(),
        })
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
    pub salad_eaten: bool,
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
        salad_eaten: Option<bool>,
        position: Option<usize>,
    ) -> Self {
        Self {
            team,
            cards: cards.unwrap_or_default(),
            carrots: carrots.unwrap_or(PluginConstants::INITIAL_CARROTS),
            salads: salads.unwrap_or(PluginConstants::INITIAL_SALADS),
            salad_eaten: salad_eaten.unwrap_or(false),
            position: position.unwrap_or(0),
        }
    }

    pub fn is_in_goal(&self) -> bool {
        self.position == PluginConstants::NUM_FIELDS - 1
    }

    pub fn can_enter_goal(&self) -> bool {
        self.carrots <= 10 && self.salads == 0
    }

    pub fn advance_by(&mut self, state: &GameState, distance: usize) -> Result<(), PyErr> {
        RulesEngine::can_advance_to(&state.board, distance, self, &state.clone_other_player())?;

        let carrots = RulesEngine::calculates_carrots(distance);

        if self.carrots - carrots < 0 {
            return Err(MissingCarrotsError::new_err("Not enough carrots"));
        }

        if self.position.checked_sub(distance).is_none() {
            return Err(FieldNonexistentError::new_err("Cannot go on this field"));
        }

        self.carrots -= carrots;
        self.position += distance;
        Ok(())
    }

    pub fn exchange_carrots(&mut self, state: &GameState, carrots: i32) -> Result<(), PyErr> {
        RulesEngine::can_exchange_carrots(&state.board, self, carrots)?;
        self.carrots += carrots;
        Ok(())
    }

    pub fn consume_carrots(&mut self, carrots: i32) -> Result<(), PyErr> {
        if self.carrots - carrots >= 0 {
            self.carrots -= carrots;
            Ok(())
        } else {
            Err(MissingCarrotsError::new_err("Not enough carrots"))
        }
    }

    pub fn eat_salad(&mut self, state: &GameState) -> Result<(), PyErr> {
        if self.salads < 1 {
            return Err(NoSaladError::new_err("Not enough salads"));
        }
        self.salads -= 1;
        self.carrots += if self.is_ahead(state) { 10 } else { 30 };
        self.salad_eaten = true;
        Ok(())
    }

    pub fn move_to_field(&mut self, state: &GameState, new_position: usize) -> Result<(), PyErr> {
        RulesEngine::can_advance_to(
            &state.board,
            new_position,
            self,
            &state.clone_other_player(),
        )?;
        self.position = new_position;
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

    pub fn fall_back(&mut self, state: &GameState) -> Result<(), PyErr> {
        match self.get_fall_back(state) {
            Some(i) => {
                self.carrots += 10 * ((self.position - i) as i32);
                self.position = i;
                Ok(())
            }
            None => Err(CannotEnterFieldError::new_err("Field not found")),
        }
    }

    pub fn is_ahead(&self, state: &GameState) -> bool {
        self.position > state.clone_other_player().position
    }
}
