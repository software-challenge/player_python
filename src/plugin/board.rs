use std::collections::HashSet;
use std::collections::VecDeque;

use pyo3::prelude::*;

use crate::plugin::coordinate::{CartesianCoordinate, CubeCoordinates, CubeDirection};
use crate::plugin::field::{Field, FieldType};
use crate::plugin::game_state::GameState;
use crate::plugin::segment::Segment;
use crate::plugin::ship::Ship;

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct Board {
    pub segments: Vec<Segment>,
    pub next_direction: CubeDirection,
}

#[pymethods]
impl Board {
    #[new]
    pub fn new(segments: Vec<Segment>, next_direction: CubeDirection) -> Self {
        Board {
            segments,
            next_direction,
        }
    }

    fn get_segment(&self, index: usize) -> Option<Segment> {
        self.segments.get(index).cloned()
    }

    pub fn get(&self, coords: &CubeCoordinates) -> Option<Field> {
        for segment in &self.segments {
            if segment.contains(coords.clone()) {
                return segment.get(coords.clone());
            }
        }
        None
    }

    pub fn does_field_have_current(&self, coords: &CubeCoordinates) -> bool {
        self.get_field_current_direction(coords).is_some()
    }

    pub fn get_field_current_direction(&self, coords: &CubeCoordinates) -> Option<CubeDirection> {
        let segment_index = self.segment_index(coords)?;
        let segment = &self.segments[segment_index];
        let next_direction = self.segments.get(segment_index + 1).map_or(self.next_direction.clone(), |s| s.direction.clone());
        let adjacent_coords = [
            segment.center.clone() + segment.direction.opposite().vector(),
            segment.center.clone(),
            segment.center.clone() + next_direction.vector(),
            segment.center.clone() + next_direction.vector() * 2,
        ];

        match adjacent_coords.iter().position(|c| *c == *coords) {
            Some(index) if index < 2 => Some(segment.direction.opposite()),
            Some(_) => Some(next_direction.opposite()),
            None => None,
        }
    }

    pub fn get_field_in_direction(&self, direction: &CubeDirection, coords: &CubeCoordinates) -> Option<Field> {
        self.get(&(coords.clone() + direction.vector()))
    }

    pub fn get_coordinate_by_index(&self, segment_index: usize, x_index: usize, y_index: usize) -> CubeCoordinates {
        let coord: CubeCoordinates = CartesianCoordinate::new(x_index as i32,
                                                              y_index as i32).to_cube();
        self.segments[segment_index].local_to_global(coord)
    }

    pub fn segment_distance(&self, coordinate1: &CubeCoordinates, coordinate2: &CubeCoordinates) -> i32 {
        let segment_index1 = self.segment_index(coordinate1).unwrap();
        let segment_index2 = self.segment_index(coordinate2).unwrap();
        i32::abs(segment_index1 as i32 - segment_index2 as i32)
    }

    pub fn segment_index(&self, coordinate: &CubeCoordinates) -> Option<usize> {
        self.segments.iter().position(|segment| segment.contains(coordinate.clone()))
    }

    pub fn find_segment(&self, coordinate: &CubeCoordinates) -> Option<Segment> {
        let index = self.segment_index(coordinate)?;
        self.segments.get(index).cloned()
    }

    pub fn neighboring_fields(&self, coords: &CubeCoordinates) -> Vec<Option<Field>> {
        CubeDirection::VALUES
            .iter()
            .map(|direction| self.get_field_in_direction(&direction, coords))
            .collect()
    }

    pub fn effective_speed(&self, ship: &Ship) -> i32 {
        let speed = ship.speed;
        if self.does_field_have_current(&ship.position) {
            speed - 1
        } else {
            speed
        }
    }

    pub fn pickup_passenger(&self, state: &GameState) -> GameState {
        let new_state: GameState = state.clone();
        let mut ship = new_state.current_ship();
        if self.effective_speed(&ship) < 2 {
            if let Some(mut field) = new_state.board.pickup_passenger_at_position(&ship.position) {
                field.passenger.as_mut().map(|passenger| passenger.passenger -= 1);
                ship.passengers += 1;
            }
        }
        new_state
    }

