#[cfg(test)]
mod tests {
    use crate::plugin2026::{
        field_type::FieldType, r#move::Move, test::common::create_test_game_state, utils::{coordinate::Coordinate, direction::Direction}
    };

    #[test]
    pub fn possible_moves_for_test() {
        let state = create_test_game_state();

        // 3 possible moves
        let position01 = Coordinate {x: 0, y: 3};
        let poss01 = state.possible_moves_for(&position01);

        assert_eq!(poss01, vec![
            Move {start: position01.clone(), direction: Direction::UpRight},
            Move {start: position01.clone(), direction: Direction::Right},
            Move {start: position01.clone(), direction: Direction::DownRight},
        ]);
        
        // no possible move because out of bounds start
        let position02 = Coordinate {x: 0, y: -1};
        let poss02 = state.possible_moves_for(&position02);

        assert_eq!(poss02, vec![]);
    }

    #[test]
    pub fn possible_moves_test() {
        // this state's board has 48 (standard number) moves for team one (turn 0)
        // team two would have 6 turns less from this position due to squids
        let mut state = create_test_game_state();

        assert_eq!(state.possible_moves().len(), 48); // team one

        state.turn = 1;
        assert_eq!(state.possible_moves().len(), 42); // team two
    }

    #[test]
    pub fn perform_move_test() {
        pyo3::prepare_freethreaded_python();

        let mut state = create_test_game_state();

        // correct move
        let new_state = state.perform_move( &Move {
            start: Coordinate { x: 0, y: 3 },
            direction: Direction::Right
        }).unwrap();

        assert_eq!(new_state.board.get_field(&Coordinate { x: 0, y: 3 }), Some(FieldType::Empty));
        assert_eq!(new_state.board.get_field(&Coordinate { x: 2, y: 3 }), Some(FieldType::OneL));

        // illegal moves
        let result = state.perform_move(&Move {
            start: Coordinate { x: -1, y: 0 },
            direction: Direction::Right
        });
        assert!(result.is_err(), "Move FROM out of bounds should fail, but succeeded");

        let result = state.perform_move(&Move {
            start: Coordinate { x: 0, y: 3 },
            direction: Direction::Left
        });
        assert!(result.is_err(), "Move TO out of bounds should fail, but succeeded");

        let result = state.perform_move(&Move {
            start: Coordinate { x: 3, y: 3 },
            direction: Direction::Right
        });
        assert!(result.is_err(), "Move FROM non-fish field should fail, but succeeded");

        let result = state.perform_move(&Move {
            start: Coordinate { x: 6, y: 0 },
            direction: Direction::Up
        });
        assert!(result.is_err(), "Move TO squid field should fail, but succeeded");

        state.board.map[3][2] = FieldType::TwoL;
        let result = state.perform_move(&Move {
            start: Coordinate { x: 2, y: 0 },
            direction: Direction::Up
        });
        assert!(result.is_err(), "Move TO own-fish field should fail, but succeeded");
    }
}
