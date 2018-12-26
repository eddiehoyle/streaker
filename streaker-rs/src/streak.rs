use range::{Range, Body};
use std::collections::BTreeSet;
use std::iter::FromIterator;

/// Strip '@' and '#' characters
fn strip_padding(pattern: &mut String) {
    pattern.retain(|c| { c != '#' && c != '@' });
}

/// Read pattern and extract padding
fn parse_padding(pattern: &String) -> Result<u32, String> {
    let padding = pattern.chars().into_iter()
        .map(|chr| {
            match chr {
                '#' => 4,
                '@' => 1,
                _ => 0,
            }
        }).sum();
    return if padding > 0 {
        Ok(padding)
    } else {
        Err(format!("No padding characters found in pattern: {}", pattern))
    };
}

/// 1 2 3 4 5 6 7 8 9 10
/// 1   3   5   7   9    // 1-10x2
///   2   4   6   8      // 2-10x2
/// 1     4     7        // 1-10x3
///   2 3   5 6   8 9    // 1-10y3

/// Convert Range object to set of frames.
fn consume(range: &Range) -> BTreeSet<u32> {
    let mut frames = BTreeSet::new();
    if range.distance() == 0 {
        frames.insert(*range.start());
    } else {
        match range.body() {
            Body::Fill => {
                for n in (*range.start()..*range.stop()).step_by(*range.step()) {
                    frames.insert(n);
                }
            },
            Body::Inverse => {
                let mut count = 0;
                for n in *range.start()..*range.stop() {
                    if count != 0 {
                        frames.insert(n);
                    }
                    count += 1;
                    if count == *range.step() {
                        count = 0;
                    }
                }
            }
        }
    }
    return frames;
}

pub struct Streak {
    padding: u32,
    range: Range,
    frames: BTreeSet<u32>,
}

impl Streak {
    pub fn from_pattern(pattern: &String) -> Result<Self, String>{
        if let Ok(padding) = parse_padding(pattern) {

            let mut pattern = pattern.clone();
            strip_padding(&mut pattern);

            if let Ok(range) = Range::from_pattern(&pattern) {
                let frames = consume(&range);
                return Ok(Streak{padding, range, frames});
            }
        }
        return Err(format!("Malformed pattern: {}", pattern));
    }

    pub fn padding(&self) -> u32 {
        return self.padding;
    }

    pub fn frames(&self) -> &BTreeSet<u32> {
        return &self.frames;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_consume_fill_ones() {
        let range = Range::new(1, 10, 1, Body::Fill).unwrap();
        let frames = BTreeSet::from_iter(vec![1, 2, 3, 4, 5, 6, 7, 8, 9].iter().cloned());
        assert_eq!(consume(&range), frames);
    }

    #[test]
    fn test_consume_fill_skip() {
        let range = Range::new(1, 10, 3, Body::Fill).unwrap();
        let frames = BTreeSet::from_iter(vec![1, 4, 7].iter().cloned());
        assert_eq!(consume(&range), frames);
    }

    #[test]
    fn test_consume_inverse_skip() {
        let range = Range::new(1, 10, 3, Body::Inverse).unwrap();
        let frames = BTreeSet::from_iter(vec![2,3,5,6,8,9].iter().cloned());
        assert_eq!(consume(&range), frames);
    }

    #[test]
    fn test_streak_from_pattern_one() {
        let streak = Streak::from_pattern(&String::from("#3")).unwrap();
        let frames : Vec<u32> = vec!(3);
        assert_eq!(streak.frames(), &BTreeSet::from_iter(frames.iter().cloned()));
        assert_eq!(streak.padding(), 4);

        let streak = Streak::from_pattern(&String::from("@3")).unwrap();
        let frames : Vec<u32> = vec!(3);
        assert_eq!(streak.frames(), &BTreeSet::from_iter(frames.iter().cloned()));
        assert_eq!(streak.padding(), 1);
    }

    #[test]
    fn test_streak_from_pattern_range() {
        let streak = Streak::from_pattern(&String::from("#1-10")).unwrap();
        let frames : Vec<u32> = (1..10).collect();
        assert_eq!(streak.frames(), &BTreeSet::from_iter(frames.iter().cloned()));
        assert_eq!(streak.padding(), 4);

        let streak = Streak::from_pattern(&String::from("@1-10")).unwrap();
        let frames : Vec<u32> = (1..10).collect();
        assert_eq!(streak.frames(), &BTreeSet::from_iter(frames.iter().cloned()));
        assert_eq!(streak.padding(), 1);
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