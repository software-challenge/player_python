use crate::plugin2026::{
    board::Board,
    field_type::FieldType,
    utils::constants::PluginConstants
};

pub fn create_test_board() -> Board {
    let example_map = [
        ["--", "T2", "T1", "T3", "T3", "T2", "T2", "T1", "T1", "--"],
        ["O1", "--", "--", "--", "--", "--", "--", "--", "--", "O1"],
        ["O3", "--", "--", "SQ", "--", "--", "--", "--", "--", "O1"],
        ["O1", "--", "--", "--", "--", "--", "--", "--", "--", "O2"],
        ["O2", "--", "--", "--", "--", "--", "--", "--", "--", "O2"],
        ["O1", "--", "--", "--", "--", "--", "--", "--", "--", "O3"],
        ["O3", "--", "--", "--", "--", "--", "--", "--", "--", "O3"],
        ["O1", "--", "--", "--", "--", "--", "SQ", "--", "--", "O1"],
        ["O2", "--", "--", "--", "--", "--", "--", "--", "--", "O2"],
        ["--", "T2", "T1", "T3", "T1", "T2", "T1", "T3", "T1", "--"],
    ];

    let mut new_map = vec![vec![FieldType::Empty; PluginConstants::BOARD_HEIGHT]; PluginConstants::BOARD_WIDTH];

    for y in 0..PluginConstants::BOARD_HEIGHT {
        for x in 0..PluginConstants::BOARD_WIDTH {            
            match example_map[9 - y][x] {       // read in reverse y because board is read from bottom to top
                "O1" => {new_map[y][x] = FieldType::OneS},
                "O2" => {new_map[y][x] = FieldType::OneM},
                "O3" => {new_map[y][x] = FieldType::OneL},
                "T1" => {new_map[y][x] = FieldType::TwoS},
                "T2" => {new_map[y][x] = FieldType::TwoM},
                "T3" => {new_map[y][x] = FieldType::TwoL},
                "SQ" => {new_map[y][x] = FieldType::Squid},
                "--" => {}, // already FieldType::Empty
                _ => {}
            }
        }
    }

    Board::new(new_map)
}