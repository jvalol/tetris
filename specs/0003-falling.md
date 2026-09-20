# 0003 Falling

**Status:** draft
**Date:** 2026-09-20

## Goal

Pieces fall, land, and clear lines, at a pace that gets harder as you go.

## Behavior

**Gravity.** The piece falls one row per interval. At level 1 that interval is
0.8 seconds, and each level multiplies it by 0.85, so level 10 is about 0.16
seconds. The interval never drops below 0.05 seconds.

**Moving.** Left and right move one column when the target cells are free. Down
is a soft drop: while it's held, the piece falls at 20 times the level's speed.

**Hard drop.** The space bar drops the piece as far as it will go and locks it
immediately.

**Locking.** When a piece cannot fall, it locks where it is on the next gravity
step, filling those board cells with its color. There is no lock delay: a piece
that can't fall locks at the next step, however it got there. A new piece spawns
right after.

**Line clears.** After a lock, full rows are removed and everything above drops
by the number of rows cleared. Clearing four at once is the maximum, because a
piece is four cells tall.

## Acceptance criteria

- A piece falls one row per interval. — `system::tests::gravity_drops_a_row_per_interval`
- The interval shortens with the level. — `util::tests::fall_interval_shortens_with_level`
- The interval has a floor. — `util::tests::fall_interval_has_a_floor`
- Left and right move the piece. — `system::tests::left_and_right_move_the_piece`
- A move into a wall doesn't happen. — `system::tests::a_move_into_a_wall_does_not_happen`
- A move into a filled cell doesn't happen. — `system::tests::a_move_into_a_filled_cell_does_not_happen`
- Soft drop makes the piece fall faster. — `system::tests::soft_drop_falls_faster`
- Hard drop lands the piece on the stack. — `system::tests::hard_drop_lands_on_the_stack`
- A piece that cannot fall locks into the board. — `system::tests::a_piece_that_cannot_fall_locks`
- A full row is cleared. — `board::tests::a_full_row_clears`
- Rows above a cleared row drop down. — `board::tests::rows_above_a_clear_drop_down`
- Four rows can clear at once. — `board::tests::four_rows_clear_at_once`

### Verified by hand

- The falling speed is playable at level 1 and frantic by level 10. — run tetris.

## Out of scope

Lock delay, sliding a piece after it lands, and ghost pieces showing where a
piece will land.
