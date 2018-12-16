#![allow(unused_imports)]

//let re = Regex::new().unwrap();
const PATTERN_RANGE : &str = r"^\d+-\d+$";

use std::fmt;
use regex::Regex;

//type ParseResult<'a, T> = Result<T, &'a str>;

#[derive(Debug, PartialEq)]
enum Surface {
    Fill,
    Inverse,
}

pub struct PatternRange {
    begin: u32,
    end: u32,
    skip: u32,
    surface: Surface,
}

impl PatternRange {

    /// Expects '#1-10' pattern
    fn from_pattern(pattern: &String) -> Result<PatternRange, String> {
        let mut stripped = pattern.clone();
        strip_padding(&mut stripped);
        if let Some((begin, end)) = parse_pattern(&stripped) {
            return PatternRange::new(begin, end, 1, Surface::Fill );
        }
        Err(format!("Unrecognised pattern: {}", pattern))
    }

    fn new(begin: u32, end: u32, skip: u32, surface: Surface) -> Result<PatternRange, String> {
        if begin <= end {
            return Ok(PatternRange {begin, end, skip, surface })
        }
        Err(format!("Cannot construct with negative frame range! Got: {}, {}.", begin, end))
    }

    fn get_skip(&self) -> &u32 {
        &self.skip
    }

    fn get_begin(&self) -> &u32 {
        &self.begin
    }

    fn get_end(&self) -> &u32 {
        &self.end
    }

    fn get_surface(&self) -> &Surface {
        &self.surface
    }
}

/// Strip '@' and '#' characters
fn strip_padding(pattern: &mut String){
    pattern.retain(|c| { c != '#' && c != '@' });
}

/// Splits pattern on ',' and returns list of slices
/// that start with numbers
fn split_ranges(pattern: &String) -> Vec<String> {
    pattern.split(",")
        .filter(|piece| { char::is_numeric(piece.chars().next().unwrap()) })
        .map(|piece| {String::from(piece)})
        .collect()
}

/// Read pattern and extract padding
fn parse_padding(pattern: &str) -> u32 {
    pattern.chars().into_iter()
        .map(|chr| {
            match chr {
                '#' => 4,
                '@' => 1,
                _ => 0,
            }
        }).sum()
}

fn parse_pattern(pattern: &String) -> Option<(u32, u32)> {
    let re = Regex::new(PATTERN_RANGE).unwrap();
    if let Some(mat) = re.find(pattern) {
        let mat = String::from(mat.as_str());
        let pieces : Vec<&str> = mat.split('-').collect();
        let a = pieces[0].parse::<u32>().unwrap();
        let b = pieces[1].parse::<u32>().unwrap();
        return Some((a, b));
    }
    None
}

mod tests {

    use super::*;

    #[test]
    fn test_patternrange_new() {
        assert!(PatternRange::new(1, 10, 1, Surface::Fill ).is_ok());
        assert!(PatternRange::new(5, 1, 1, Surface::Fill ).is_err());
        assert!(PatternRange::new(3, 3, 1, Surface::Fill ).is_ok());
    }
    #[test]
    fn test_patternrange_from_pattern() {
        assert!(PatternRange::from_pattern(&String::from("#1-4")).is_ok());
        assert!(PatternRange::from_pattern(&String::from("@1-4")).is_ok());
        assert!(PatternRange::from_pattern(&String::from("1-4")).is_ok());
        assert!(PatternRange::from_pattern(&String::from("-1-4")).is_err());
    }

    #[test]
    #[test]
    fn test_patternrange_get_begin() {
        let range = PatternRange::new(1, 10, 1, Surface::Fill ).unwrap();
        assert_eq!(*range.get_begin(), 1);
    }

    #[test]
    fn test_patternrange_get_end() {
        let range = PatternRange::new(1, 10, 1, Surface::Fill ).unwrap();
        assert_eq!(*range.get_end(), 10);
    }

    #[test]
    fn test_patternrange_get_skip() {
        let pattern = PatternRange::new(1, 10, 2, Surface::Fill).unwrap();
        assert_eq!(*pattern.get_skip(), 2);
    }

    #[test]
    fn test_patternrange_get_surface() {
        let pattern = PatternRange::new(1, 10, 2, Surface::Fill).unwrap();
        assert_eq!(*pattern.get_surface(), Surface::Fill);
        let pattern = PatternRange::new(1, 10, 2, Surface::Inverse).unwrap();
        assert_eq!(*pattern.get_surface(), Surface::Inverse);
    }

    #[test]
    fn test_parse_padding() {
        assert_eq!(parse_padding(&String::from("#")), 4);
        assert_eq!(parse_padding(&String::from("@")), 1);
        assert_eq!(parse_padding(&String::from("##")), 8);
        assert_eq!(parse_padding(&String::from("@@")), 2);
        assert_eq!(parse_padding(&String::from("#@")), 5);
        assert_eq!(parse_padding(&String::from("@#")), 5);
        assert_eq!(parse_padding(&String::from("")), 0);
        assert_eq!(parse_padding(&String::from("jskdf")), 0);
    }

    #[test]
    fn test_parse_ranges() {
        assert_eq!(split_ranges(&String::from("1-5,7")), ["1-5", "7"]);
        assert_eq!(split_ranges(&String::from("1,4")), ["1", "4"]);
        assert_eq!(split_ranges(&String::from("a1,b4")), Vec::<String>::new());
    }

    #[test]
    fn test_parse_pattern() {
        assert_eq!(parse_pattern(&String::from("1-10")), Some((1, 10)));
        assert_eq!(parse_pattern(&String::from("1-1")), Some((1, 1)));
        assert_eq!(parse_pattern(&String::from("01-004")), Some((1, 4)));
        assert_eq!(parse_pattern(&String::from("-0-0")), None);
        assert_eq!(parse_pattern(&String::from("-1--4")), None);
        assert_eq!(parse_pattern(&String::from("1-")), None);
        assert_eq!(parse_pattern(&String::from("3-asd")), None);
        assert_eq!(parse_pattern(&String::from("3-asd")), None);
    }

    #[test]
    fn test_strip_padding() {
        let mut stripped = String::from("#");
        strip_padding(&mut stripped);
        assert_eq!(stripped, "");
        let mut stripped = String::from("@");
        strip_padding(&mut stripped);
        assert_eq!(stripped, "");
        let mut stripped = String::from("##");
        strip_padding(&mut stripped);
        assert_eq!(stripped, "");
        let mut stripped = String::from("@@");
        strip_padding(&mut stripped);
        assert_eq!(stripped, "");
        let mut stripped = String::from("#@");
        strip_padding(&mut stripped);
        assert_eq!(stripped, "");
        let mut stripped = String::from("@#");
        strip_padding(&mut stripped);
        assert_eq!(stripped, "");
        let mut stripped = String::from("");
        strip_padding(&mut stripped);
        assert_eq!(stripped, "");
        let mut stripped = String::from("asd");
        strip_padding(&mut stripped);
        assert_eq!(stripped, "asd");
        let mut stripped = String::from("#1-2");
        strip_padding(&mut stripped);
        assert_eq!(stripped, "1-2");
        let mut stripped = String::from("#1,3,7");
        strip_padding(&mut stripped);
        assert_eq!(stripped, "1,3,7");
    }
}