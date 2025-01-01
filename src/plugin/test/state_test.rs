#[cfg(test)]
mod tests {
    use std::vec;

    use crate::plugin::{
        action::{advance::Advance, card::Card, Action},
        board::Board,
        field::Field,
        game_state::GameState,
        hare::{Hare, TeamEnum},
        r#move::Move,
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
            Field::Salad,
            Field::Position2,
            Field::Hare,
            Field::Carrots,
            Field::Market,
            Field::Position1,
            Field::Hare,
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
}
