
use std::fmt;

//use pattern::Pattern;

//pub struct Range {
//    frames: Vec<u32>,
//}
//
//impl Range {
//
////    pub fn from(pattern: &Pattern) -> Self {
////
////    }
//
//    pub fn new(frames: Vec<u32>) -> Self {
//        Range{ frames: frames }
//    }
//    pub fn has(&self, frame: u32)->bool {
//        return self.frames.contains(&frame);
//    }
//    pub fn frames(&self) -> &Vec<u32> {
//        return &self.frames;
//    }
//}
//
//impl fmt::Display for Range {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "{:?}", self.frames)
//    }
//}
//
//mod tests {
//
//    use super::*;
//
//    #[test]
//    fn test_new() {
//        let _ = Range::new(vec![1, 2, 3]);
//    }
//
//    #[test]
//    fn test_has() {
//        assert!(Range::new(vec![1,2,3]).has(1));
//        assert!(!Range::new(vec![1,2,3]).has(4));
//    }
//
//    #[test]
//    fn test_frames() {
//        let result = vec![1,2,3];
//        assert_eq!(Range::new(vec![1,2,3]).frames(), &result);
//    }
//}