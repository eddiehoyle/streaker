#![allow(unused_imports)]

use std::fmt;
use regex::Regex;

const PATTERN_FRAME : &str = r"^(?P<frame>\d+)$";
const PATTERN_RANGE : &str = r"^(?P<start>\d+)-(?P<stop>\d+)$";
const PATTERN_FULL : &str = r"^(?P<start>\d+)-(?P<stop>\d+)(?P<body>[xy])(?P<step>\d+)$";

#[derive(Debug, PartialEq)]
enum Body {
    Fill,
    Inverse,
}

#[derive(Debug, PartialEq)]
pub struct PatternRange {
    start: u32,
    stop: u32,
    step: u32,
    body: Body,
}

impl PatternRange {

    /// Expects '#1-10' pattern
    fn from_pattern(pattern: &String) -> Result<PatternRange, String> {
        let mut range = pattern.clone();
        strip_padding(&mut range);
        let re = |s| { Regex::new(s).unwrap() };
        if let Some(cap) = re(PATTERN_FRAME).captures(range.as_str() ) {
            let frame = cap.name("frame").unwrap().as_str().parse::<u32>().unwrap();
            return Ok(PatternRange { start: frame, stop: frame, step: 1, body: Body::Fill });
        } else if let Some(cap) = re(PATTERN_RANGE).captures(range.as_str() ) {
            let start = cap.name("start").unwrap().as_str().parse::<u32>().unwrap();
            let stop = cap.name("stop").unwrap().as_str().parse::<u32>().unwrap();
            return Ok(PatternRange{ start: start, stop: stop, step: 1, body: Body::Fill});
        } else if let Some(cap) = re(PATTERN_FULL).captures(range.as_str() ) {
            let start = cap.name("start").unwrap().as_str().parse::<u32>().unwrap();
            let stop = cap.name("stop").unwrap().as_str().parse::<u32>().unwrap();
            let body = match cap.name("body").unwrap().as_str() {
                "x" => Body::Fill,
                "y" => Body::Inverse,
                _ => Body::Fill,
            };
            let step = cap.name("step").unwrap().as_str().parse::<u32>().unwrap();
            return Ok(PatternRange{ start: start, stop: stop, step, body});
        }
        Err(format!("Not a valid pattern."))
    }

    fn new(start: u32, stop: u32, skip: u32, surface: Body) -> Result<PatternRange, String> {
        if start <= stop {
            return Ok(PatternRange { start, stop, step: skip, body: surface })
        }
        Err(format!("Cannot construct with negative frame range! Got: {}, {}.", start, stop))
    }

    fn start(&self) -> &u32 {
        &self.start
    }

    fn stop(&self) -> &u32 {
        &self.stop
    }

    fn body(&self) -> &Body {
        &self.body
    }

    fn step(&self) -> &u32 {
        &self.step
    }
}

/// Strip '@' and '#' characters
fn strip_padding(pattern: &mut String){
    pattern.retain(|c| { c != '#' && c != '@' });
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_patternrange_from_pattern_frame() {
        let range = PatternRange::from_pattern(&String::from("1"));
        assert!(range.is_ok());
        let range = range.unwrap();
        assert_eq!(range.start, 1);
        assert_eq!(range.stop, 1);
        assert_eq!(range.body, Body::Fill);
        assert_eq!(range.step, 1);
    }

    #[test]
    fn test_patternrange_from_pattern_range() {
        let range = PatternRange::from_pattern(&String::from("1-10"));
        assert!(range.is_ok());
        let range = range.unwrap();
        assert_eq!(range.start, 1);
        assert_eq!(range.stop, 10);
        assert_eq!(range.body, Body::Fill);
        assert_eq!(range.step, 1);
    }

    #[test]
    fn test_patternrange_from_pattern_full() {
        let range = PatternRange::from_pattern(&String::from("1-10x2"));
        assert!(range.is_ok());
        let range = range.unwrap();
        assert_eq!(range.start, 1);
        assert_eq!(range.stop, 10);
        assert_eq!(range.body, Body::Fill);
        assert_eq!(range.step, 2);

        let range = PatternRange::from_pattern(&String::from("5-15y3"));
        assert!(range.is_ok());
        let range = range.unwrap();
        assert_eq!(range.start, 5);
        assert_eq!(range.stop, 15);
        assert_eq!(range.body, Body::Inverse);
        assert_eq!(range.step, 3);
    }

    #[test]
    fn test_patternrange_new() {
        assert!(PatternRange::new(1, 5, 1, Body::Fill ).is_ok());
        assert!(PatternRange::new(3, 3, 1, Body::Fill ).is_ok());
        assert!(PatternRange::new(5, 1, 1, Body::Fill ).is_err());
    }

    #[test]
    fn test_patternrange_get_begin() {
        let range = PatternRange::new(1, 10, 1, Body::Fill ).unwrap();
        assert_eq!(*range.start(), 1);
    }

    #[test]
    fn test_patternrange_get_end() {
        let range = PatternRange::new(1, 10, 1, Body::Fill ).unwrap();
        assert_eq!(*range.stop(), 10);
    }

    #[test]
    fn test_patternrange_get_skip() {
        let pattern = PatternRange::new(1, 10, 2, Body::Fill).unwrap();
        assert_eq!(pattern.step(), &2);
    }

    #[test]
    fn test_patternrange_get_body() {
        let pattern = PatternRange::new(1, 10, 2, Body::Fill).unwrap();
        assert_eq!(*pattern.body(), Body::Fill);
        let pattern = PatternRange::new(1, 10, 2, Body::Inverse).unwrap();
        assert_eq!(*pattern.body(), Body::Inverse);
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