use std::convert::TryFrom;

struct Padding {
  value: u32,
}

impl TryFrom<&str> for Padding {
  type Error = String;

  fn try_from(pattern: &str) -> Result<Self, Self::Error> {
    let value = pattern.chars().into_iter()
      .map(|chr| {
        match chr {
          '#' => 4,
          '@' => 1,
          _ => 0,
        }
      }).sum();
    if value > 0 {
      Ok(Padding::new(value))
    } else {
      Err(format!("Pattern not recognised: {}", pattern))
    }
  }
}

impl Padding {
  pub fn new(value: u32) -> Self {
    Padding { value }
  }
}

#[cfg(test)]
mod tests {

}