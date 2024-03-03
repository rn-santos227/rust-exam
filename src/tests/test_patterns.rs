
pub fn pattern_1(input: Vec<String>) -> bool {
	todo!("Returns true if the last two strings in the vector start with `BLOKC`.");
}

pub fn pattern_2(input: Vec<String>) -> bool {
    todo!("Returns true if the first and last string in the vector start with `BLOKC`.");
}

pub fn pattern_3(input: &str) -> bool {
    todo!("Returns true if a string that contains all the letters of the word 'BLOKC'");
}

pub fn pattern_4(input: &str) -> &str {
    todo!("Returns a string that rearranges its characters in alphabetic order");
}

pub fn pattern_5(input: &str) -> &str {
    todo!("Returns a string that had the following characters converted similar to the first character");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
	fn test_pattern_1() {
		let strs_1 = vec![
			"HELLO".to_string(),
			"RUST".to_string(),
			"BLOKC".to_string(),
			"BLOCKCHANG".to_string(),
		];
		assert!(pattern_1(strs_1));

        let strs_2 = vec![
			"BLOKCS".to_string(),
			"WORLD".to_string(),
			"BLOKC".to_string(),
		];
		assert!(!pattern_1(strs_2));
	}

    #[test]
    fn test_pattern_2() {
        let strs_1 = vec![
			"HELLO".to_string(),
			"RUST".to_string(),
			"BLOKC".to_string(),
			"BLOCKCHANG".to_string(),
		];
		assert!(!pattern_2(strs_1));

        let strs_2 = vec![
			"BLOKCS".to_string(),
			"WORLD".to_string(),
			"BLOKC".to_string(),
		];
		assert!(pattern_2(strs_2));
    }

    #[test]
    fn test_pattern_3() {
		assert!(pattern_3("BLOKCBUSTER"));
		assert!(pattern_3("THEBLOKC"));
        assert!(pattern_3("KCOLB"));
        assert!(pattern_3("B*L*O*C*K"));
        assert!(!pattern_3("BLOB"));
        assert!(!pattern_3("IDONTKNOW"));
    }

    #[test]
    fn test_pattern_4() {
        assert_eq!(pattern_4("BLOCK"), "BCKLMO");
        assert_eq!(pattern_4("HELLO"), "EHLLO");
        assert_eq!(pattern_4("EDCBA"), "ABCDE");
    }

    #[test]
    fn test_pattern_5() {
        assert_eq!(pattern_5("BLOCK"), "BBBBB");
        assert_eq!(pattern_5("HELLO"), "HHHHH");
        assert_eq!(pattern_5("RUST"), "RRRR");
        assert_eq!(pattern_5("CONGRATS"), "CCCCCCCC");
    }
}