use crate::piece::{Shape, SHAPES};
use rand::seq::SliceRandom;
use std::collections::VecDeque;

/// Deals all seven pieces in a shuffled order, then shuffles them again, so no
/// piece can go missing for long and no more than two of a kind come in a row.
pub struct Bag {
  queue: VecDeque<Shape>,
}

impl Bag {
  pub fn new() -> Bag {
    let mut bag = Bag {
      queue: VecDeque::new(),
    };
    bag.refill();
    bag
  }

  pub fn next(&mut self) -> Shape {
    let shape = self.queue.pop_front().expect("the bag is never empty");
    if self.queue.is_empty() {
      self.refill();
    }
    shape
  }

  /// The piece that `next` will deal, for the preview.
  pub fn peek(&self) -> Shape {
    *self.queue.front().expect("the bag is never empty")
  }

  fn refill(&mut self) {
    let mut shapes = SHAPES;
    shapes.shuffle(&mut rand::thread_rng());
    self.queue.extend(shapes.iter());
  }
}

impl Default for Bag {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_bag_deals_all_seven() {
    let mut bag = Bag::new();
    let mut dealt: Vec<Shape> = (0..7).map(|_| bag.next()).collect();
    dealt.sort_unstable_by_key(|shape| format!("{:?}", shape));
    dealt.dedup();

    assert_eq!(dealt.len(), 7);
  }

  #[test]
  fn bags_keep_coming() {
    let mut bag = Bag::new();
    let dealt: Vec<Shape> = (0..14).map(|_| bag.next()).collect();

    for shape in SHAPES {
      assert_eq!(
        dealt.iter().filter(|dealt| **dealt == shape).count(),
        2,
        "{:?}",
        shape
      );
    }
  }

  #[test]
  fn peek_shows_what_is_next() {
    let mut bag = Bag::new();

    for _ in 0..20 {
      let peeked = bag.peek();
      assert_eq!(bag.next(), peeked);
    }
  }
}
