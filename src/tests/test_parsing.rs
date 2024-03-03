pub fn parse_u32(input: &str) -> u32 {
    todo!("Parse string to unsigned integer 32 for this test.")
}

pub fn parse_u128(input: u32) -> u128 {
    todo!("Parse unsiged integer 32 to unsigned integer 128 for this test.")
}

pub fn parse_i32(input: u32) -> i32 {
    todo!("Parse string to signed integer 32 for this test.")
}

pub fn pars_f32(input: u32) -> f32 {
    todo!("Parse unsigned integer 32 to float 32 for this test.")
}

pub fn parse_string(input: &u32) -> &str {
    todo!("Parse unsigned integer 32 to string for this test.")
}

#[cfg(test)]
mod tests {
    use crate::tests::test_parsing::parse_u32;
    use crate::tests::test_parsing::parse_u128;
    use crate::tests::test_parsing::parse_i32;

    #[test]
    fn test_u32_parse() {
        let var_test: u32 = 2;
        assert_eq!(parse_u32("2"), var_test);
    }

    #[test]
    fn test_u128_parse() {
        let var_test: u128 = 12;
        let var_input: u32 = 12;
        assert_eq!(parse_u128(var_input), var_test);
    }

    #[test]
    fn test_i32_parse() {
        let var_test: i32 = -1;
        let var_input: u32 = 1;
        assert_eq!(parse_i32(var_input), var_test);
    }
}