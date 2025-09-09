#[cfg(test)]
mod tests {
    use crate::plugin2026::{
        field_type::FieldType, test::common::*, utils::{
            coordinate::Coordinate,
            direction::Direction
        }
    };

    #[test]
    pub fn get_field_test() {
        let b = create_test_board();

        assert_eq!(b.get_field(&Coordinate { x: 0, y: 4 }), Some(FieldType::OneS));
        assert_eq!(b.get_field(&Coordinate { x: 7, y: 0 }), Some(FieldType::TwoL));
        assert_eq!(b.get_field(&Coordinate { x: 3, y: 3 }), Some(FieldType::Empty));
        assert_eq!(b.get_field(&Coordinate { x: 6, y: 2 }), Some(FieldType::Squid));
        assert_eq!(b.get_field(&Coordinate { x: -2, y: 0 }), None);     // out of bounds
    }

    #[test]
    pub fn get_fields_by_type_test() {
        let mut b = create_test_board();

        // remove squids
        b.map[2][6] = FieldType::Empty;
        b.map[7][3] = FieldType::Empty;

        let one_s_positions = vec![
            Coordinate {x: 0, y: 2},
            Coordinate {x: 9, y: 2},
            Coordinate {x: 0, y: 4},
            Coordinate {x: 0, y: 6},
            Coordinate {x: 9, y: 7},
            Coordinate {x: 0, y: 8},
            Coordinate {x: 9, y: 8},
        ];

        assert_eq!(b.get_fields_by_type(FieldType::OneS), one_s_positions);
        assert_eq!(b.get_fields_by_type(FieldType::Squid), vec![]);
    }

    #[test]
    pub fn get_fields_in_direction_test() {
        let b = create_test_board();
        let start = Coordinate {x: 2, y: 6};
        
        for d in Direction::all_directions() {
            match d {
                Direction::Up => assert_eq!(b.get_fields_in_direction(&start, &d), vec![
                    FieldType::Empty, FieldType::Empty, FieldType::TwoS
                ]),
                Direction::UpRight => assert_eq!(b.get_fields_in_direction(&start, &d), vec![
                    FieldType::Squid, FieldType::Empty, FieldType::TwoM
                ]),
                Direction::Right => assert_eq!(b.get_fields_in_direction(&start, &d), vec![
                    FieldType::Empty, FieldType::Empty, FieldType::Empty,
                    FieldType::Empty, FieldType::Empty, FieldType::Empty, FieldType::OneM
                ]),
                Direction::DownRight => assert_eq!(b.get_fields_in_direction(&start, &d), vec![
                    FieldType::Empty, FieldType::Empty, FieldType::Empty,
                    FieldType::Squid, FieldType::Empty, FieldType::TwoS
                ]),
                Direction::Down => assert_eq!(b.get_fields_in_direction(&start, &d), vec![
                    FieldType::Empty, FieldType::Empty, FieldType::Empty,
                    FieldType::Empty, FieldType::Empty, FieldType::TwoS
                ]),
                Direction::DownLeft => assert_eq!(b.get_fields_in_direction(&start, &d), vec![
                    FieldType::Empty, FieldType::OneS
                ]),
                Direction::Left => assert_eq!(b.get_fields_in_direction(&start, &d), vec![
                    FieldType::Empty, FieldType::OneS
                ]),
                Direction::UpLeft => assert_eq!(b.get_fields_in_direction(&start, &d), vec![
                    FieldType::Empty, FieldType::OneS
                ]),
            }
        }
    }

    #[test]
    pub fn get_fields_on_line_test() {
        let b = create_test_board();
        let start = Coordinate {x: 2, y: 7};
        let direction = Direction::Right;

        assert_eq!(b.get_fields_on_line(&start, &direction), vec![
            FieldType::OneL, FieldType::Empty, FieldType::Empty,
            FieldType::Squid, FieldType::Empty, FieldType::Empty,
            FieldType::Empty, FieldType::Empty, FieldType::Empty, FieldType::OneS
        ]);
    }

    #[test]
    pub fn get_fish_on_line_test() {
        let b = create_test_board();
        let start = Coordinate {x: 2, y: 7};
        let direction = Direction::Right;

        assert_eq!(b.get_fish_on_line(&start, &direction), vec![
            FieldType::OneL, FieldType::OneS
        ]);
    }
}
