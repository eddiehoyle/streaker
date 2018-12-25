extern crate regex;

//mod range;
mod pattern;
mod streak;

//
//pub fn get_padding(num: &str) -> Result<u32, &str> {
//    let trimmed = &num.trim();
//    if let Ok(_) = trimmed.parse::<u32>() {
//        return Ok(trimmed.len() as u32);
//    }
//    Err("String was not a number.")
//}
//
//pub fn to_token(padding: u32) -> String {
//    let hashes = (0..(padding / 4)).map(|_| "#").collect::<String>();
//    let ats = (0..(padding % 4)).map(|_| "@").collect::<String>();
//    let token = format!("{}{}", hashes, ats);
//    token
//}
//
//#[cfg(test)]
//mod tests {
//    // Note this useful idiom: importing names from outer (for mod tests) scope.
//    use super::*;
//
//    #[test]
//    fn test_token_hashes() {
//        assert_eq!(to_token(4), "#");
//    }
//
//    #[test]
//    fn test_token_ats() {
//        assert_eq!(to_token(3), "@@@");
//    }
//
//    #[test]
//    fn test_token_both() {
//        assert_eq!(to_token(5), "#@");
//    }
//
//    #[test]
//    fn test_padding() {
//        assert_eq!(get_padding("0001").unwrap(), 4);
//    }
//
//    #[test]
//    fn test_max() {
//        assert_eq!(get_padding("123").unwrap(), 3);
//    }
//
//    #[test]
//    fn test_whitespace() {
//        assert_eq!(get_padding("    123").unwrap(), 3);
//    }
//
//    #[test]
//    fn test_negative() {
//        assert!(get_padding("-554").is_err());
//    }
//
//
//    #[test]
//    fn test_malformed() {
//        assert!(get_padding("butt").is_err());
//    }
//}