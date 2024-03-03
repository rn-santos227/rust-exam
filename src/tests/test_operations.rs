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
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(4, 2), 2);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(6, 6), 36);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 5), 2.0);
    }

    #[test]
    fn test_modulus() {
        assert_eq!(modulus(5, 2), 1);
    }
}
