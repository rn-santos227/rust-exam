pub fn parse_u32(a: &str, b: &u32) -> bool {
    todo!("Parse string to unsigned integer 32 for this test, return true if matched.")
}

pub fn parse_u128(a: u32, b: u128) -> bool {
    todo!("Parse unsiged integer 32 to unsigned integer 128 for this test, return true if matched.")
}

pub fn parse_i32(a: u32, b: i32) -> bool {
    todo!("Parse string to signed integer 32 for this test, return true if matched.")
}

pub fn parse_f32(a: u32, b: f32) -> bool {
    todo!("Parse unsigned integer 32 to float 32 for this test, return true if matched.")
}

pub fn parse_string(a: &u32, b: &str) -> bool {
    todo!("Parse unsigned integer 32 to string for this test, return true if matched.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_parse() {
        assert_eq!(parse_u32("2", &2), true);
        assert_eq!(parse_u32("2024", &2024), true);
    }

    #[test]
    fn test_u128_parse() {
        let (a, c): (u32, u32) = (12, 60);
        let (b, d): (u128, u128) = (12, 60);
        assert_eq!(parse_u128(a, b), true);
        assert_eq!(parse_u128(c, d), true);
    }

    #[test]
    fn test_i32_parse() {
        let (a, c): (u32, u32) = (1, 55);
        let (b, d): (i32, i32) = (-1, -55);
        assert_eq!(parse_i32(a, b), true);
    }

    #[test]
    fn test_f32_parse() {
        assert_eq!(parse_f32(32, 32.0), true);
        assert_eq!(parse_f32(14, 14.0), true);
    }

    #[test]
    fn test_string_parse() {
        assert_eq!(parse_string(&5, "5"), true);
        assert_eq!(parse_string(&10, "10"), true);
    }
}