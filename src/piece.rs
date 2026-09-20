use cgmath::Vector4;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Shape {
  I,
  O,
  T,
  S,
  Z,
  J,
  L,
}

pub const SHAPES: [Shape; 7] = [
  Shape::I,
  Shape::O,
  Shape::T,
  Shape::S,
  Shape::Z,
  Shape::J,
  Shape::L,
];

impl Shape {
  pub fn color(self) -> Vector4<f32> {
    let (r, g, b) = match self {
      Shape::I => (0.0, 0.85, 0.9),
      Shape::O => (0.95, 0.85, 0.1),
      Shape::T => (0.7, 0.3, 0.85),
      Shape::S => (0.3, 0.8, 0.3),
      Shape::Z => (0.9, 0.25, 0.25),
      Shape::J => (0.25, 0.4, 0.9),
      Shape::L => (0.95, 0.55, 0.15),
    };

    Vector4::new(r, g, b, 1.0)
  }

  /// The piece's four cells around its own origin, in its starting rotation.
  /// x grows right and y grows down, so a cell at y -1 sits above the origin.
  pub fn cells(self) -> [(i32, i32); 4] {
    match self {
      Shape::I => [(-1, 0), (0, 0), (1, 0), (2, 0)],
      Shape::O => [(0, -1), (1, -1), (0, 0), (1, 0)],
      Shape::T => [(0, -1), (-1, 0), (0, 0), (1, 0)],
      Shape::S => [(0, -1), (1, -1), (-1, 0), (0, 0)],
      Shape::Z => [(-1, -1), (0, -1), (0, 0), (1, 0)],
      Shape::J => [(-1, -1), (-1, 0), (0, 0), (1, 0)],
      Shape::L => [(1, -1), (-1, 0), (0, 0), (1, 0)],
    }
  }
}

#[derive(Debug, Copy, Clone)]
pub struct Piece {
  pub shape: Shape,
  /// Cells around the piece's origin, rotated as the piece has been.
  cells: [(i32, i32); 4],
  /// Where the origin sits on the board.
  pub position: (i32, i32),
}

impl Piece {
  pub fn new(shape: Shape) -> Piece {
    Piece {
      shape,
      cells: shape.cells(),
      position: (0, 0),
    }
  }

  pub fn at(shape: Shape, position: (i32, i32)) -> Piece {
    let mut piece = Piece::new(shape);
    piece.position = position;
    piece
  }

  pub fn color(&self) -> Vector4<f32> {
    self.shape.color()
  }

  /// The cells the piece covers on the board.
  pub fn board_cells(&self) -> [(i32, i32); 4] {
    let mut cells = self.cells;
    for cell in cells.iter_mut() {
      *cell = (cell.0 + self.position.0, cell.1 + self.position.1);
    }
    cells
  }

  pub fn moved(&self, dx: i32, dy: i32) -> Piece {
    let mut piece = *self;
    piece.position = (self.position.0 + dx, self.position.1 + dy);
    piece
  }

  /// A quarter turn clockwise about the origin. O is square, so it never changes.
  pub fn rotated_cw(&self) -> Piece {
    self.rotated(|(x, y)| (-y, x))
  }

  pub fn rotated_ccw(&self) -> Piece {
    self.rotated(|(x, y)| (y, -x))
  }

  fn rotated(&self, turn: fn((i32, i32)) -> (i32, i32)) -> Piece {
    if self.shape == Shape::O {
      return *self;
    }

    let mut piece = *self;
    for cell in piece.cells.iter_mut() {
      *cell = turn(*cell);
    }
    piece
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn sorted(mut cells: [(i32, i32); 4]) -> [(i32, i32); 4] {
    cells.sort_unstable();
    cells
  }

  #[test]
  fn every_piece_has_four_cells() {
    for shape in SHAPES {
      let cells = Piece::new(shape).board_cells();
      let mut unique = cells.to_vec();
      unique.sort_unstable();
      unique.dedup();

      assert_eq!(unique.len(), 4, "{:?} has overlapping cells", shape);
    }
  }

  #[test]
  fn every_piece_has_its_own_color() {
    for (i, shape) in SHAPES.iter().enumerate() {
      for other in SHAPES.iter().skip(i + 1) {
        assert_ne!(shape.color(), other.color(), "{:?} and {:?}", shape, other);
      }
    }
  }

  #[test]
  fn four_rotations_come_back_around() {
    for shape in SHAPES {
      let piece = Piece::new(shape);
      let turned = piece
        .rotated_cw()
        .rotated_cw()
        .rotated_cw()
        .rotated_cw();

      assert_eq!(
        sorted(turned.board_cells()),
        sorted(piece.board_cells()),
        "{:?}",
        shape
      );
    }
  }

  #[test]
  fn o_does_not_change_when_rotated() {
    let piece = Piece::new(Shape::O);

    assert_eq!(
      sorted(piece.rotated_cw().board_cells()),
      sorted(piece.board_cells())
    );
  }

  #[test]
  fn rotations_undo_each_other() {
    for shape in SHAPES {
      let piece = Piece::new(shape);

      assert_eq!(
        sorted(piece.rotated_cw().rotated_ccw().board_cells()),
        sorted(piece.board_cells()),
        "{:?}",
        shape
      );
    }
  }

  #[test]
  fn moving_shifts_every_cell() {
    let piece = Piece::at(Shape::T, (4, 1));
    let moved = piece.moved(1, 2);

    for (before, after) in piece.board_cells().iter().zip(moved.board_cells().iter()) {
      assert_eq!(*after, (before.0 + 1, before.1 + 2));
    }
  }
}
