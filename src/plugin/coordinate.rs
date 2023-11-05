use std::{fmt, ops::Sub};
use std::ops::{Add, AddAssign, Mul, SubAssign};

use pyo3::{pyclass, pymethods};

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Hash)]
pub struct CartesianCoordinate {
    #[pyo3(get, set)]
    pub x: i32,
    #[pyo3(get, set)]
    pub y: i32,
}

#[pymethods]
impl CartesianCoordinate {
    #[new]
    pub fn new(x: i32, y: i32) -> Self {
        CartesianCoordinate { x, y }
    }

    pub fn to_cube(&self) -> CubeCoordinates {
        let q = (self.x - self.y) / 2;
        let r = self.y;
        CubeCoordinates::new(q, r)
    }

    pub fn to_index(&self) -> Option<u64> {
        if self.x < 0 || self.y < 0 || self.x > 3 || self.y > 4 {
            return None;
        }
        Some((self.y * 4 + self.x) as u64)
    }

    #[staticmethod]
    pub fn from_index(index: u64) -> CartesianCoordinate {
        CartesianCoordinate {
            x: (index % 4) as i32,
            y: (index / 5) as i32,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash, Copy)]
#[pyclass]
pub struct CubeCoordinates {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

#[pymethods]
impl CubeCoordinates {
    pub const ORIGIN: Self = Self { q: 0, r: 0, s: 0 };

    #[new]
    pub fn new(q: i32, r: i32) -> Self {
        let s: i32 = -q - r;
        CubeCoordinates { q, r, s }
    }

    pub fn coordinates(&self) -> [i32; 3] {
        [self.q, self.r, self.s]
    }

    pub fn x(&self) -> i32 {
        self.q * 2 + self.r
    }

    pub fn y(&self) -> i32 {
        self.r
    }

    pub fn to_cartesian(&self) -> CartesianCoordinate {
        CartesianCoordinate::new(self.x(), self.y())
    }

    pub fn times(&self, count: i32) -> CubeCoordinates {
        CubeCoordinates::new(self.q * count, self.r * count)
    }

    pub fn plus(&self, other: &CubeCoordinates) -> CubeCoordinates {
        CubeCoordinates::new(self.q + other.q, self.r + other.r)
    }

    pub fn minus(&self, other: &CubeCoordinates) -> CubeCoordinates {
        CubeCoordinates::new(self.q - other.q, self.r - other.r)
    }

    pub fn unary_minus(&self) -> CubeCoordinates {
        CubeCoordinates::new(-self.q, -self.r)
    }

    pub fn rotated_by(&self, turns: i32) -> CubeCoordinates {
        let coordinates = vec![self.q, self.r, self.s];
        let rotated = vec![
            coordinates[((0 + turns.rem_euclid(3)) % 3) as usize],
            coordinates[((1 + turns.rem_euclid(3)) % 3) as usize],
            coordinates[((2 + turns.rem_euclid(3)) % 3) as usize],
        ];
        if turns % 2 == 1 {
            CubeCoordinates::new(-rotated[0], -rotated[1])
        } else {
            CubeCoordinates::new(rotated[0], rotated[1])
        }
    }

    pub fn distance_to(&self, other: &CubeCoordinates) -> i32 {
        ((self.q - other.q).abs() + (self.r - other.r).abs() + (self.s - other.s).abs()) / 2
    }

    pub fn turn_count_to(&self, target: &CubeDirection) -> i32 {
        let mut current: CubeDirection = CubeDirection::Right;
        let mut count: i32 = 0;
        while current != *target {
            current = current.rotated_by(1);
            count += 1;
        }
        count
    }
}

impl Add<CubeCoordinates> for CubeCoordinates {
    type Output = Self;

    fn add(self, other: CubeCoordinates) -> CubeCoordinates {
        CubeCoordinates::new(self.q + other.q, self.r + other.r)
    }
}

impl Add<i32> for CubeCoordinates {
    type Output = Self;

    fn add(self, other: i32) -> CubeCoordinates {
        CubeCoordinates::new(self.q + other, self.r + other)
    }
}

impl AddAssign for CubeCoordinates {
    fn add_assign(&mut self, other: Self) {
        self.q += other.q;
        self.r += other.r;
        self.s = -self.q - self.r;
    }
}


impl Sub<CubeCoordinates> for CubeCoordinates {
    type Output = Self;

