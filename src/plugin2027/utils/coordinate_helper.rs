use std::collections::HashSet;

use crate::plugin2027::{
    utils::{coordinate::Coordinate, vector::Vector, constants::Constants},
    rotation::Rotation,
};

pub trait CoordinateSetExt {
    fn rotate(&self, rotation: Rotation) -> HashSet<Coordinate>;
    fn flip(&self, should_flip: bool) -> HashSet<Coordinate>;
    fn mirror(&self) -> HashSet<Coordinate>;
    fn turn_right(&self) -> HashSet<Coordinate>;
    fn turn_left(&self) -> HashSet<Coordinate>;
    fn align(&self) -> HashSet<Coordinate>;
    fn area(&self) -> Vector;
}

impl CoordinateSetExt for HashSet<Coordinate> {
    fn rotate(&self, rotation: Rotation) -> HashSet<Coordinate> {
        match rotation {
            Rotation::NONE => self.clone(),
            Rotation::RIGHT => self.turn_right().align(),
            Rotation::MIRROR => self.mirror().align(),
            Rotation::LEFT => self.turn_left().align(),
        }
    }

    fn flip(&self, should_flip: bool) -> HashSet<Coordinate> {
        if !should_flip {
            self.clone()
        } else {
            self.iter()
                .map(|c| Coordinate::new(-c.x, c.y))
                .collect::<HashSet<_>>()
                .align()
        }
    }

    fn mirror(&self) -> HashSet<Coordinate> {
        self.iter().map(|c| Coordinate::new(-c.x, -c.y)).collect()
    }

    fn turn_right(&self) -> HashSet<Coordinate> {
        self.iter().map(|c| Coordinate::new(-c.y, c.x)).collect()
    }

    fn turn_left(&self) -> HashSet<Coordinate> {
        self.iter().map(|c| Coordinate::new(c.y, -c.x)).collect()
    }

    fn align(&self) -> HashSet<Coordinate> {
        let board_length = Constants::BOARD_LENGTH as isize;
        let mut min_x = board_length;
        let mut min_y = board_length;
        for c in self.iter() {
            min_x = min_x.min(c.x);
            min_y = min_y.min(c.y);
        }
        self.iter()
            .map(|c| Coordinate::new(c.x - min_x, c.y - min_y))
            .collect()
    }

    fn area(&self) -> Vector {
        let mut dx = 0;
        let mut dy = 0;
        for c in self.iter() {
            dx = dx.max(c.x);
            dy = dy.max(c.y);
        }
        Vector::new(dx, dy)
    }
}