use crate::board::{HIDDEN_ROWS, VISIBLE_HEIGHT, WIDTH};
use glam::Vec2;

/// The board takes 10 columns and the panels four each, so the window is
/// divided into this many columns of width.
pub const TOTAL_COLUMNS: f32 = 18.0;

/// Where the board sits in the window, and how big a cell is.
///
/// Board cells are the game's units. This is the only place they become pixels.
#[derive(Debug, Copy, Clone)]
pub struct Layout {
  /// Top-left corner of the visible board, in pixels.
  pub origin: Vec2,
  pub cell: f32,
  /// The window, so the panels can center themselves in what is left of it.
  pub window: Vec2,
}

impl Layout {
  pub fn new(size: Vec2) -> Layout {
    let by_height = size.y / VISIBLE_HEIGHT as f32;
    let by_width = size.x / TOTAL_COLUMNS;
    let cell = by_height.min(by_width).floor().max(1.0);

    let board = Vec2::new(cell * WIDTH as f32, cell * VISIBLE_HEIGHT as f32);
    let origin = Vec2::new((size.x - board.x) * 0.5, (size.y - board.y) * 0.5);

    Layout {
      origin,
      cell,
      window: size,
    }
  }

  pub fn board_size(&self) -> Vec2 {
    Vec2::new(
      self.cell * WIDTH as f32,
      self.cell * VISIBLE_HEIGHT as f32,
    )
  }

  /// The center of a board cell in pixels. Hidden rows land above the board's
  /// origin, which is off the top of the playfield and not drawn.
  pub fn cell_center(&self, x: i32, y: i32) -> Vec2 {
    Vec2::new(
      self.origin.x + (x as f32 + 0.5) * self.cell,
      self.origin.y + ((y - HIDDEN_ROWS) as f32 + 0.5) * self.cell,
    )
  }

  /// True for rows below the hidden spawn rows, the ones that get drawn.
  pub fn is_visible_row(&self, y: i32) -> bool {
    y >= HIDDEN_ROWS
  }

  /// Center of the space left of the board, where hold and the score go. Panel
  /// content is centered on this, so it never runs over the board's edge.
  pub fn left_panel(&self) -> Vec2 {
    Vec2::new(self.origin.x * 0.5, self.origin.y + self.cell)
  }

  /// Center of the space right of the board, where the next piece goes.
  pub fn right_panel(&self) -> Vec2 {
    let board_right = self.origin.x + self.board_size().x;
    Vec2::new(
      (board_right + self.window.x) * 0.5,
      self.origin.y + self.cell,
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn cell_size_fits_the_window() {
    // height bound: 600 / 20 is 30, width bound: 800 / 18 is 44, so 30 wins
    let tall = Layout::new((800.0, 600.0).into());
    assert_eq!(tall.cell, 30.0);

    // width bound: 360 / 18 is 20, height bound: 600 / 20 is 30, so 20 wins
    let narrow = Layout::new((360.0, 600.0).into());
    assert_eq!(narrow.cell, 20.0);
  }

  #[test]
  fn board_is_centered() {
    let layout = Layout::new((800.0, 600.0).into());
    let board = layout.board_size();

    assert_eq!(board.x, 300.0);
    assert_eq!(board.y, 600.0);
    assert_eq!(layout.origin.x, 250.0);
    assert_eq!(layout.origin.y, 0.0);
  }

  #[test]
  fn cell_centers_are_pixels() {
    let layout = Layout::new((800.0, 600.0).into());
    // the first visible row sits at the board's origin
    let first = layout.cell_center(0, HIDDEN_ROWS);

    assert_eq!(first.x, layout.origin.x + 15.0);
    assert_eq!(first.y, layout.origin.y + 15.0);

    let along = layout.cell_center(3, HIDDEN_ROWS + 2);
    assert_eq!(along.x, layout.origin.x + 30.0 * 3.0 + 15.0);
    assert_eq!(along.y, layout.origin.y + 30.0 * 2.0 + 15.0);
  }

  #[test]
  fn panels_sit_beside_the_board() {
    let layout = Layout::new((800.0, 600.0).into());
    let board_right = layout.origin.x + layout.board_size().x;

    assert!(layout.left_panel().x < layout.origin.x);
    assert!(layout.left_panel().x > 0.0);
    assert!(layout.right_panel().x > board_right);
    assert!(layout.right_panel().x < 800.0);
  }

  #[test]
  fn hidden_rows_are_not_visible() {
    let layout = Layout::new((800.0, 600.0).into());

    assert!(!layout.is_visible_row(0));
    assert!(!layout.is_visible_row(HIDDEN_ROWS - 1));
    assert!(layout.is_visible_row(HIDDEN_ROWS));
  }
}
