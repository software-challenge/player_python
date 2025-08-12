#[cfg(test)]
mod tests {
    use crate::plugin2026::field_type::FieldType;
    use crate::plugin2026::test::common::*;
    use crate::plugin2026::utils::coordinate::Coordinate;
    use crate::plugin2026::utils::direction::Direction;
    
    #[test]
    pub fn test01() {
        let b = create_test_board();

        println!("{}", b);

        for variant in Direction::all_directions() {
            let c = Coordinate {x: 2, y: 1};
            let f = b.get_fields_in_direction(&c, &variant);
            
            print!("[ ");
            for field in &f {
                print!("{} ", field);
            }
            print!("] {} {}", c, variant);
            println!();
        }

        let mut sum = 0;

        for fvarient in FieldType::all_field_types() {
            let f = b.get_fields_by_type(fvarient);
            
            print!("[ ");
            for field in &f {
                print!("{} ", field);
            }
            print!("]");
            println!();

            sum += f.len();
        }

        println!("{}", sum);
    }
}
