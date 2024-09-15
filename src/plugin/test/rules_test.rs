#[cfg(test)]
mod tests {
    use crate::plugin::{
        action::card::Card,
        board::Board,
        field::Field,
        hare::{Hare, TeamEnum},
        rules_engine::RulesEngine,
    };

    fn create_player(team: TeamEnum, position: usize) -> Hare {
        Hare::new(
            team,
            Some(vec![Card::FallBack, Card::EatSalad, Card::SwapCarrots]),
            Some(60),
            Some(3),
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
            Field::Hare,
            Field::Position1,
            Field::Goal,
        ])
    }

    #[test]
    fn test_calculates_carrots() {
        assert_eq!(RulesEngine::calculates_carrots(0), 0);
        assert_eq!(RulesEngine::calculates_carrots(1), 1);
        assert_eq!(RulesEngine::calculates_carrots(2), 3);
        assert_eq!(RulesEngine::calculates_carrots(3), 6);
        assert_eq!(RulesEngine::calculates_carrots(4), 10);
    }

    #[test]
    fn test_can_exchange_carrots() {
        let board = create_board();
        let mut player = create_player(TeamEnum::One, 4);

        let result = RulesEngine::can_exchange_carrots(&board, &player, 10);
        assert!(result.is_ok());

        player.carrots = 5;
        let result = RulesEngine::can_exchange_carrots(&board, &player, -10);
        assert!(result.is_err());

        player.carrots = 10;
        let result = RulesEngine::can_exchange_carrots(&board, &player, -10);
        assert!(result.is_ok());

        let result = RulesEngine::can_exchange_carrots(&board, &player, 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_can_eat_salad() {
        let board = create_board();
        let player = create_player(TeamEnum::One, 1);

        let result = RulesEngine::can_eat_salad(&board, &player);
        assert!(result.is_ok());
    }

    #[test]
    fn test_can_advance_to() {
        let board = create_board();
        let mut player_one = create_player(TeamEnum::One, 0);
        let player_two = create_player(TeamEnum::Two, 2);

        assert!(RulesEngine::can_move_to(&board, 3, &player_one, &player_two, vec![Card::FallBack]).is_ok());

        assert!(RulesEngine::can_move_to(&board, 2, &player_one, &player_two, vec![]).is_err());

        player_one.carrots = 1;
        assert!(RulesEngine::can_move_to(&board, 5, &player_one, &player_two, vec![]).is_err());

        player_one.cards = vec![];
        assert!(RulesEngine::can_move_to(&board, 6, &player_one, &player_two, vec![]).is_err());
    }
}
