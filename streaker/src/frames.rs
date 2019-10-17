use std::collections::BTreeSet;

#[derive(Debug, PartialEq)]
enum Body {
  Contiguous,
  Indirect,
}

pub struct Frames {
  frames: BTreeSet<u32>,
}

impl Frames {
  pub fn new(frames: &[u32]) -> Self {
    Frames { frames: frames
      .iter()
      .map(|t| t.clone())
      .collect()
    }
  }

  pub fn body(&self) -> Body {
    let result = {
      if self.frames.len() > 1 {
        let mut iter = self.frames.iter();
        let first = iter.next().unwrap();
        let last = iter.last().unwrap();
        if (last - first + 1) as usize == self.frames.len() {
          Body::Contiguous
        } else {
          Body::Indirect
        }
      } else {
        Body::Contiguous
      }
    };
    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new() {
    assert_eq!(Frames::new(&[0,1,2]).frames.len(), 3);
  }

  #[test]
  fn test_body() {
    assert_eq!(Frames::new(&[]).body(), Body::Contiguous);
    assert_eq!(Frames::new(&[1]).body(), Body::Contiguous);
    assert_eq!(Frames::new(&[0,1]).body(), Body::Contiguous);
    assert_eq!(Frames::new(&[0,3]).body(), Body::Indirect);
  }
}