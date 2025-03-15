#[cfg(test)]
mod tests {
    use crate::plugin::{
        action::{advance::Advance, card::Card, Action},
        board::Board,
        field::Field,
        game_state::GameState,
        hare::{Hare, TeamEnum},
        r#move::Move,
    };

    fn create_test_game_state() -> GameState {
        let board = Board::new(vec![
            Field::Start,
            Field::Carrots,
            Field::Position2,
            Field::Hare,
            Field::Position1,
            Field::Market,
            Field::Carrots,
            Field::Hare,
            Field::Carrots,
            Field::Hedgehog,
            Field::Salad,
            Field::Goal,
        ]);
        let player_one = Hare::new(
            TeamEnum::One,
            Some(vec![Card::FallBack, Card::EatSalad, Card::SwapCarrots]),
            Some(30),
            Some(3),
            None,
            Some(7),
        );
        let player_two = Hare::new(
            TeamEnum::Two,
            Some(vec![Card::HurryAhead]),
            Some(60),
            Some(3),
            None,
            Some(3),
        );
        GameState::new(board, 0, player_one, player_two, None)
    }

    #[test]
    fn test_fallback_card() {
        let mut state = create_test_game_state();
        let fallback_card = Card::FallBack;
        assert!(fallback_card
            .perform(&mut state, vec![Card::EatSalad, Card::SwapCarrots], 0)
            .is_ok());
        let current_player = state.clone_current_player();
        assert_eq!(current_player.position, 2);
    }

    #[test]
    fn test_hurryahead_card() {
        let mut state = create_test_game_state();
        state.turn = 1;
        let hurry_ahead_card: Card = Card::HurryAhead;
        assert!(hurry_ahead_card.perform(&mut state, vec![], 0).is_ok());
        let current_player = state.clone_current_player();
        assert_eq!(current_player.position, 8);
    }

    #[test]
    fn test_eatsalad_card() {
        let mut state = create_test_game_state();
        let eat_salad_card = Card::EatSalad;
        assert!(eat_salad_card
            .perform(&mut state, vec![Card::FallBack, Card::SwapCarrots], 0)
            .is_ok());
        let current_player = state.clone_current_player();
        assert_eq!(current_player.salads, 2);
    }

    #[test]
    fn test_swapcarrots_card_general() {
        let mut state = create_test_game_state();

        // modify player one
        let mut player_one = state.clone_current_player();
        player_one.last_move = Some(Move {
            action: Action::Advance(Advance {
                distance: 2,
                cards: vec![],
            }),
        });

        state.update_player(player_one);

        // modify player two
        let mut player_two = state.clone_other_player();
        player_two.last_move = Some(Move {
            action: Action::Advance(Advance {
                distance: 3,
                cards: vec![],
            }),
        });
        
        state.update_player(player_two);

        // test card
        let swap_carrots_card = Card::SwapCarrots;
        assert!(swap_carrots_card
            .perform(&mut state, vec![Card::FallBack, Card::EatSalad], 1)
            .is_ok());
        let current_player = state.clone_current_player();
        let other_player = state.clone_other_player();
        assert_eq!(current_player.carrots, 60);
        assert_eq!(other_player.carrots, 30);
    }

    #[test]
    fn test_swapcarrots_card_bought_last_two_rounds() {
        let mut state = create_test_game_state();

        // modify player one
        let mut player_one = state.clone_current_player();
        player_one.last_move = Some(Move {
            action: Action::Advance(Advance {
                distance: 1,
                cards: vec![Card::SwapCarrots],
            }),
        });

        state.update_player(player_one);

        // modify player two
        let mut player_two = state.clone_other_player();
        player_two.last_move = Some(Move {
            action: Action::Advance(Advance {
                distance: 3,
                cards: vec![],
            }),
        });
        
        state.update_player(player_two);

        // test card
        let swap_carrots_card = Card::SwapCarrots;
        assert!(swap_carrots_card
            .perform(&mut state, vec![Card::FallBack, Card::EatSalad], 2)
            .is_ok());
        let current_player = state.clone_current_player();
        let other_player = state.clone_other_player();
        assert_eq!(current_player.carrots, 60);
        assert_eq!(other_player.carrots, 30);
    }

    #[test]
    fn test_swapcarrots_card_played_last_two_rounds() {
        let mut state = create_test_game_state();

        // modify player one
        let mut player_one = state.clone_current_player();
        player_one.last_move = Some(Move {
            action: Action::Advance(Advance {
                distance: 2,
                cards: vec![],
            }),
        });

        state.update_player(player_one);

        // modify player two
        let mut player_two = state.clone_other_player();
        player_two.last_move = Some(Move {
            action: Action::Advance(Advance {
                distance: 3,
                cards: vec![Card::SwapCarrots],
            }),
        });
        
        state.update_player(player_two);

        // test card
        let swap_carrots_card = Card::SwapCarrots;
        assert!(swap_carrots_card
            .perform(&mut state, vec![Card::FallBack, Card::EatSalad], 1)
            .is_err());
    }

    #[test]
    fn test_play_card_not_owned() {
        let mut state = create_test_game_state();
        state.turn = 1;
        let card_not_owned = Card::FallBack;
        let result = card_not_owned.perform(&mut state, vec![Card::HurryAhead], 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_play_card_not_on_hare_field() {
        let mut state = create_test_game_state();
        let card = Card::FallBack;
        let mut current_player = state.clone_current_player();
        current_player.position = 1;
        state.update_player(current_player);
        let result = card.perform(&mut state, vec![Card::EatSalad, Card::SwapCarrots], 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_field() {
        let mut state = create_test_game_state();
        let invalid_card = Card::FallBack;
        state.board.track.clear();
        let result = invalid_card.perform(&mut state, vec![Card::EatSalad, Card::SwapCarrots], 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_no_salad_but_salad_card() {
        let mut state = create_test_game_state();
        let card = Card::EatSalad;
        let mut current_player = state.clone_current_player();
        current_player.salads = 0;
        current_player.cards = vec![card];
        state.update_player(current_player);
        let result = card.perform(&mut state, vec![], 0);
        assert!(result.is_err());
    }
}
