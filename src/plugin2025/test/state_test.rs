#[cfg(test)]
mod tests {
    use std::vec;

    use crate::plugin2025::{
        action::{advance::Advance, card::Card, Action},
        board::Board,
        field::Field,
        game_state::GameState,
        hare::{Hare, TeamEnum},
        r#move::Move,
        rules_engine::RulesEngine,
    };

    fn create_player(
        team: TeamEnum,
        position: usize,
        cards: Vec<Card>,
        carrots: i32,
        salads: i32,
    ) -> Hare {
        Hare::new(
            team,
            Some(cards),
            Some(carrots),
            Some(salads),
            None,
            Some(position),
        )
    }

    fn create_board() -> Board {
        Board::new(vec![
            Field::Start,
            Field::Carrots,
            Field::Position2,
            Field::Hare,
            Field::Position1,
            Field::Market,
            Field::Carrots,
            Field::Hare,
            Field::Hedgehog,
            Field::Salad,
            Field::Goal,
        ])
    }

    #[test]
    fn test_possible_advance_moves_with_one_card() {
        let state = GameState::new(
            create_board(),
            20,
            create_player(TeamEnum::One, 2, vec![Card::EatSalad], 37, 1),
            create_player(TeamEnum::Two, 6, vec![], 11, 1),
            None,
        );
        let moves = state.possible_moves();
        assert!(moves.contains(&Move::new(Action::Advance(Advance::new(
            1,
            vec![Card::EatSalad]
        )))));
    }

    #[test]
    fn test_possible_advance_moves_with_hurry_ahead_back_and_market() {
        let state = GameState::new(
            create_board(),
            20,
            create_player(
                TeamEnum::One,
                2,
                vec![Card::HurryAhead, Card::FallBack],
                37,
                0,
            ),
            create_player(TeamEnum::Two, 6, vec![], 11, 0),
            None,
        );
        let moves = state.possible_moves();

        assert!(moves.contains(&Move::new(Action::Advance(Advance::new(
            1,
            vec![Card::HurryAhead, Card::FallBack, Card::EatSalad]
        )))));
    }

    #[test]
    fn test_correct_carrot_update() {
        let state_depth_0 = GameState::new(
            create_board(),
            0,
            create_player(TeamEnum::One, 0, vec![], 75, 0),
            create_player(TeamEnum::Two, 0, vec![], 200, 0),
            None,
        );

        // perform all poss moves for current player ("A")
        let moves_depth_0 = state_depth_0.possible_moves();
        for move_depth_0 in moves_depth_0.iter() {
            let depth_1 = state_depth_0.perform_move(move_depth_0);
            assert!(depth_1.is_ok());

            match depth_1 {
                Ok(state_depth_1) => {
                    let moves_depth_1 = state_depth_1.possible_moves();
                    let ref move_first_depth_1 = moves_depth_1[0];
                    let ref move_last_depth_1 = moves_depth_1[moves_depth_1.len() - 1];
                    
                    // performed player "A" on Pos1 or Pos2 Field -> calculate next depth (player B)
                    let on_pos1 = state_depth_1.board.get_field(state_depth_1.clone_other_player().position) == Some(Field::Position1);
                    let on_pos2 = state_depth_1.board.get_field(state_depth_1.clone_other_player().position) == Some(Field::Position2);
                    if on_pos1 || on_pos2 {

                        let moved_distance = match &move_depth_0.action {
                            Action::Advance(advance) => advance.distance,
                            _ => 0,
                        };

                        let expected_carrots = state_depth_0.clone_current_player().carrots - RulesEngine::calculates_carrots(moved_distance);
                        
                        // player "A" should be missing the exact carrot amount for the distance
                        assert_eq!(expected_carrots, state_depth_1.clone_other_player().carrots);

                        // first (shortest) poss move of player "B" gets performed -> A is (with this board) in front
                        let depth_2_first = state_depth_1.perform_move(move_first_depth_1);
                        assert!(depth_2_first.is_ok());
                        match depth_2_first {
                            Ok(state_depth_2_first) => {
                                // "A" got the 10 ten extra carrots if on pos1 field and in front
                                if on_pos1 {
                                    assert_eq!(expected_carrots + 10, state_depth_2_first.clone_current_player().carrots);
                                }

                                // no carrots should have been added to "A" if on pos2 field and in front
                                if on_pos2 {
                                    assert_eq!(expected_carrots, state_depth_2_first.clone_current_player().carrots);
                                }
                            }
                            Err(e) => println!("Error {e}")
                        }

                        // last (farthest) poss move of player "B" gets performed -> A is (with this board) behind
                        let depth_2_last = state_depth_1.perform_move(move_last_depth_1);
                        assert!(depth_2_last.is_ok());
                        match depth_2_last {
                            Ok(state_depth_2_last) => {
                                // no carrots should have been added to "A" if on pos1 field and behind
                                if on_pos1 {
                                    assert_eq!(expected_carrots, state_depth_2_last.clone_current_player().carrots);
                                }

                                // "A" got the 30 ten extra carrots if on pos2 field and behind
                                if on_pos2 {
                                    assert_eq!(expected_carrots + 30, state_depth_2_last.clone_current_player().carrots);
                                }
                            }
                            Err(e) => println!("Error {e}")
                        }
                    }
                },
                Err(e) => println!("Error {e}")
            }
        }
    }
}
