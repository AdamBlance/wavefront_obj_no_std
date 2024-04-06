use core::cmp::Ordering;

/// Extensions to orderings.
pub(crate) trait OrderingExt {
  /// Lexicographically chains comparisions.
  fn lexico<F: Fn() -> Ordering>(self, f: F) -> Self;
}

impl OrderingExt for Ordering {
  fn lexico<F: Fn() -> Ordering>(self, f: F) -> Ordering {
    match self {
      Ordering::Less | Ordering::Greater => self,
      Ordering::Equal => f(),
    }
  }
}

pub(crate) fn abs(x: f64) -> f64 {
  if x < 0.0 { -1.0*x } else { x }
}
