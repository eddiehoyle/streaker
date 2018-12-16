//use std::fmt;
//
//use range;
//use pattern;
//
//pub struct Streak {
//    range: range::Range,
//    padding: u32,
//    pattern: pattern::Pattern,
//}
//
//
//impl Streak {
//    fn new(pattern: pattern::Pattern, range: range::Range, padding: u32) -> Streak {
//        Streak { pattern, range, padding }
//    }
//
//    fn to_frame(&self, frame: u32) -> String {
//        String::new()
//    }
//}
//
//impl fmt::Display for Streak {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "(Pattern: {}, Range: {}, Padding: {})",
//               self.pattern,
//               self.range,
//               self.padding,
//        )
//    }
//}
