use range::{Range, Body};
//use std::collections::IndexSet;
use indexmap::IndexSet;
use std::iter::FromIterator;
use std::cmp::Ordering;
use std::fmt::Display;

/// Minimum amount of frames needed to be matched with a pattern.
const MIN_PATTERN_THRESHOLD : usize = 3;

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


enum PatternRules {
    Contiguous,
    Step,
    Inverse,
}

fn to_pattern(frames: &IndexSet<u32>) -> String {

    let mut head_iter = frames.iter();
    let mut tail_iter = frames.iter();
    while let Some(head_frame) = head_iter.next() {

        let mut step : i32 = -1;

        while let Some(tail_frame) = tail_iter.next() {
            if step == -1 {
                step = (tail_frame - head_frame) as i32;
            }
        }
    }
    return String::new();

}

/// Convert Range object to set of frames.
fn consume(range: &Range) -> IndexSet<u32> {
    let mut frames = IndexSet::new();
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

#[derive(Eq)]
pub struct Streak {
    name: String,
    ext: String,
    padding: u32,
    frames: IndexSet<u32>,
}

impl Streak {
    pub fn new(name: String, ext: String, padding: u32, frames: IndexSet<u32> ) -> Self {
        Streak{ name, ext, padding, frames }
    }

    pub fn from_pattern(pattern: &String) -> Result<Self, String>{
        if let Ok(padding) = parse_padding(pattern) {

            let mut pattern = pattern.clone();
            strip_padding(&mut pattern);

            if let Ok(range) = Range::from_pattern(&pattern) {
                let frames = consume(&range);
                return Ok(Streak::new(String::from(""),
                                      String::from(""),
                                      padding,
                                      frames));
            }
        }
        return Err(format!("Malformed pattern: {}", pattern));
    }

    pub fn from_frames(frames: IndexSet<u32>, padding: u32) -> Self {
        Streak::new(String::from(""),
                    String::from(""),
                    padding,
                    frames)
    }

    pub fn is_match(&self, name: &String, ext: &String, padding: u32) -> bool {
        return self.name() == name &&
            self.ext() == ext &&
            self.padding() <= padding;
    }

    pub fn pattern(&self) -> String {
        let mut _padding = String::new();
        _padding.push_str((0..self.padding() / 4).map(|_| "#").collect::<String>().as_str());
        _padding.push_str((0..self.padding() % 4).map(|_| "@").collect::<String>().as_str());

        // TODO: How do I avoid a full copy here just to sort?
        let mut sorted_frames = self.frames.clone();
        sorted_frames.sort();

        if sorted_frames.is_empty() {
            return format!("{}", _padding);
        } else if sorted_frames.len() == 1 {
            return format!("{}{}", _padding, sorted_frames.get_index(0).unwrap());
        } else {

//            let first = sorted_frames.get_index(0).unwrap();
//            let last = sorted_frames.get_index( sorted_frames.len() - 1).unwrap();
//            let distance = last - first;

            let mut contiguous = true;
            let mut iter = sorted_frames.iter();
            let mut last_frame = iter.next().unwrap();
            let mut this_frame = iter.next().unwrap();
            let mut skip = this_frame - last_frame;
            while let Some(this_frame) = iter.next() {

                let last_skip = skip.clone();
                skip = this_frame - last_frame;
                if last_skip != skip {
                    contiguous = false;
                    break;
                }
                last_frame = this_frame;
            }
            if contiguous && skip > 1 {
                return format!("{}{}-{}", _padding,
                               sorted_frames.get_index(0).unwrap(),
                               sorted_frames.get_index(sorted_frames.len() - 1).unwrap() + 1);
            } else {
                let mut frames : String = String::new();
                for index in 0..sorted_frames.len() {
                    frames.push_str(sorted_frames.get_index(index).unwrap().to_string().as_str());
                    if index != sorted_frames.len() {
                        frames += ",";
                    }
                }
                return format!("{}{}", _padding, frames);
            }
        }
    }

    pub fn set_padding(&mut self, padding: u32) {
        self.padding = padding;
    }

    pub fn padding(&self) -> u32 {
        return self.padding;
    }

    pub fn frames(&self) -> &IndexSet<u32> {
        return &self.frames;
    }

    pub fn frames_mut(&mut self) -> &mut IndexSet<u32> {
        return &mut self.frames;
    }

    pub fn name(&self) -> &String {
        return &self.name;
    }

    pub fn ext(&self) -> &String {
        return &self.ext;
    }
}

impl Ord for Streak {
    fn cmp(&self, other: &Streak) -> Ordering {
        self.name().cmp(&other.name())
    }
}

impl PartialOrd for Streak {
    fn partial_cmp(&self, other: &Streak) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Streak {
    fn eq(&self, other: &Streak) -> bool {
        self.name() == other.name() &&
            self.ext() == other.ext() &&
            self.padding() <= other.padding()
    }
}

impl Display for Streak {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.name(), self.pattern(), self.ext())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_consume_fill_ones() {
        let range = Range::new(1, 10, 1, Body::Fill).unwrap();
        let frames : IndexSet<u32> = indexset!{1,2,3,4,5,6,7,8,9};
        assert_eq!(consume(&range), frames);
    }

    #[test]
    fn test_consume_fill_skip() {
        let range = Range::new(1, 10, 3, Body::Fill).unwrap();
        let frames : IndexSet<u32> = indexset!{1,4,7};
        assert_eq!(consume(&range), frames);
    }

    #[test]
    fn test_consume_inverse_skip() {
        let range = Range::new(1, 10, 3, Body::Inverse).unwrap();
        let frames : IndexSet<u32> = indexset!{2,3,5,6,8,9};
        assert_eq!(consume(&range), frames);
    }

    #[test]
    fn test_streak_from_pattern_one() {
        let streak = Streak::from_pattern(&String::from("#3")).unwrap();
        let frames : IndexSet<u32> = indexset!{3};
        assert_eq!(streak.frames(), &frames);
        assert_eq!(streak.padding(), 4);

        let streak = Streak::from_pattern(&String::from("@3")).unwrap();
        let frames : IndexSet<u32> = indexset!{3};
        assert_eq!(streak.frames(), &frames);
        assert_eq!(streak.padding(), 1);
    }

    #[test]
    fn test_streak_from_pattern_range() {
        let streak = Streak::from_pattern(&String::from("#1-10")).unwrap();
        let frames : IndexSet<u32> = indexset!{1,2,3,4,5,6,7,8,9};
        assert_eq!(streak.frames(), &frames);
        assert_eq!(streak.padding(), 4);

        let streak = Streak::from_pattern(&String::from("@1-10")).unwrap();
        let frames : IndexSet<u32> = indexset!{1,2,3,4,5,6,7,8,9};
        assert_eq!(streak.frames(), &frames);
        assert_eq!(streak.padding(), 1);
    }

    #[test]
    fn test_streak_from_frames() {
        let frames : IndexSet<u32> = indexset!{1,2,3,4,5,6,7,8,9};
        let padding : u32 = 4;
        let streak = Streak::from_frames(IndexSet::from_iter(frames.iter().cloned()), padding);
        assert_eq!(streak.frames(), &frames);
        assert_eq!(streak.padding(), 4);
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

    #[test]
    fn test_to_pattern() {
        let frames : Vec<u32> = (1..10).collect();
        assert_eq!(to_pattern( &IndexSet::from_iter(frames.iter().cloned())), String::new());
    }
}