    fn pickup_passenger_at_position(&self, pos: &CubeCoordinates) -> Option<Field> {
        CubeDirection::VALUES
            .iter()
            .filter_map(|direction| {
                let field = self.get_field_in_direction(direction, pos)?;
                if field.passenger.as_ref()?.passenger > 0 {
                    Some(field)
                } else {
                    None
                }
            })
            .next()
    }

    /// A function `find_nearest_field_types` to find the nearest field(s) of a specific type from a starting point in a 3D grid.
    ///
    /// # Arguments
    ///
    /// * `start_coordinates` - A CubeCoordinates object representing the starting point for the search.
    /// * `field_type` - A FieldType object representing the type of field being searched for.
    ///
    /// # Returns
    ///
    /// This function will return a Vector of CubeCoordinates corresponding to the location of the nearest field(s) of the specified type.
    ///
    /// This function uses the Breadth-First Search (BFS) algorithm to search through the grid.
    /// BFS was chosen because it perfectly suits for finding the shortest way in such kind of tasks.
    /// It starts at the `start_coordinates`, explores the nearest nodes first and moves towards the next level neighbours only when all the current level nodes are visited.
    ///
    /// It returns immediately when the distance to the current node is larger than the distance to the node in the `nearest_field_coordinates`,
    /// meaning it has passed the nearest node(s) and there is no need to continue the search.
    ///
    /// # Note
    ///
    /// This function will always return the coordinates of the nearest field(s) of the specified type, if such a field(s) exist.
    /// If multiple fields of the same type are at the same minimum distance, it returns all of them.
    /// If there isn't a field of the specified type or path to it, it will return an empty Vec.
    ///
    pub fn find_nearest_field_types(&mut self, start_coordinates: &CubeCoordinates, field_type: FieldType) -> Vec<CubeCoordinates> {
        let mut visited_coordinates: HashSet<CubeCoordinates> = HashSet::new();
        let mut neighbour_coordinates_queue: VecDeque<CubeCoordinates> = VecDeque::new();
        let mut nearest_field_coordinates: Vec<CubeCoordinates> = Vec::new();

        let check_field_and_add_to_queue = |coordinates: &CubeCoordinates, neighbour_coordinates_queue: &mut VecDeque<CubeCoordinates>, visited_coordinates: &HashSet<CubeCoordinates>| {
            let neighbour_field = self.get(coordinates);
            if neighbour_field.is_some() && field_type == neighbour_field.unwrap().field_type {
                if !visited_coordinates.contains(coordinates) && !neighbour_coordinates_queue.contains(coordinates) {
                    neighbour_coordinates_queue.push_back(coordinates.clone());
                }
            }
        };

        for direction in CubeDirection::VALUES {
            check_field_and_add_to_queue(&(start_coordinates.clone() + direction.vector()), &mut neighbour_coordinates_queue, &visited_coordinates);
        }

        while let Some(current_coordinates) = neighbour_coordinates_queue.pop_front() {
            if !nearest_field_coordinates.is_empty() &&
                start_coordinates.distance_to(&current_coordinates) > start_coordinates.distance_to(nearest_field_coordinates.last().unwrap()) {
                break;
            }
            visited_coordinates.insert(current_coordinates.clone());
            let current_field = self.get(&current_coordinates);
            if let Some(current_field_is) = current_field {
                if field_type == current_field_is.field_type {
                    nearest_field_coordinates.push(current_coordinates.clone());
                } else {
                    for direction in CubeDirection::VALUES {
                        check_field_and_add_to_queue(&(current_coordinates.clone() + direction.vector()), &mut neighbour_coordinates_queue, &visited_coordinates);
                    }
                }
            }
        }

        nearest_field_coordinates
    }
}
