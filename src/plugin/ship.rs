use std::sync::{Arc, Mutex};

use pyo3::prelude::*;

use crate::plugin::r#move::Move;

use super::{constants::PluginConstants, coordinate::{CubeCoordinates, CubeDirection}};

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
#[pyclass]
pub enum TeamEnum {
    ONE,
    TWO,
}

#[derive(Debug, Clone)]
#[pyclass]
pub struct Ship {
    pub team: TeamEnum,
    pub moves: Vec<Move>,
    pub opponent: Option<Arc<Mutex<Ship>>>,
    pub position: CubeCoordinates,
    pub direction: CubeDirection,
    pub speed: i32,
    pub coal: i32,
    pub passengers: i32,
    pub free_turns: i32,
    pub points: i32,
    pub free_acc: i32,
    pub movement: i32,
}

#[pymethods]
impl Ship {
    #[new]
    pub fn new(team: TeamEnum, position: CubeCoordinates) -> Self {
        Ship {
            team,
            moves: Vec::new(),
            opponent: None,
            position,
            direction: CubeDirection::Right,
            speed: PluginConstants::MIN_SPEED,
            coal: PluginConstants::START_COAL,
            passengers: 0,
            free_turns: 0,
            points: 0,
            free_acc: PluginConstants::FREE_ACC,
            movement: 0,
        }
    }

    pub fn can_turn(&self) -> bool {
        self.free_turns > 0 || self.coal > 0
    }

    pub fn max_acc(&self) -> i32 {
        (self.coal + self.free_acc).min(PluginConstants::MAX_SPEED - self.speed)
    }

    pub fn accelerate_by(&mut self, diff: i32) {
        self.speed += diff;
        self.movement += diff;
    }

    pub fn read_resolve(&mut self) {
        self.free_acc = PluginConstants::FREE_ACC;
        self.movement = self.speed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_turn() {
        let mut ship = Ship {
            team: TeamEnum::ONE,
            moves: vec![],
            opponent: None,
            position: CubeCoordinates::new(0, 0),
            direction: CubeDirection::Right,
            speed: 0,
            coal: 0,
            passengers: 0,
            free_turns: 0,
            points: 0,
            free_acc: 0,
            movement: 0,
        };
        assert_eq!(ship.can_turn(), false);

        ship.free_turns = 1;
        assert_eq!(ship.can_turn(), true);

        ship.free_turns = 0;
        ship.coal = 1;
        assert_eq!(ship.can_turn(), true);
    }

    #[test]
    fn test_max_acc() {
        let mut ship = Ship {
            team: TeamEnum::ONE,
            moves: vec![],
            opponent: None,
            position: CubeCoordinates::new(0, 0),
            direction: CubeDirection::Right,
            speed: 0,
            coal: 0,
            passengers: 0,
            free_turns: 0,
            points: 0,
            free_acc: 0,
            movement: 0,
        };
        assert_eq!(ship.max_acc(), PluginConstants::MAX_SPEED);

        ship.coal = 1;
        assert_eq!(ship.max_acc(), PluginConstants::MAX_SPEED);

        ship.speed = PluginConstants::MAX_SPEED - 1;
        assert_eq!(ship.max_acc(), 1);

        ship.free_acc = 1;
        assert_eq!(ship.max_acc(), 2);
    }

    #[test]
    fn test_accelerate_by() {
        let mut ship = Ship {
            team: TeamEnum::ONE,
            moves: vec![],
            opponent: None,
            position: CubeCoordinates::new(0, 0),
            direction: CubeDirection::Right,
            speed: 0,
            coal: 0,
            passengers: 0,
            free_turns: 0,
            points: 0,
            free_acc: 0,
            movement: 0,
        };
        ship.accelerate_by(1);
        assert_eq!(ship.speed, 1);
        assert_eq!(ship.movement, 1);

        ship.accelerate_by(-1);
        assert_eq!(ship.speed, 0);
        assert_eq!(ship.movement, 0);
    }

    #[test]
    fn test_read_resolve() {
        let mut ship = Ship {
            team: TeamEnum::ONE,
            moves: vec![],
            opponent: None,
            position: CubeCoordinates::new(0, 0),
            direction: CubeDirection::Right,
            speed: 0,
            coal: 0,
            passengers: 0,
            free_turns: 0,
            points: 0,
            free_acc: 0,
            movement: 0,
        };
        ship.free_acc = 1;
        ship.speed = 1;
        ship.read_resolve();
        assert_eq!(ship.free_acc, PluginConstants::FREE_ACC);
        assert_eq!(ship.movement, 1);
    }
}