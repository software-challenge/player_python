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
            Field::Position1,
            Field::Position2,
            Field::Hare,
            Field::Hedgehog,
            Field::Salad,
            Field::Hare,
            Field::Position1,
            Field::Goal,
        ]);
        let player_one = Hare::new(
            TeamEnum::One,
            Some(vec![Card::FallBack, Card::EatSalad, Card::SwapCarrots]),
            Some(60),
            Some(3),
            None,
            Some(6),
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
            .perform(&mut state, vec![Card::EatSalad, Card::SwapCarrots])
            .is_ok());
        let current_player = state.clone_current_player();
        assert_eq!(current_player.position, 2);
    }

    #[test]
    fn test_hurryahead_card() {
        let mut state = create_test_game_state();
        state.turn = 1;
        let hurry_ahead_card: Card = Card::HurryAhead;
        assert!(hurry_ahead_card.perform(&mut state, vec![]).is_ok());
        let current_player = state.clone_current_player();
        assert_eq!(current_player.position, 7);
    }

    #[test]
    fn test_eatsalad_card() {
        let mut state = create_test_game_state();
        let eat_salad_card = Card::EatSalad;
        assert!(eat_salad_card
            .perform(&mut state, vec![Card::FallBack, Card::SwapCarrots])
            .is_ok());
        let current_player = state.clone_current_player();
        assert_eq!(current_player.salads, 2);
    }

    #[test]
    fn test_swapcarrots_card() {
        let mut state = create_test_game_state();
        let mut player_one = state.clone_current_player();
        let mut player_two = state.clone_other_player();
        player_one.position = 3;
        player_two.position = 2;
        state.update_player(player_one);
        state.update_player(player_two);

        let swap_carrots_card = Card::SwapCarrots;
        assert!(swap_carrots_card
            .perform(&mut state, vec![Card::FallBack, Card::EatSalad])
            .is_ok());
        let current_player = state.clone_current_player();
        let other_player = state.clone_other_player();
        assert_eq!(current_player.carrots, 60);
        assert_eq!(other_player.carrots, 60);
    }

    #[test]
    fn test_swapcarrots_card_already_occurred_last_two_rounds() {
        let mut state = create_test_game_state();
        let mut player_one = state.clone_current_player();
        let mut player_two = state.clone_other_player();
        player_one.position = 3;
        player_two.position = 2;
        player_one.last_move = Some(Move {
            action: Action::Advance(Advance {
                distance: 1,
                cards: vec![Card::SwapCarrots],
            }),
        });
        player_two.last_move = Some(Move {
            action: Action::Advance(Advance {
                distance: 1,
                cards: vec![Card::SwapCarrots],
            }),
        });
        state.update_player(player_one);
        state.update_player(player_two);

        let swap_carrots_card = Card::SwapCarrots;
        let result = swap_carrots_card.perform(&mut state, vec![Card::FallBack, Card::EatSalad]);
        assert!(result.is_err());
    }

    #[test]
    fn test_play_card_not_owned() {
        let mut state = create_test_game_state();
        state.turn = 1;
        let card_not_owned = Card::FallBack;
        let result = card_not_owned.perform(&mut state, vec![Card::HurryAhead]);
        assert!(result.is_err());
    }

    #[test]
    fn test_play_card_not_on_hare_field() {
        let mut state = create_test_game_state();
        let _card = Card::FallBack;
        let mut current_player = state.clone_current_player();
        current_player.position = 1;
        state.update_player(current_player);
    }

    #[test]
    fn test_invalid_field() {
        let mut state = create_test_game_state();
        let invalid_card = Card::FallBack;
        state.board.track.clear();
        let result = invalid_card.perform(&mut state, vec![Card::EatSalad, Card::SwapCarrots]);
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
        let result = card.perform(&mut state, vec![]);
        assert!(result.is_err());
    }
}
