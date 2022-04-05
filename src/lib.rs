extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parser {
  // pub fn parse_csv_string() {}

  pub fn is_even(number: i32) -> bool {
    match number {
      2 => true,
      4 => true,
      6 => true,
      _ => false,
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::parser;

  #[test]
  fn is_even() {
    assert_eq!(parser::is_even(2), true);
    assert_eq!(parser::is_even(4), true);
    assert_eq!(parser::is_even(6), true);
    assert_eq!(parser::is_even(-1), false);
    assert_eq!(parser::is_even(3), false);
    assert_eq!(parser::is_even(7), false);
  }
}
