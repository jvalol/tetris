use crate::piece::Piece;
use glam::Vec4;

pub const WIDTH: i32 = 10;
pub const VISIBLE_HEIGHT: i32 = 20;
/// Rows above the visible board where pieces spawn.
pub const HIDDEN_ROWS: i32 = 2;
pub const HEIGHT: i32 = VISIBLE_HEIGHT + HIDDEN_ROWS;

/// The stack of locked cells. Cell (0, 0) is the top-left, x grows right and y
/// grows down, so the first visible row is y == HIDDEN_ROWS.
pub struct Board {
  cells: Vec<Option<Vec4>>,
}

impl Board {
  pub fn new() -> Board {
    Board {
      cells: vec![None; (WIDTH * HEIGHT) as usize],
    }
  }

  pub fn clear(&mut self) {
    for cell in self.cells.iter_mut() {
      *cell = None;
    }
  }

  pub fn cell(&self, x: i32, y: i32) -> Option<Vec4> {
    if !Board::on_board(x, y) {
      return None;
    }
    self.cells[(y * WIDTH + x) as usize]
  }

  pub fn fill(&mut self, x: i32, y: i32, color: Vec4) {
    if Board::on_board(x, y) {
      self.cells[(y * WIDTH + x) as usize] = Some(color);
    }
  }

  /// Anything off the board counts as occupied, which is what stops a piece
  /// leaving through a wall or the floor.
  pub fn is_occupied(&self, x: i32, y: i32) -> bool {
    !Board::on_board(x, y) || self.cell(x, y).is_some()
  }

  /// True when every cell the piece covers is free.
  pub fn fits(&self, piece: &Piece) -> bool {
    piece
      .board_cells()
      .iter()
      .all(|(x, y)| !self.is_occupied(*x, *y))
  }

  pub fn lock(&mut self, piece: &Piece) {
    let color = piece.color();
    for (x, y) in piece.board_cells().iter() {
      self.fill(*x, *y, color);
    }
  }

  /// Removes full rows, drops everything above them, and says how many went.
  pub fn clear_full_rows(&mut self) -> u32 {
    let mut cleared = 0;

    for y in 0..HEIGHT {
      if (0..WIDTH).all(|x| self.cell(x, y).is_some()) {
        cleared += 1;
        for row in (1..=y).rev() {
          for x in 0..WIDTH {
            self.cells[(row * WIDTH + x) as usize] = self.cells[((row - 1) * WIDTH + x) as usize];
          }
        }
        for x in 0..WIDTH {
          self.cells[x as usize] = None;
        }
      }
    }

    cleared
  }

  fn on_board(x: i32, y: i32) -> bool {
    (0..WIDTH).contains(&x) && (0..HEIGHT).contains(&y)
  }
}

impl Default for Board {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::piece::{Piece, Shape};

  fn red() -> Vec4 {
    Vec4::new(1.0, 0.0, 0.0, 1.0)
  }

  fn fill_row(board: &mut Board, y: i32) {
    for x in 0..WIDTH {
      board.fill(x, y, red());
    }
  }

  #[test]
  fn board_has_its_dimensions() {
    assert_eq!(WIDTH, 10);
    assert_eq!(VISIBLE_HEIGHT, 20);
    assert_eq!(HEIGHT, 22);
  }

  #[test]
  fn a_new_board_is_empty() {
    let board = Board::new();

    for y in 0..HEIGHT {
      for x in 0..WIDTH {
        assert!(board.cell(x, y).is_none());
      }
    }
  }

  #[test]
  fn cells_hold_a_color() {
    let mut board = Board::new();
    board.fill(3, 5, red());

    assert_eq!(board.cell(3, 5), Some(red()));
    assert!(board.cell(3, 6).is_none());
  }

  #[test]
  fn outside_the_board_is_occupied() {
    let board = Board::new();

    assert!(board.is_occupied(-1, 5));
    assert!(board.is_occupied(WIDTH, 5));
    assert!(board.is_occupied(5, HEIGHT));
    assert!(board.is_occupied(5, -1));
    assert!(!board.is_occupied(5, 5));
  }

  #[test]
  fn a_piece_fits_where_the_board_is_free() {
    let mut board = Board::new();
    let piece = Piece::at(Shape::T, (4, 10));
    assert!(board.fits(&piece));

    board.lock(&piece);
    assert!(!board.fits(&piece));
  }

  #[test]
  fn locking_fills_the_pieces_cells() {
    let mut board = Board::new();
    let piece = Piece::at(Shape::T, (4, 10));
    board.lock(&piece);

    for (x, y) in piece.board_cells().iter() {
      assert_eq!(board.cell(*x, *y), Some(Shape::T.color()));
    }
  }

  #[test]
  fn a_full_row_clears() {
    let mut board = Board::new();
    fill_row(&mut board, HEIGHT - 1);

    assert_eq!(board.clear_full_rows(), 1);
    assert!(board.cell(0, HEIGHT - 1).is_none());
  }

  #[test]
  fn rows_above_a_clear_drop_down() {
    let mut board = Board::new();
    fill_row(&mut board, HEIGHT - 1);
    // one cell sitting on top of the full row
    board.fill(2, HEIGHT - 2, Shape::T.color());

    assert_eq!(board.clear_full_rows(), 1);
    assert_eq!(board.cell(2, HEIGHT - 1), Some(Shape::T.color()));
    assert!(board.cell(2, HEIGHT - 2).is_none());
  }

  #[test]
  fn four_rows_clear_at_once() {
    let mut board = Board::new();
    for y in HEIGHT - 4..HEIGHT {
      fill_row(&mut board, y);
    }

    assert_eq!(board.clear_full_rows(), 4);
    for y in 0..HEIGHT {
      for x in 0..WIDTH {
        assert!(board.cell(x, y).is_none());
      }
    }
  }

  #[test]
  fn an_incomplete_row_stays() {
    let mut board = Board::new();
    fill_row(&mut board, HEIGHT - 1);
    // punch a hole in it
    board.cells[((HEIGHT - 1) * WIDTH + 4) as usize] = None;

    assert_eq!(board.clear_full_rows(), 0);
    assert!(board.cell(0, HEIGHT - 1).is_some());
  }
}
