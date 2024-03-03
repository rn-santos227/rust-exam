pub fn add(a: i32, b: i32) -> i32 {
    todo!("Implement add function for this test.")
}

pub fn subtract(a: i32, b: i32) -> i32 {
    todo!("Implement subtract function for this test.")
}

pub fn multiply(a: i32, b: i32) -> i32 {
    todo!("Implement multiply function for this test.")
}

pub fn divide(a: i32, b: i32) -> f32 {
    todo!("Implement divide function for this test.")
}

pub fn modulus(a: i32, b: i32) -> i32 {
    todo!("Implement modulus function for this test.")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(4, 4), 8);
        assert_eq!(add(3, 4), 7);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(4, 2), 2);
        assert_eq!(subtract(10, 2), 8);
        assert_eq!(subtract(9, 4), 5);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(6, 6), 36);
        assert_eq!(multiply(4, 2), 8);
        assert_eq!(multiply(9, 8), 72);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 5), 2.0);
        assert_eq!(divide(20, 4), 5.0);
        assert_eq!(divide(81, 3), 27.0);
    }

    #[test]
    fn test_modulus() {
        assert_eq!(modulus(5, 2), 1);
        assert_eq!(modulus(11, 3), 2);
        assert_eq!(modulus(29, 5), 4);
    }
}
