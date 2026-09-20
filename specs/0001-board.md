# 0001 Board

**Status:** draft
**Date:** 2026-09-20

## Goal

A playfield that fills the window at any size, with room beside it for the hold
slot, the next piece, and the score.

## Behavior

The board is 10 columns by 20 visible rows. Two rows above the top are where
pieces spawn: they are part of the board but are not drawn.

Cells are square. The cell size is whichever fits: the window height divided by
20, or the window width divided by 18, whichever is smaller, rounded down. The
extra 8 columns of width are the side panels, four columns on each side. The
board is centered in the window.

A cell holds either nothing or one color. Filled cells are drawn as quads in that
color, and empty ones are left as background. A one cell border is drawn around
the board so the playfield's edges are visible.

Cell (0, 0) is the top-left of the board, x grows right and y grows down. The
board converts a cell to pixels for drawing, and nothing outside it does that
arithmetic.

Resizing recomputes the cell size and the origin. What is on the board doesn't
change, since the board is stored in cells.

## Acceptance criteria

- The board is 10 wide and 22 tall, 20 of them visible. — `board::tests::board_has_its_dimensions`
- A new board is empty. — `board::tests::a_new_board_is_empty`
- Cells can be filled and read back. — `board::tests::cells_hold_a_color`
- Cells off the board read as occupied, so pieces can't leave it. — `board::tests::outside_the_board_is_occupied`
- The cell size is the smaller of the width and height fits. — `layout::tests::cell_size_fits_the_window`
- The board is centered in the window. — `layout::tests::board_is_centered`
- A cell converts to its center in pixels. — `layout::tests::cell_centers_are_pixels`

### Verified by hand

- The board fills the window with panels either side, at any window size. — run
  tetris and resize it.

## Out of scope

Board sizes other than 10 by 20, and a visible grid inside the playfield.
