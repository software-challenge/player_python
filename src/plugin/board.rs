use std::cmp::Ordering;
use std::collections::{ HashSet, BinaryHeap };

use pyo3::prelude::*;

use crate::plugin::coordinate::{ CartesianCoordinate, CubeCoordinates, CubeDirection };
use crate::plugin::field::{ Field, FieldType };
use crate::plugin::game_state::GameState;
use crate::plugin::segment::Segment;
use crate::plugin::ship::Ship;

use super::constants::PluginConstants;

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct Board {
    #[pyo3(get, set)]
    pub segments: Vec<Segment>,
    #[pyo3(get, set)]
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

    pub fn get_segment(&self, index: usize) -> Option<Segment> {
        self.segments.get(index).cloned()
    }

    pub fn segment_with_index_at(&self, coords: CubeCoordinates) -> Option<(usize, Segment)> {
        self.segments
            .iter()
            .enumerate()
            .find(|(_, segment)| { segment.contains(coords.clone()) })
            .map(|(i, s)| (i, s.clone()))
    }

    pub fn get(&self, coords: &CubeCoordinates) -> Option<Field> {
        for segment in &self.segments {
            if segment.contains(*coords) {
                return segment.get(*coords);
            }
        }
        None
    }

    pub fn does_field_have_stream(&self, coords: &CubeCoordinates) -> bool {
        self.segment_with_index_at(*coords)
            .map(|(i, s)| {
                let next_dir: CubeCoordinates = self.segments
                    .get(i + 1)
                    .map(|s| s.direction.vector())
                    .unwrap_or(self.next_direction.vector());
                [
                    s.center - s.direction.vector(),
                    s.center,
                    s.center + next_dir,
                    s.center + next_dir * 2,
                ].contains(&coords)
            })
            .unwrap_or(false)
    }

    pub fn get_field_in_direction(
        &self,
        direction: &CubeDirection,
        coords: &CubeCoordinates
    ) -> Option<Field> {
        self.get(&(coords.clone() + direction.vector()))
    }

    pub fn set_field_in_direction(
        &mut self,
        direction: &CubeDirection,
        coords: &CubeCoordinates,
        field: Field
    ) {
        for segment in &mut self.segments {
            if segment.contains(*coords) {
                segment.set(*coords + direction.vector(), field);
                break;
            }
        }
    }

    pub fn get_coordinate_by_index(
        &self,
        segment_index: usize,
        x_index: usize,
        y_index: usize
    ) -> CubeCoordinates {
        let coord: CubeCoordinates = CartesianCoordinate::new(
            x_index as i32,
            y_index as i32
        ).to_cube();
        self.segments[segment_index].local_to_global(coord)
    }

    pub fn segment_distance(
        &self,
        coordinate1: &CubeCoordinates,
        coordinate2: &CubeCoordinates
    ) -> i32 {
        let segment_index1 = self.segment_index(coordinate1).unwrap();
        let segment_index2 = self.segment_index(coordinate2).unwrap();
        i32::abs((segment_index1 as i32) - (segment_index2 as i32))
    }

    pub fn segment_index(&self, coordinate: &CubeCoordinates) -> Option<usize> {
        self.segments.iter().position(|segment| segment.contains(coordinate.clone()))
    }

    pub fn find_segment(&self, coordinate: &CubeCoordinates) -> Option<Segment> {
        let index = self.segment_index(coordinate)?;
        self.segments.get(index).cloned()
    }

    pub fn neighboring_fields(&self, coords: &CubeCoordinates) -> Vec<Option<Field>> {
        CubeDirection::VALUES.iter()
            .map(|direction| self.get_field_in_direction(&direction, coords))
            .collect()
    }

    pub fn neighboring_coordinates(
        &self,
        coords: &CubeCoordinates
    ) -> Vec<Option<CubeCoordinates>> {
        CubeDirection::VALUES.iter()
            .zip(
                CubeDirection::VALUES.iter().map(|direction|
                    self.get_field_in_direction(&direction, coords)
                )
            )
            .map(|(direction, field)| field.map(|_| coords.clone() + direction.vector()))
            .collect()
    }

    pub fn effective_speed(&self, ship: &Ship) -> i32 {
        let speed = ship.speed;
        if self.does_field_have_stream(&ship.position) {
            speed - 1
        } else {
            speed
        }
    }

    pub fn is_sandbank(&self, coords: &CubeCoordinates) -> bool {
        self.get(coords)
            .map(|field| field.field_type == FieldType::Sandbank)
            .unwrap_or(false)
    }

    pub fn pickup_passenger(&self, state: &GameState) -> GameState {
        let new_state: GameState = state.clone();
        let mut ship = new_state.current_ship;
        if self.effective_speed(&ship) < 2 {
            if let Some(mut field) = new_state.board.pickup_passenger_at_position(&ship.position) {
                field.passenger.as_mut().map(|passenger| {
                    passenger.passenger -= 1;
                });
                ship.passengers += 1;
            }
        }
        new_state
    }

    fn pickup_passenger_at_position(&self, pos: &CubeCoordinates) -> Option<Field> {
        CubeDirection::VALUES.iter()
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

    /// A function `find_nearest_field_types` to find the nearest field(s) of a specific type from a starting point in a hexagonal grid.
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
    /// # Examples
    ///
    /// ```python
    /// from plugin import Board, CubeCoordinates, FieldType
    ///
    /// board = Board()
    /// board.find_nearest_field_types(CubeCoordinates(0, 0), FieldType.Water)
    /// ```
    ///
    pub fn find_nearest_field_types(
        &self,
        start_coordinates: &CubeCoordinates,
        field_type: FieldType
    ) -> HashSet<CubeCoordinates> {
        let max_fields: usize = ((self.segments.len() as i32) *
            PluginConstants::SEGMENT_FIELDS_HEIGHT *
            PluginConstants::SEGMENT_FIELDS_WIDTH) as usize;
        let mut nearest_coordinates: HashSet<CubeCoordinates> = HashSet::with_capacity(max_fields);
        let mut visited: HashSet<CubeCoordinates> = HashSet::with_capacity(max_fields);
        let mut queue: BinaryHeap<QueueItem> = BinaryHeap::new();
        queue.push(QueueItem {
            coordinates: start_coordinates.clone(),
            distance: 0,
        });

        while let Some(QueueItem { coordinates: current_coords, distance }) = queue.pop() {
            if self.found_fields(start_coordinates, nearest_coordinates.clone(), distance) {
                break;
            }

            if let Some(field) = self.get(&current_coords) {
                if field.field_type == field_type {
                    nearest_coordinates.insert(current_coords.clone());
                }
            }

            self.neighboring_coordinates(&current_coords)
                .into_iter()
                .filter_map(|coord| coord)
                .filter(|coord| visited.insert(coord.clone()))
                .for_each(|coord|
                    queue.push(QueueItem {
                        coordinates: coord,
                        distance: distance + 1,
                    })
                );
        }

        nearest_coordinates
    }

    fn found_fields(
        &self,
        start_coordinates: &CubeCoordinates,
        nearest_coordinates: HashSet<CubeCoordinates>,
        distance: i32
    ) -> bool {
        !nearest_coordinates.is_empty() &&
            distance >
                start_coordinates.distance_to(
                    nearest_coordinates.iter().next().unwrap_or(start_coordinates)
                )
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Board(segments={:?}, next_direction={:?})", self.segments, self.next_direction))
    }
}

#[derive(Eq)]
struct QueueItem {
    coordinates: CubeCoordinates,
    distance: i32,
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for QueueItem {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

#[cfg(test)]
mod tests {
    use crate::plugin::field::Passenger;

    use super::*;

    #[test]
    fn test_get_segment() {}

    #[test]
    fn test_get() {}

    #[test]
    fn test_does_field_have_stream() {
        let mut segment: Vec<Segment> = vec![Segment {
            direction: CubeDirection::Right,
            center: CubeCoordinates::new(0, 0),
            fields: vec![
                vec![
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None)
                ],
                vec![
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None)
                ],
                vec![
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None)
                ],
                vec![
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None)
                ],
                vec![
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None)
                ]
            ],
        }];
        let mut board: Board = Board::new(segment, CubeDirection::DownRight);

        assert_eq!(board.does_field_have_stream(&CubeCoordinates::new(0, 0)), true);
        assert_eq!(board.does_field_have_stream(&CubeCoordinates::new(0, 1)), true);
        assert_eq!(board.does_field_have_stream(&CubeCoordinates::new(-1, 1)), false);
        assert_eq!(board.does_field_have_stream(&CubeCoordinates::new(1, 1)), false);

        segment = vec![Segment {
            direction: CubeDirection::DownRight,
            center: CubeCoordinates::new(0, 0),
            fields: vec![
                vec![
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None)
                ],
                vec![
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None)
                ],
                vec![
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None)
                ],
                vec![
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None)
                ],
                vec![
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None)
                ]
            ],
        }];

        board = Board::new(segment, CubeDirection::DownRight);

        assert_eq!(board.does_field_have_stream(&CubeCoordinates::new(0, 0)), true);
        assert_eq!(board.does_field_have_stream(&CubeCoordinates::new(0, 1)), true);
        assert_eq!(board.does_field_have_stream(&CubeCoordinates::new(-1, 1)), false);
        assert_eq!(board.does_field_have_stream(&CubeCoordinates::new(1, 1)), false);
    }

    #[test]
    fn test_get_field_in_direction() {}

    #[test]
    fn test_get_coordinate_by_index() {}

    #[test]
    fn test_segment_distance() {}

    #[test]
    fn test_segment_index() {}

    #[test]
    fn test_find_segment() {}

    #[test]
    fn test_neighboring_fields() {}

    #[test]
    fn test_effective_speed() {}

    #[test]
    fn test_get_field_current_direction() {}

    #[test]
    fn test_find_nearest_field_types() {
        let segment: Vec<Segment> = vec![
            Segment {
                direction: CubeDirection::Right,
                center: CubeCoordinates::new(0, 0),
                fields: vec![
                    vec![
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Sandbank, None),
                        Field::new(FieldType::Sandbank, None),
                        Field::new(FieldType::Sandbank, None),
                        Field::new(FieldType::Water, None)
                    ],
                    vec![
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Sandbank, None),
                        Field::new(FieldType::Sandbank, None),
                        Field::new(FieldType::Sandbank, None),
                        Field::new(FieldType::Water, None)
                    ],
                    vec![
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Sandbank, None),
                        Field::new(FieldType::Sandbank, None),
                        Field::new(FieldType::Sandbank, None),
                        Field::new(FieldType::Water, None)
                    ],
                    vec![
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Water, None)
                    ]
                ],
            },
            Segment {
                direction: CubeDirection::Right,
                center: CubeCoordinates::new(4, 0),
                fields: vec![
                    vec![
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Water, None)
                    ],
                    vec![
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Water, None)
                    ],
                    vec![
                        Field::new(
                            FieldType::Passenger,
                            Some(Passenger {
                                direction: CubeDirection::DownRight,
                                passenger: 1,
                            })
                        ),
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Water, None)
                    ],
                    vec![
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Water, None)
                    ],
                    vec![
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Water, None)
                    ]
                ],
            }
        ];
        let mut board: Board = Board::new(segment, CubeDirection::DownRight);

        assert_eq!(
            board.find_nearest_field_types(&CubeCoordinates::new(0, 0), FieldType::Sandbank),
            vec![CubeCoordinates::new(0, 0)].into_iter().collect()
        );

        board.segments[0].fields[1][2] = Field::new(FieldType::Water, None);

        assert_eq!(
            board.find_nearest_field_types(&CubeCoordinates::new(0, 0), FieldType::Sandbank),
            vec![
                CubeCoordinates::new(1, 0),
                CubeCoordinates::new(0, 1),
                CubeCoordinates::new(-1, 1),
                CubeCoordinates::new(-1, 0),
                CubeCoordinates::new(0, -1),
                CubeCoordinates::new(1, -1)
            ]
                .into_iter()
                .collect()
        );
        assert_eq!(
            board.find_nearest_field_types(&CubeCoordinates::new(2, 0), FieldType::Sandbank),
            vec![CubeCoordinates::new(1, 0)].into_iter().collect()
        );

        assert_eq!(
            board.find_nearest_field_types(&CubeCoordinates::new(1, 0), FieldType::Passenger),
            vec![CubeCoordinates::new(5, -2)].into_iter().collect()
        );

        assert_eq!(
            board.find_nearest_field_types(&CubeCoordinates::new(1, 0), FieldType::Island),
            vec![].into_iter().collect()
        );
    }
}
