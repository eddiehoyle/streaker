//use std::fmt;
//
//use range;
use pattern::{PatternRange, Body};
use std::collections::BTreeSet;
use std::iter::FromIterator;

/// Strip '@' and '#' characters
fn strip_padding(pattern: &mut String) {
    pattern.retain(|c| { c != '#' && c != '@' });
}

fn consume(range: &PatternRange) -> BTreeSet<u32> {
    let mut frames = BTreeSet::new();
    for n in *range.start()..*range.stop() {
        match range.body() {
            Body::Fill => {
                if n % range.step() == 0 {
                    frames.insert(n);
                }
            },
            Body::Inverse => {
                if n % range.step() != 0 {
                    frames.insert(n);
                }
            }
        }
    }
    return frames;
}

pub struct Streak {
    padding: u32,
    frames: BTreeSet<u32>,
}

impl Streak {
    fn from_pattern(pattern: &String) {
        let mut range = pattern.clone();
        strip_padding(&mut range);


    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_consume() {
        let range = PatternRange::new(1, 6, 1, Body::Fill).unwrap();
        let frames = BTreeSet::from_iter(vec![1,2,3,4,5].iter().cloned());
        assert_eq!(consume(&range), frames);

        let range = PatternRange::new(2, 9, 2, Body::Fill).unwrap();
        let frames = BTreeSet::from_iter(vec![2,4,6,8].iter().cloned());
        assert_eq!(consume(&range), frames);

        let range = PatternRange::new(1, 10, 3, Body::Inverse).unwrap();
        let frames = BTreeSet::from_iter(vec![1,2,4,5,7,8].iter().cloned());
        assert_eq!(consume(&range), frames);
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