    fn sub(self, other: CubeCoordinates) -> CubeCoordinates {
        CubeCoordinates::new(self.q - other.q, self.r - other.r)
    }
}

impl Sub<i32> for CubeCoordinates {
    type Output = Self;

    fn sub(self, other: i32) -> CubeCoordinates {
        CubeCoordinates::new(self.q - other, self.r - other)
    }
}

impl SubAssign for CubeCoordinates {
    fn sub_assign(&mut self, other: Self) {
        self.q -= other.q;
        self.r -= other.r;
        self.s = -self.q - self.r;
    }
}

impl Mul for CubeCoordinates {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        CubeCoordinates {
            q: self.q * rhs.q,
            r: self.r * rhs.r,
            s: self.s * rhs.s,
        }
    }
}

impl Mul<i32> for CubeCoordinates {
    type Output = CubeCoordinates;
    fn mul(self, rhs: i32) -> CubeCoordinates {
        CubeCoordinates {
            q: rhs * self.q,
            r: rhs * self.r,
            s: rhs * self.s,
        }
    }
}

impl Default for CubeCoordinates {
    fn default() -> Self {
        CubeCoordinates::new(0, 0)
    }
}

impl fmt::Display for CubeCoordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.q, self.r, self.s)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash, Default, Copy)]
#[pyclass]
pub enum CubeDirection {
    #[default]
    Right,
    DownRight,
    DownLeft,
    Left,
    UpLeft,
    UpRight,
}

#[pymethods]
impl CubeDirection {
    pub const VALUES: [CubeDirection; 6] =
        [CubeDirection::Right, CubeDirection::DownRight,
            CubeDirection::DownLeft, CubeDirection::Left,
            CubeDirection::UpLeft, CubeDirection::UpRight];
    pub fn vector(&self) -> CubeCoordinates {
        match *self {
            CubeDirection::Right => CubeCoordinates { q: 1, r: 0, s: -1 },
            CubeDirection::DownRight => CubeCoordinates { q: 0, r: 1, s: -1 },
            CubeDirection::DownLeft => CubeCoordinates { q: -1, r: 1, s: 0 },
            CubeDirection::Left => CubeCoordinates { q: -1, r: 0, s: 1 },
            CubeDirection::UpLeft => CubeCoordinates { q: 0, r: -1, s: 1 },
            CubeDirection::UpRight => CubeCoordinates { q: 1, r: -1, s: 0 },
        }
    }

    pub fn angle(&self) -> i32 {
        self.clone() as i32 * 60
    }

    pub fn with_neighbors(&self) -> [CubeDirection; 3] {
        [
            self.rotated_by(-1),
            self.clone(),
            self.rotated_by(1),
        ]
    }

    pub fn opposite(&self) -> CubeDirection {
        match self {
            CubeDirection::Right => CubeDirection::Left,
            CubeDirection::DownRight => CubeDirection::UpLeft,
            CubeDirection::DownLeft => CubeDirection::UpRight,
            CubeDirection::Left => CubeDirection::Right,
            CubeDirection::UpLeft => CubeDirection::DownRight,
            CubeDirection::UpRight => CubeDirection::DownLeft,
        }
    }

    pub fn turn_count_to(&self, target: CubeDirection) -> i32 {
        let diff = (target as i32 - self.clone() as i32).rem_euclid(6);
        if diff > 3 {
            diff - 6
        } else {
            diff
        }
    }

    pub fn rotated_by(&self, turns: i32) -> CubeDirection {
        let directions: [CubeDirection; 6] = [
            CubeDirection::Right,
            CubeDirection::DownRight,
            CubeDirection::DownLeft,
            CubeDirection::Left,
            CubeDirection::UpLeft,
            CubeDirection::UpRight,
        ];

        directions[((self.clone() as i32 + turns).rem_euclid(directions.len() as i32)) as usize].clone()
    }

    pub fn ordinal(&self) -> i32 {
        match self {
            CubeDirection::Right => 0,
            CubeDirection::DownRight => 1,
            CubeDirection::DownLeft => 2,
            CubeDirection::Left => 3,
            CubeDirection::UpLeft => 4,
            CubeDirection::UpRight => 5,
        }
    }
}

