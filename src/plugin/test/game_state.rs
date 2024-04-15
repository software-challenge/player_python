#[cfg(test)]
mod tests {
    use pyo3::Python;

    use crate::plugin::{
        actions::{ accelerate::Accelerate, advance::Advance, push::Push, turn::Turn, Action },
        board::Board,
        coordinate::{ CubeCoordinates, CubeDirection },
        errors::advance_errors::AdvanceProblem,
        field::{ Field, FieldType, Passenger },
        game_state::{ AdvanceInfo, GameState },
        r#move::Move,
        segment::Segment,
        ship::{ Ship, TeamEnum },
    };

    fn create_water_segment(center: CubeCoordinates, direction: CubeDirection) -> Segment {
        Segment {
            direction,
            center,
            fields: vec![vec![Field::new(FieldType::Water, None); 4]; 5],
        }
    }

    fn create_ship(position: CubeCoordinates, team: TeamEnum) -> Ship {
        Ship::new(position, team, None, None, None, None, None, None, None)
    }

    fn create_game_state(segment: Vec<Segment>, team_one: Ship, team_two: Ship) -> GameState {
        GameState::new(Board::new(segment, CubeDirection::Right), 0, team_one, team_two, None)
    }

    #[test]
    fn test_perform_move() {
        let mut segment = vec![
            create_water_segment(CubeCoordinates::new(0, 0), CubeDirection::Right),
            create_water_segment(CubeCoordinates::new(4, 0), CubeDirection::Right)
        ];
        let mut team_one = create_ship(CubeCoordinates::new(5, 0), TeamEnum::One);
        let team_two = create_ship(CubeCoordinates::new(2, 0), TeamEnum::Two);
        team_one.movement = 2;
        team_one.speed = 2;
        segment[1].set(
            CubeCoordinates::new(5, 1),
            Field::new(FieldType::Passenger, Some(Passenger::new(CubeDirection::UpRight, 1)))
        );
        let game_state = create_game_state(segment, team_one, team_two);

        let move_: Move = Move::new(vec![Action::Advance(Advance::new(1))]);

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|_| {
            let new_state = game_state.perform_move(move_);

            match new_state {
                Ok(value) => {
                    println!(
                        "Result: {} {}",
                        value.current_ship.passengers,
                        value.other_ship.passengers
                    );
                    value.board.pretty_print()
                }
                Err(err) => println!("Error: {}", err),
            }
        });
    }

    #[test]
    fn test_remove_passenger_at() {
        let mut segment = vec![
            create_water_segment(CubeCoordinates::new(0, 0), CubeDirection::Right)
        ];
        let team_one = create_ship(CubeCoordinates::new(0, -1), TeamEnum::One);
        let team_two = create_ship(CubeCoordinates::new(-1, 1), TeamEnum::Two);
        segment[0].set(
            CubeCoordinates::new(0, 0),
            Field::new(FieldType::Passenger, Some(Passenger::new(CubeDirection::UpLeft, 1)))
        );
        let mut game_state = create_game_state(segment, team_one, team_two);

        assert_eq!(game_state.current_ship.passengers, 0);
        assert_eq!(game_state.current_ship.points, 0);
        game_state.pick_up_passenger_current_ship();
        assert_eq!(game_state.current_ship.passengers, 1);
        assert_eq!(game_state.current_ship.points, 6);
        game_state.pick_up_passenger_current_ship();
        assert_eq!(game_state.current_ship.passengers, 1);
        assert_eq!(game_state.current_ship.points, 6);
    }

    #[test]
    fn find_possible_moves_returns_correct_count() {
        let segment = vec![create_water_segment(CubeCoordinates::new(0, 0), CubeDirection::Right)];
        let team_one = create_ship(CubeCoordinates::new(0, -1), TeamEnum::One);
        let team_two = create_ship(CubeCoordinates::new(-1, 1), TeamEnum::Two);
        let game_state = create_game_state(segment, team_one, team_two);

        let possible_moves = game_state.possible_action_comb(&game_state, vec![], 0, 5);
        assert_eq!(possible_moves.len(), 6725);
    }

    #[test]
    fn test_check_advance_limit() {
        let segment = vec![create_water_segment(CubeCoordinates::new(0, 0), CubeDirection::Right)];
        let mut team_one = create_ship(CubeCoordinates::new(0, 0), TeamEnum::One);
        team_one.speed = 5;
        team_one.movement = 5;
        let mut team_two = create_ship(CubeCoordinates::new(-1, 1), TeamEnum::Two);
        team_two.speed = 5;
        team_two.movement = 5;
        let game_state = create_game_state(segment, team_one, team_two);

        let advances: AdvanceInfo = game_state.check_ship_advance_limit(&team_one);

        assert_eq!(advances.costs, vec![2, 3, 4]);
        assert_eq!(advances.problem, AdvanceProblem::FieldIsBlocked);
        assert_eq!(advances.distance(), 3);
    }

    #[test]
    fn test_check_advance_limit_to_upperleft_end() {
        let segment = vec![create_water_segment(CubeCoordinates::new(0, 0), CubeDirection::Right)];
        let mut team_one = create_ship(CubeCoordinates::new(0, -1), TeamEnum::One);
        team_one.speed = 5;
        team_one.movement = 5;
        team_one.direction = CubeDirection::Left;
        let mut team_two = create_ship(CubeCoordinates::new(-1, 1), TeamEnum::Two);
        team_two.speed = 5;
        team_two.movement = 5;
        let game_state = create_game_state(segment, team_one, team_two);

        let advances: AdvanceInfo = game_state.check_ship_advance_limit(&team_one);

        assert_eq!(advances.costs, vec![1]);
        assert_eq!(advances.problem, AdvanceProblem::FieldIsBlocked);
        assert_eq!(advances.distance(), 1);
    }

    #[test]
    fn test_get_accelerations() {
        let segment = vec![create_water_segment(CubeCoordinates::new(0, 0), CubeDirection::Right)];
        let mut team_one = create_ship(CubeCoordinates::new(0, 0), TeamEnum::One);
        team_one.speed = 5;
        team_one.movement = 5;
        let mut team_two = create_ship(CubeCoordinates::new(-1, 1), TeamEnum::Two);
        team_two.speed = 5;
        team_two.movement = 5;
        let game_state = create_game_state(segment, team_one, team_two);

        let accelerations: Vec<Accelerate> = game_state.possible_accelerations(None);
        assert_eq!(accelerations.len(), 5);
        assert_eq!(accelerations[4].acc, -4);
    }

    #[test]
    fn test_get_turns() {
        let segment = vec![create_water_segment(CubeCoordinates::new(0, 0), CubeDirection::Right)];
        let mut team_one = create_ship(CubeCoordinates::new(0, 0), TeamEnum::One);
        team_one.speed = 5;
        team_one.movement = 5;
        let mut team_two = create_ship(CubeCoordinates::new(-1, 1), TeamEnum::Two);
        team_two.speed = 5;
        team_two.movement = 5;
        let game_state = create_game_state(segment, team_one, team_two);

        let turns: Vec<Turn> = game_state.possible_turns(None);
        assert_eq!(turns.len(), 5);
        assert_eq!(turns[4].direction, CubeDirection::Left);
    }

    #[test]
    fn test_get_advances() {
        let segment = vec![create_water_segment(CubeCoordinates::new(0, 0), CubeDirection::Right)];
        let mut team_one = create_ship(CubeCoordinates::new(0, 0), TeamEnum::One);
        team_one.speed = 5;
        team_one.movement = 5;
        let mut team_two = create_ship(CubeCoordinates::new(-1, 1), TeamEnum::Two);
        team_two.speed = 5;
        team_two.movement = 5;
        let game_state = create_game_state(segment, team_one, team_two);

        let advances: Vec<Advance> = game_state.possible_advances();
        assert_eq!(advances.len(), 3);
        assert_eq!(advances[1].distance, 2);
    }

    #[test]
    fn test_get_pushes() {
        let segment = vec![create_water_segment(CubeCoordinates::new(0, 0), CubeDirection::Right)];
        let mut team_one = create_ship(CubeCoordinates::new(0, 0), TeamEnum::One);
        team_one.speed = 5;
        team_one.movement = 5;
        let mut team_two = create_ship(CubeCoordinates::new(0, 0), TeamEnum::Two);
        team_two.speed = 5;
        team_two.movement = 5;
        let game_state = create_game_state(segment, team_one, team_two);

        let pushes: Vec<Push> = game_state.possible_pushes();
        assert_eq!(pushes.len(), 5);
        assert_eq!(pushes[0].direction, CubeDirection::Right);
    }

    #[test]
    fn test_only_pushes_if_must_push() {
        let segment = vec![create_water_segment(CubeCoordinates::new(0, 0), CubeDirection::Right)];
        let mut team_one = create_ship(CubeCoordinates::new(0, 0), TeamEnum::One);
        team_one.speed = 5;
        team_one.movement = 5;
        let mut team_two = create_ship(CubeCoordinates::new(0, 0), TeamEnum::Two);
        team_two.speed = 5;
        team_two.movement = 5;
        let game_state = create_game_state(segment, team_one, team_two);

        let actions: Vec<Action> = game_state.possible_actions(1, None);
        assert_eq!(actions.len(), 5);
        assert!(actions.iter().all(|a| matches!(a, Action::Push(_))));
    }

    #[test]
    fn test_performe_move() {
        let segment: Vec<Segment> = vec![
            Segment {
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
            },
            Segment {
                direction: CubeDirection::Right,
                center: CubeCoordinates::new(3, 0),
                fields: vec![
                    vec![
                        Field::new(FieldType::Water, None),
                        Field::new(
                            FieldType::Passenger,
                            Some(Passenger::new(CubeDirection::DownLeft, 1))
                        ),
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
                        Field::new(FieldType::Island, None),
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Water, None)
                    ],
                    vec![
                        Field::new(FieldType::Island, None),
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Water, None),
                        Field::new(FieldType::Water, None)
                    ]
                ],
            }
        ];
        let board: Board = Board::new(segment, CubeDirection::Right);
        let team_one: &mut Ship = &mut Ship::new(
            CubeCoordinates::new(-1, -1),
            TeamEnum::One,
            Some(CubeDirection::Right),
            Some(1),
            Some(6),
            Some(0),
            Some(0),
            Some(0),
            Some(1)
        );
        let team_two: &mut Ship = &mut Ship::new(
            CubeCoordinates::new(-2, 1),
            TeamEnum::Two,
            Some(CubeDirection::Right),
            Some(1),
            Some(6),
            Some(0),
            Some(0),
            Some(0),
            Some(1)
        );
        let game_state: GameState = GameState::new(board, 0, *team_one, *team_two, None);

        let move_: Move = Move::new(
            vec![Action::Accelerate(Accelerate::new(1)), Action::Advance(Advance::new(2))]
        );

        assert_eq!(game_state.current_ship.team, TeamEnum::One);
        assert_eq!(game_state.current_ship.position, CubeCoordinates::new(-1, -1));

        let new_state: GameState = game_state.perform_move(move_).unwrap();
        assert_eq!(new_state.other_ship.team, TeamEnum::One);
        assert_eq!(new_state.other_ship.position, CubeCoordinates::new(1, -1));

        assert_eq!(new_state.current_ship.team, TeamEnum::Two);
        assert_eq!(new_state.current_ship.position, CubeCoordinates::new(-2, 1));

        let second_move_: Move = Move::new(
            vec![Action::Accelerate(Accelerate::new(1)), Action::Advance(Advance::new(2))]
        );

        let second_new_state: GameState = new_state.perform_move(second_move_).unwrap();
        assert_eq!(second_new_state.current_ship.team, TeamEnum::One);
        assert_eq!(second_new_state.current_ship.position, CubeCoordinates::new(1, -1));
        assert_eq!(second_new_state.other_ship.team, TeamEnum::Two);
        assert_eq!(second_new_state.other_ship.position, CubeCoordinates::new(0, 1));
    }

    #[test]
    fn test_advance_turn() {
        let segment = vec![create_water_segment(CubeCoordinates::new(0, 0), CubeDirection::Right)];
        let team_one = create_ship(CubeCoordinates::new(0, -1), TeamEnum::One);
        let team_two = create_ship(CubeCoordinates::new(-1, 1), TeamEnum::Two);
        let mut game_state = create_game_state(segment, team_one, team_two);

        assert_eq!(game_state.current_ship.team, TeamEnum::One);
        assert_eq!(game_state.other_ship.team, TeamEnum::Two);

        game_state.advance_turn();

        assert_eq!(game_state.current_ship.team, TeamEnum::Two);
        assert_eq!(game_state.other_ship.team, TeamEnum::One);

        game_state.advance_turn();

        assert_eq!(game_state.current_ship.team, TeamEnum::One);
        assert_eq!(game_state.other_ship.team, TeamEnum::Two);
    }

    #[test]
    fn test_team_ahead() {
        let segment = vec![create_water_segment(CubeCoordinates::new(0, 0), CubeDirection::Right)];
        let team_one = create_ship(CubeCoordinates::new(0, -1), TeamEnum::One);
        let team_two = create_ship(CubeCoordinates::new(-1, 1), TeamEnum::Two);
        let game_state = create_game_state(segment, team_one, team_two);

        assert_eq!(game_state.determine_ahead_team().team, TeamEnum::One);

        let game_move: Move = Move::new(vec![Action::Advance(Advance::new(1))]);

        let new_state: GameState = game_state.perform_move(game_move).unwrap();

        assert_eq!(new_state.determine_ahead_team().team, TeamEnum::One);

        let second_move: Move = Move::new(
            vec![Action::Accelerate(Accelerate::new(1)), Action::Advance(Advance::new(2))]
        );

        let second_new_state: GameState = new_state.perform_move(second_move).unwrap();

        assert_eq!(second_new_state.determine_ahead_team().team, TeamEnum::Two);
    }
}
