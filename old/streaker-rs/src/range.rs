
use std::fmt;
use regex::Regex;

const PATTERN_FRAME : &str = r"^(?P<frame>\d+)$";
const PATTERN_RANGE : &str = r"^(?P<start>\d+)-(?P<stop>\d+)$";
const PATTERN_FULL : &str = r"^(?P<start>\d+)-(?P<stop>\d+)(?P<body>[xy])(?P<step>\d+)$";

#[derive(Debug, PartialEq)]
pub enum Body {
    Fill,
    Inverse,
}

#[derive(Debug, PartialEq)]
pub struct Range {
    start: u32,
    stop: u32,
    step: usize,
    body: Body,
}

impl Range {

    /// Expects '#1-10' pattern
    pub fn from_pattern(pattern: &String) -> Result<Range, String> {
        let re = |s| { Regex::new(s).unwrap() };
        if let Some(cap) = re(PATTERN_FRAME).captures(pattern.as_str() ) {
            let frame = cap.name("frame").unwrap().as_str().parse::<u32>().unwrap();
            return Ok(Range { start: frame, stop: frame, step: 1, body: Body::Fill });
        } else if let Some(cap) = re(PATTERN_RANGE).captures(pattern.as_str() ) {
            let start = cap.name("start").unwrap().as_str().parse::<u32>().unwrap();
            let stop = cap.name("stop").unwrap().as_str().parse::<u32>().unwrap();
            return Ok(Range { start: start, stop: stop, step: 1, body: Body::Fill});
        } else if let Some(cap) = re(PATTERN_FULL).captures(pattern.as_str() ) {
            let start = cap.name("start").unwrap().as_str().parse::<u32>().unwrap();
            let stop = cap.name("stop").unwrap().as_str().parse::<u32>().unwrap();
            let body = match cap.name("body").unwrap().as_str() {
                "x" => Body::Fill,
                "y" => Body::Inverse,
                _ => Body::Fill,
            };
            let step = cap.name("step").unwrap().as_str().parse::<usize>().unwrap();
            return Ok(Range { start: start, stop: stop, step, body});
        }
        Err(format!("Not a valid pattern."))
    }

    pub fn new(start: u32, stop: u32, step: usize, body: Body) -> Result<Range, String> {
        if start <= stop {
            return Ok(Range { start, stop, step, body })
        }
        Err(format!("Cannot construct with negative frame range! Got: {}, {}.", start, stop))
    }

    pub fn start(&self) -> &u32 { &self.start }

    pub fn stop(&self) -> &u32 {
        &self.stop
    }

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn step(&self) -> &usize {
        &self.step
    }

    pub fn distance(&self) -> u32 {
        return self.stop - self.start;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_range_from_pattern_frame() {
        let range = Range::from_pattern(&String::from("3"));
        assert!(range.is_ok());
        let range = range.unwrap();
        assert_eq!(range.start, 3);
        assert_eq!(range.stop, 3);
        assert_eq!(range.body, Body::Fill);
        assert_eq!(range.step, 1);
    }

    #[test]
    fn test_range_from_pattern_range() {
        let range = Range::from_pattern(&String::from("1-10"));
        assert!(range.is_ok());
        let range = range.unwrap();
        assert_eq!(range.start, 1);
        assert_eq!(range.stop, 10);
        assert_eq!(range.body, Body::Fill);
        assert_eq!(range.step, 1);
    }

    #[test]
    fn test_range_from_pattern_full() {
        let range = Range::from_pattern(&String::from("1-10x2"));
        assert!(range.is_ok());
        let range = range.unwrap();
        assert_eq!(range.start, 1);
        assert_eq!(range.stop, 10);
        assert_eq!(range.body, Body::Fill);
        assert_eq!(range.step, 2);

        let range = Range::from_pattern(&String::from("5-15y3"));
        assert!(range.is_ok());
        let range = range.unwrap();
        assert_eq!(range.start, 5);
        assert_eq!(range.stop, 15);
        assert_eq!(range.body, Body::Inverse);
        assert_eq!(range.step, 3);
    }

    #[test]
    fn test_range_new() {
        assert!(Range::new(1, 5, 1, Body::Fill ).is_ok());
        assert!(Range::new(3, 3, 1, Body::Fill ).is_ok());
        assert!(Range::new(5, 1, 1, Body::Fill ).is_err());
    }

    #[test]
    fn test_range_get_begin() {
        let range = Range::new(1, 10, 1, Body::Fill ).unwrap();
        assert_eq!(*range.start(), 1);
    }

    #[test]
    fn test_range_get_end() {
        let range = Range::new(1, 10, 1, Body::Fill ).unwrap();
        assert_eq!(*range.stop(), 10);
    }

    #[test]
    fn test_range_get_skip() {
        let pattern = Range::new(1, 10, 2, Body::Fill).unwrap();
        assert_eq!(pattern.step(), &2);
    }

    #[test]
    fn test_range_get_body() {
        let pattern = Range::new(1, 10, 2, Body::Fill).unwrap();
        assert_eq!(*pattern.body(), Body::Fill);
        let pattern = Range::new(1, 10, 2, Body::Inverse).unwrap();
        assert_eq!(*pattern.body(), Body::Inverse);
    }

    #[test]
    fn test_range_distance() {
        let pattern = Range::new(1, 10, 1, Body::Fill).unwrap();
        assert_eq!(pattern.distance(), 9);
    }
}