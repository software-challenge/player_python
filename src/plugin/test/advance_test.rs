#[cfg(test)]
mod tests {
    use crate::plugin::{
        action::{advance::Advance, card::Card},
        board::Board,
        field::Field,
        game_state::GameState,
        hare::{Hare, TeamEnum},
    };

    fn create_test_game_state() -> GameState {
        let board = Board::new(vec![
            Field::Start,
            Field::Position1,
            Field::Position2,
            Field::Hare,
            Field::Hedgehog,
            Field::Market,
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
            Some(4),
        );
        let player_two = Hare::new(
            TeamEnum::Two,
            Some(vec![Card::HurryAhead]),
            Some(60),
            Some(3),
            None,
            Some(3),
        );
        GameState::new(board, 0, player_one, player_two)
    }

    #[test]
    fn test_new() {
        let cards = vec![Card::FallBack];
        let advance = Advance::new(5, cards.clone());
        assert_eq!(advance.distance, 5);
        assert_eq!(advance.cards, cards);
    }

    #[test]
    fn test_perform_success() {
        let cards = vec![Card::FallBack];
        let advance = Advance::new(2, cards.clone());

        let mut state = create_test_game_state();

        let result = advance.perform(&mut state);
        assert!(result.is_ok());

        let current_player = state.clone_current_player();
        assert_eq!(current_player.position, 6);
    }

    #[test]
    fn test_perform_success_without_cards() {
        let board = Board::new(vec![
            Field::Start,
            Field::Position1,
            Field::Position2,
            Field::Hare,
            Field::Hedgehog,
            Field::Market,
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
            Some(0),
        );
        let player_two = Hare::new(
            TeamEnum::Two,
            Some(vec![Card::HurryAhead]),
            Some(60),
            Some(3),
            None,
            Some(0),
        );

        let mut state = GameState::new(board, 0, player_one, player_two);

        let advance = Advance::new(2, vec![]);

        let result = advance.perform(&mut state);
        assert!(result.is_ok());

        let current_player = state.clone_current_player();
        assert_eq!(current_player.position, 2);
    }

    #[test]
    fn test_perform_buy_card_success() {
        let cards = vec![Card::HurryAhead];
        let advance = Advance::new(2, cards.clone());

        let mut state = create_test_game_state();
        state.turn = 1;

        let result = advance.perform(&mut state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_perform_buy_card_error() {
        let cards = vec![Card::HurryAhead, Card::FallBack];
        let advance = Advance::new(2, cards.clone());

        let mut state = create_test_game_state();
        state.turn = 1;

        let result = advance.perform(&mut state);
        assert!(result.is_err());
    }

    #[test]
    fn test_perform_cannot_play_card_error() {}
}