impl fmt::Display for CubeDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            CubeDirection::Right => "Right",
            CubeDirection::DownRight => "DownRight",
            CubeDirection::DownLeft => "DownLeft",
            CubeDirection::Left => "Left",
            CubeDirection::UpLeft => "UpLeft",
            CubeDirection::UpRight => "UpRight",
        })
    }
}


#[cfg(test)]
mod tests {
    use crate::plugin::coordinate::{CubeDirection, CubeCoordinates};

    #[test]
    fn test_cube_direction_angle() {
        assert_eq!(CubeDirection::Right.angle(), 0);
        assert_eq!(CubeDirection::DownRight.angle(), 60);
        assert_eq!(CubeDirection::DownLeft.angle(), 120);
        assert_eq!(CubeDirection::Left.angle(), 180);
        assert_eq!(CubeDirection::UpLeft.angle(), 240);
        assert_eq!(CubeDirection::UpRight.angle(), 300);
    }

    #[test]
    fn test_cube_direction_with_neighbors() {
        assert_eq!(
            CubeDirection::Right.with_neighbors(),
            [
                CubeDirection::UpRight,
                CubeDirection::Right,
                CubeDirection::DownRight
            ]
        );
        assert_eq!(
            CubeDirection::DownRight.with_neighbors(),
            [
                CubeDirection::Right,
                CubeDirection::DownRight,
                CubeDirection::DownLeft
            ]
        );
        assert_eq!(
            CubeDirection::DownLeft.with_neighbors(),
            [
                CubeDirection::DownRight,
                CubeDirection::DownLeft,
                CubeDirection::Left
            ]
        );
        assert_eq!(
            CubeDirection::Left.with_neighbors(),
            [
                CubeDirection::DownLeft,
                CubeDirection::Left,
                CubeDirection::UpLeft
            ]
        );
        assert_eq!(
            CubeDirection::UpLeft.with_neighbors(),
            [
                CubeDirection::Left,
                CubeDirection::UpLeft,
                CubeDirection::UpRight
            ]
        );
        assert_eq!(
            CubeDirection::UpRight.with_neighbors(),
            [
                CubeDirection::UpLeft,
                CubeDirection::UpRight,
                CubeDirection::Right
            ]
        );
    }

    #[test]
    fn test_cube_direction_opposite() {
        assert_eq!(CubeDirection::Right.opposite(), CubeDirection::Left);
        assert_eq!(
            CubeDirection::DownRight.opposite(),
            CubeDirection::UpLeft
        );
        assert_eq!(CubeDirection::DownLeft.opposite(), CubeDirection::UpRight);
        assert_eq!(CubeDirection::Left.opposite(), CubeDirection::Right);
        assert_eq!(CubeDirection::UpLeft.opposite(), CubeDirection::DownRight);
        assert_eq!(CubeDirection::UpRight.opposite(), CubeDirection::DownLeft);
    }

    #[test]
    fn test_cube_direction_turn_count_to() {
        assert_eq!(
            CubeDirection::Right.turn_count_to(CubeDirection::Right),
            0
        );
        assert_eq!(
            CubeDirection::Right.turn_count_to(CubeDirection::DownRight),
            1
        );
        assert_eq!(
            CubeDirection::Right.turn_count_to(CubeDirection::DownLeft),
            2
        );
        assert_eq!(CubeDirection::Right.turn_count_to(CubeDirection::Left), 3);
        assert_eq!(
            CubeDirection::Right.turn_count_to(CubeDirection::UpLeft),
            -2
        );
        assert_eq!(
            CubeDirection::Right.turn_count_to(CubeDirection::UpRight),
            -1
        );
    }

    #[test]
    fn test_cube_direction_rotated_by() {
        assert_eq!(CubeDirection::Right.rotated_by(0), CubeDirection::Right);
        assert_eq!(
            CubeDirection::Right.rotated_by(1),
            CubeDirection::DownRight
        );
        assert_eq!(
            CubeDirection::Right.rotated_by(2),
            CubeDirection::DownLeft
        );
        assert_eq!(CubeDirection::Right.rotated_by(3), CubeDirection::Left);
        assert_eq!(CubeDirection::Right.rotated_by(4), CubeDirection::UpLeft);
        assert_eq!(
            CubeDirection::Right.rotated_by(5),
            CubeDirection::UpRight
        );
        assert_eq!(
            CubeDirection::Right.rotated_by(-1),
            CubeDirection::UpRight
        );
        assert_eq!(
            CubeDirection::Right.rotated_by(-2),
            CubeDirection::UpLeft
        );
        assert_eq!(
            CubeDirection::Right.rotated_by(-3),
            CubeDirection::Left
        );
    }

