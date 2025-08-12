#[cfg(test)]
mod tests {
    use crate::plugin2026::r#move::Move;
    use crate::plugin2026::rules_engine::RulesEngine;
    use crate::plugin2026::test::common::*;
    use crate::plugin2026::utils::coordinate::Coordinate;
    use crate::plugin2026::utils::direction::Direction;
    
    #[test]
    pub fn test01() {
        pyo3::prepare_freethreaded_python();

        let b = create_test_board();

        println!("{}", b);

        for variant in Direction::all_directions() {
            let c = Coordinate {x: 2, y: 0};
            let move_ = Move {start: c, direction: variant};

            println!("Move: {}", move_);

            if let Err(e) = RulesEngine::can_execute_move(&b, &move_) {
                println!("Error occurred: {:?}", e);
            } else {
                println!("Alles super");
            }

            println!();
        }
    }
}
