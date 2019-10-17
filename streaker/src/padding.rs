use std::convert::TryFrom;

pub struct Padding {
  value: u32,
}

impl TryFrom<&str> for Padding {
  type Error = String;

  fn try_from(pattern: &str) -> Result<Self, Self::Error> {
    if pattern.chars().all(|c| r#"@#"#.contains(c)) {
      let value = pattern.chars().into_iter()
        .map(|chr| {
          match chr {
            '#' => 4,
            '@' => 1,
            _ => 0,
          }
        }).sum();
      Ok(Padding::new(value))
    } else {
      Err(format!("Invalid pattern: {}", pattern))
    }
  }
}

impl Padding {

  /// Constructor
  pub fn new(value: u32) -> Self {
    Padding { value }
  }

  /// Format token from padding value
  pub fn token(&self) -> String {
    let fours = (self.value / 4) as usize;
    let ones = ::std::cmp::max(if fours > 0 {0} else {1},(self.value % 4) as usize);
    format!("{}{}",
            "@".repeat(ones),
            "#".repeat(fours),
    )
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_new() {
    assert_eq!(Padding::new(0).value, 0);
    assert_eq!(Padding::new(1).value, 1);
  }

  #[test]
  fn test_try_from() {

    // Good
    let padding = Padding::try_from("");
    assert!(padding.is_ok());
    assert_eq!(padding.unwrap().value, 0);
    let padding = Padding::try_from("@");
    assert!(padding.is_ok());
    assert_eq!(padding.unwrap().value, 1);
    let padding = Padding::try_from("#");
    assert!(padding.is_ok());
    assert_eq!(padding.unwrap().value, 4);
    let padding = Padding::try_from("@#");
    assert!(padding.is_ok());
    assert_eq!(padding.unwrap().value, 5);

    // Bad
    assert!(Padding::try_from("asd").is_err());
    assert!(Padding::try_from("1234").is_err());
    assert!(Padding::try_from("#d").is_err());
    assert!(Padding::try_from("@d").is_err());
    assert!(Padding::try_from("@#dasd_1").is_err());
  }

  #[test]
  fn test_token() {
    assert_eq!(Padding::new(1).token(), r#"@"#);
    assert_eq!(Padding::new(2).token(), r#"@@"#);
    assert_eq!(Padding::new(3).token(), r#"@@@"#);
    assert_eq!(Padding::new(4).token(), r#"#"#);
    assert_eq!(Padding::new(5).token(), r#"@#"#);
    assert_eq!(Padding::new(8).token(), r#"##"#);
    assert_eq!(Padding::new(9).token(), r#"@##"#);
    assert_eq!(Padding::new(0).token(), r#"@"#);
  }
}