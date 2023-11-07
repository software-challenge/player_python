use pyo3::prelude::*;

use super::{ constants::PluginConstants, coordinate::{ CubeCoordinates, CubeDirection } };

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash, Copy)]
#[pyclass]
pub enum TeamEnum {
    One,
    Two,
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash, Copy)]
#[pyclass]
pub struct Ship {
    #[pyo3(get, set)]
    pub team: TeamEnum,
    #[pyo3(get, set)]
    pub position: CubeCoordinates,
    #[pyo3(get, set)]
    pub direction: CubeDirection,
    #[pyo3(get, set)]
    pub speed: i32,
    #[pyo3(get, set)]
    pub coal: i32,
    #[pyo3(get, set)]
    pub passengers: i32,
    #[pyo3(get, set)]
    pub free_turns: i32,
    #[pyo3(get, set)]
    pub points: i32,
    #[pyo3(get, set)]
    pub free_acc: i32,
    #[pyo3(get, set)]
    pub movement: i32,
}

#[pymethods]
impl Ship {
    #[new]
    pub fn new(
        position: CubeCoordinates,
        team: TeamEnum,
        direction: Option<CubeDirection>,
        speed: Option<i32>,
        coal: Option<i32>,
        passengers: Option<i32>,
        points: Option<i32>,
        free_turns: Option<i32>,
        movement: Option<i32>
    ) -> Self {
        Ship {
            team,
            position,
            direction: direction.unwrap_or(CubeDirection::Right),
            speed: speed.unwrap_or(PluginConstants::MIN_SPEED),
            coal: coal.unwrap_or(PluginConstants::START_COAL),
            passengers: passengers.unwrap_or(0),
            free_turns: free_turns.unwrap_or(0),
            points: points.unwrap_or(0),
            free_acc: PluginConstants::FREE_ACC,
            movement: movement.unwrap_or(speed.unwrap_or(PluginConstants::MIN_SPEED)),
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

    fn __repr__(&self) -> PyResult<String> {
        Ok(
            format!(
                "Ship(position: {}, team: {:?}, direction: {:?}, speed: {}, coal: {}, passengers: {}, free_turns: {}, points: {}, free_acc: {}, movement: {})",
                self.position,
                self.team,
                self.direction,
                self.speed,
                self.coal,
                self.passengers,
                self.free_turns,
                self.points,
                self.free_acc,
                self.movement
            )
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_turn() {
        let mut ship = Ship {
            team: TeamEnum::One,
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
            team: TeamEnum::One,
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
        assert_eq!(ship.max_acc(), 0);

        ship.coal = 1;
        assert_eq!(ship.max_acc(), 1);

        ship.speed = PluginConstants::MAX_SPEED - 1;
        assert_eq!(ship.max_acc(), 1);

        ship.free_acc = 1;
        assert_eq!(ship.max_acc(), 1);
    }

    #[test]
    fn test_accelerate_by() {
        let mut ship = Ship {
            team: TeamEnum::One,
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
            team: TeamEnum::One,
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
