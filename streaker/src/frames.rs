use std::collections::BTreeSet;
use crate::traits::Token;

type FrameNumbers = BTreeSet<u32>;

pub struct Frames {
  frames: FrameNumbers,
}

#[derive(Debug, PartialEq)]
pub enum Body {

  /// Frame sequence can be defined as an expression
  Contiguous,

  /// Frame sequence is unique
  Indirect,
}

impl Token for Frames {
  fn token(&self) -> String {
    let frame_token = match self.body() {
      Body::Contiguous => {
        if self.count() == 0 {
          String::new()
        } else if self.count() == 1 {
          self.first().unwrap().to_string()
        } else {
          format!("{}-{}",
                  self.first().unwrap().to_string(),
                  self.last().unwrap().to_string())
        }
      }
      Body::Indirect => {
        let s: Vec<String> = self.frames.iter().map(|f| f.to_string()).collect();
        s.join(",")
      }
    };
    frame_token
  }
}

impl Frames {

  pub fn new(frames: &[u32]) -> Self {
    Frames {
      frames: frames
        .iter()
        .map(|t| t.clone())
        .collect()
    }
  }

  pub fn skip(&self) -> Option<u32> {
    let skip = {
      if self.count() == 0 {
        None
      } else if self.count() == 1 {
        Some(1)
      } else {
        let mut iter = self.frames.iter();
        let first = iter.next().unwrap();
        let next = iter.next().unwrap();
        let mut skip = next - first;
        let mut prev = next;
        for val in iter {
          let next_skip = val - prev;
          if next_skip != skip {
            return None;
          }
          skip = next_skip;
          prev = val;
        }
        Some(skip)
      }
    };
    skip
  }

  pub fn first(&self) -> Option<u32> {
    let first = {
      if self.count() > 0 {
        Some(*self.frames.iter().next().unwrap())
      } else {
        None
      }
    };
    first
  }

  pub fn last(&self) -> Option<u32> {
    let last = {
      if self.count() > 0 {
        Some(*self.frames.iter().last().unwrap())
      } else {
        None
      }
    };
    last
  }

  pub fn body(&self) -> Body {
    if self.count() == 0 {
      Body::Contiguous
    } else {
      match self.skip() {
        Some(_) => Body::Contiguous,
        _ => Body::Indirect,
      }
    }
  }

  pub fn frames(&self) -> &FrameNumbers {
    &self.frames
  }

  pub fn count(&self) -> usize {
    self.frames.len()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new() {
    assert_eq!(Frames::new(&[0, 1, 2]).frames.len(), 3);
  }

  #[test]
  fn test_body() {
    assert_eq!(Frames::new(&[]).body(), Body::Contiguous);
    assert_eq!(Frames::new(&[1]).body(), Body::Contiguous);
    assert_eq!(Frames::new(&[0, 1]).body(), Body::Contiguous);
    assert_eq!(Frames::new(&[0, 3]).body(), Body::Contiguous);
    assert_eq!(Frames::new(&[2, 4]).body(), Body::Contiguous);
    assert_eq!(Frames::new(&[0,1,3]).body(), Body::Indirect);
    assert_eq!(Frames::new(&[0,3,7]).body(), Body::Indirect);
  }

  #[test]
  fn test_skip() {
    assert_eq!(Frames::new(&[0]).skip(), Some(1));
    assert_eq!(Frames::new(&[1]).skip(), Some(1));
    assert_eq!(Frames::new(&[0, 1]).skip(), Some(1));
    assert_eq!(Frames::new(&[1, 2]).skip(), Some(1));
    assert_eq!(Frames::new(&[0, 2]).skip(), Some(2));
    assert_eq!(Frames::new(&[1, 4]).skip(), Some(3));
    assert_eq!(Frames::new(&[10, 20, 30]).skip(), Some(10));
    assert_eq!(Frames::new(&[]).skip(), None);
    assert_eq!(Frames::new(&[0,1,3]).skip(), None);
    assert_eq!(Frames::new(&[2,4,7]).skip(), None);
  }

  #[test]
  fn test_first() {
    assert_eq!(Frames::new(&[]).first(), None);
    assert_eq!(Frames::new(&[0]).first(), Some(0));
    assert_eq!(Frames::new(&[0, 1]).first(), Some(0));
  }

  #[test]
  fn test_last() {
    assert_eq!(Frames::new(&[]).last(), None);
    assert_eq!(Frames::new(&[0]).last(), Some(0));
    assert_eq!(Frames::new(&[0, 1]).last(), Some(1));
  }

  #[test]
  fn test_token() {
    assert_eq!(Frames::new(&[0]).token(), r#"0"#);
    assert_eq!(Frames::new(&[0,1]).token(), r#"0-1"#);
    assert_eq!(Frames::new(&[2,4]).count(), 2);
    assert_eq!(Frames::new(&[2,4]).token(), r#"2-4"#);
    assert_eq!(Frames::new(&[0,1,3]).token(), r#"0,1,3"#);
    assert_eq!(Frames::new(&[0,1,4,8,13]).token(), r#"0,1,4,8,13"#);
  }
}