#[cfg(test)]
mod tests {
    use crate::plugin2026::{
        utils::coordinate::Coordinate,
        utils::vector::Vector
    };
    
    #[test]
    pub fn test_eq() {
        let coord01 = Coordinate {x: 1, y: 2};
        let coord02 = Coordinate {x: 1, y: 2};
        let coord03 = Coordinate {x: 2, y: 1};
        let coord04 = Coordinate {x: 1, y: 1};

        assert_eq!(coord01, coord02);
        assert_ne!(coord02, coord03);
        assert_ne!(coord03, coord04);
    }

    #[test]
    pub fn test_add_vector() {
        let coord01 = Coordinate {x: 1, y: 2};
        let coord02 = Coordinate {x: 1, y: 2};
        let vec01 = Vector {delta_x: 2, delta_y: 3};
        let vec02 = Vector {delta_x: -1, delta_y: 1};

        println!("{}", coord01.add_vector(&vec01));
        println!("{}", coord02.add_vector(&vec02));
    }
}
