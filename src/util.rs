#![macro_use]

/// Seconds between gravity steps at level 1.
pub const BASE_FALL_INTERVAL: f32 = 0.8;
/// However high the level goes, a piece never falls faster than this.
pub const MIN_FALL_INTERVAL: f32 = 0.05;
/// How much faster a piece falls while down is held.
pub const SOFT_DROP_MULTIPLIER: f32 = 20.0;
/// Rows cleared per level.
pub const ROWS_PER_LEVEL: u32 = 10;

/// Seconds between gravity steps at `level`, which starts at 1.
pub fn fall_interval(level: u32) -> f32 {
  let steps = level.saturating_sub(1) as i32;
  (BASE_FALL_INTERVAL * 0.85f32.powi(steps)).max(MIN_FALL_INTERVAL)
}

/// What clearing `rows` at once is worth at `level`.
pub fn score_for_rows(rows: u32, level: u32) -> u32 {
  let base = match rows {
    1 => 100,
    2 => 300,
    3 => 500,
    4 => 800,
    _ => 0,
  };

  base * level
}

/// The level after clearing `rows` in total. Level 1 until the tenth row.
pub fn level_for_rows(rows: u32) -> u32 {
  rows / ROWS_PER_LEVEL + 1
}

#[macro_export]
macro_rules! any {
    ($x:expr, $($y:expr),+ $(,)?) => {
        {
            false $(|| $x == $y)+
        }
    };
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn fall_interval_shortens_with_level() {
    assert_eq!(fall_interval(1), BASE_FALL_INTERVAL);
    assert!(fall_interval(2) < fall_interval(1));
    assert!(fall_interval(10) < fall_interval(5));
  }

  #[test]
  fn fall_interval_has_a_floor() {
    assert_eq!(fall_interval(100), MIN_FALL_INTERVAL);
    assert!(fall_interval(30) >= MIN_FALL_INTERVAL);
  }

  #[test]
  fn a_single_row_scores_a_hundred() {
    assert_eq!(score_for_rows(1, 1), 100);
  }

  #[test]
  fn four_rows_score_eight_hundred() {
    assert_eq!(score_for_rows(4, 1), 800);
    assert_eq!(score_for_rows(2, 1), 300);
    assert_eq!(score_for_rows(3, 1), 500);
  }

  #[test]
  fn the_level_multiplies_the_score() {
    assert_eq!(score_for_rows(1, 5), 500);
    assert_eq!(score_for_rows(4, 3), 2400);
  }

  #[test]
  fn ten_rows_raise_the_level() {
    assert_eq!(level_for_rows(0), 1);
    assert_eq!(level_for_rows(9), 1);
    assert_eq!(level_for_rows(10), 2);
    assert_eq!(level_for_rows(25), 3);
  }
}
