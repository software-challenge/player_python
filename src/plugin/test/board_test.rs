#[cfg(test)]
mod tests {
    use crate::plugin::{board::Board, field::Field};

    #[test]
    fn test_new_board() {
        let fields = vec![
            Field::Start,
            Field::Position1,
            Field::Position2,
            Field::Goal,
        ];
        let board = Board::new(fields.clone());
        assert_eq!(board.track, fields);
    }

    #[test]
    fn test_get_field() {
        let fields = vec![
            Field::Start,
            Field::Position1,
            Field::Position2,
            Field::Goal,
        ];
        let board = Board::new(fields);
        assert_eq!(board.get_field(0), Some(Field::Start));
        assert_eq!(board.get_field(2), Some(Field::Position2));
        assert_eq!(board.get_field(4), None);
    }

    #[test]
    fn test_find_field() {
        let fields = vec![
            Field::Start,
            Field::Position1,
            Field::Position2,
            Field::Goal,
        ];
        let board = Board::new(fields);
        assert_eq!(board.find_field(Field::Position1, 0, 4), Some(1));
        assert_eq!(board.find_field(Field::Goal, 1, 4), Some(3));
        assert_eq!(board.find_field(Field::Hedgehog, 0, 4), None);
        assert_eq!(board.find_field(Field::Position1, 2, 4), None);
    }

    #[test]
    fn test_get_previous_field() {
        let fields = vec![
            Field::Start,
            Field::Position1,
            Field::Position2,
            Field::Position1,
            Field::Goal,
        ];
        let board = Board::new(fields);
        assert_eq!(board.get_previous_field(Field::Position1, 3), Some(1));
        assert_eq!(board.get_previous_field(Field::Start, 4), Some(0));
        assert_eq!(board.get_previous_field(Field::Goal, 4), None);
        assert_eq!(board.get_previous_field(Field::Position2, 2), None);
        assert_eq!(board.get_previous_field(Field::Position2, 3), Some(2));
    }

    #[test]
    fn test_get_next_field() {
        let fields = vec![
            Field::Start,
            Field::Position1,
            Field::Position2,
            Field::Position1,
            Field::Goal,
        ];
        let board = Board::new(fields);
        assert_eq!(board.get_next_field(Field::Position1, 1), Some(3));
        assert_eq!(board.get_next_field(Field::Start, 0), None);
        assert_eq!(board.get_next_field(Field::Position2, 2), None);
        assert_eq!(board.get_next_field(Field::Goal, 3), Some(4));
        assert_eq!(board.get_next_field(Field::Goal, 4), None);
        assert_eq!(board.get_next_field(Field::Position1, 2), Some(3));
    }
}