    #[test]
    fn test_cube_coordinates_new() {
        let coords: CubeCoordinates = CubeCoordinates::new(1, 2);
        assert_eq!(coords.q, 1);
        assert_eq!(coords.r, 2);
        assert_eq!(coords.s, -3);
    }

    #[test]
    fn test_cube_coordinates_coordinates() {
        let coords: CubeCoordinates = CubeCoordinates::new(1, 2);
        assert_eq!(coords.coordinates(), [1, 2, -3]);
    }

    #[test]
    fn test_cube_coordinates_x() {
        let coords: CubeCoordinates = CubeCoordinates::new(1, 2);
        assert_eq!(coords.x(), 4);
    }

    #[test]
    fn test_cube_coordinates_times() {
        let coords: CubeCoordinates = CubeCoordinates::new(1, 2);
        let result: CubeCoordinates = coords.times(3);
        assert_eq!(result.q, 3);
        assert_eq!(result.r, 6);
        assert_eq!(result.s, -9);
    }

    #[test]
    fn test_cube_coordinates_plus() {
        let coords1: CubeCoordinates = CubeCoordinates::new(1, 2);
        let coords2: CubeCoordinates = CubeCoordinates::new(3, 4);
        let result: CubeCoordinates = coords1.plus(&coords2);
        assert_eq!(result.q, 4);
        assert_eq!(result.r, 6);
        assert_eq!(result.s, -10);
    }

    #[test]
    fn test_cube_coordinates_minus() {
        let coords1: CubeCoordinates = CubeCoordinates::new(1, 2);
        let coords2: CubeCoordinates = CubeCoordinates::new(3, 4);
        let result: CubeCoordinates = coords1.minus(&coords2);
        assert_eq!(result.q, -2);
        assert_eq!(result.r, -2);
        assert_eq!(result.s, 4);
    }

    #[test]
    fn test_cube_coordinates_unary_minus() {
        let coords: CubeCoordinates = CubeCoordinates::new(1, 2);
        let result: CubeCoordinates = coords.unary_minus();
        assert_eq!(result.q, -1);
        assert_eq!(result.r, -2);
        assert_eq!(result.s, 3);
    }

    #[test]
    fn test_cube_coordinates_rotated_by() {
        let coords: CubeCoordinates = CubeCoordinates::new(3, 2);
        let result: CubeCoordinates = coords.rotated_by(1);
        assert_eq!(result.q, -2);
        assert_eq!(result.r, 5);
        assert_eq!(result.s, -3);
    }

    #[test]
    fn test_cube_coordinates_distance_to() {
        let coords1: CubeCoordinates = CubeCoordinates::new(0, 0);
        let coords2: CubeCoordinates = CubeCoordinates::new(3, 0);
        let result = coords1.distance_to(&coords2);
        assert_eq!(result, 3);

        let coords1: CubeCoordinates = CubeCoordinates::new(-1, -1);
        let coords2: CubeCoordinates = CubeCoordinates::new(-1, 3);
        let result = coords1.distance_to(&coords2);
        assert_eq!(result, 4);

        let coords1: CubeCoordinates = CubeCoordinates::new(0, 0);
        let coords2: CubeCoordinates = CubeCoordinates::new(3, -1);
        let result = coords1.distance_to(&coords2);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_cube_coordinates_turn_count_to() {
        let coords: CubeCoordinates = CubeCoordinates::new(1, 2);
        let result: i32 = coords.turn_count_to(&CubeDirection::DownRight);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_cube_coordinates_sub_i32() {
        let coords: CubeCoordinates = CubeCoordinates::new(1, 2);
        let result: CubeCoordinates = coords - 10;
        assert_eq!(result.q, -9);
        assert_eq!(result.r, -8);
        assert_eq!(result.s, 17);
    }
}
