# 0002 Pieces

**Status:** draft
**Date:** 2026-09-20

## Goal

The seven tetrominoes, each recognizable by shape and color, rotating in a way
that feels fair near walls.

## Behavior

The seven pieces are I, O, T, S, Z, J and L, each four cells and its own color:
I cyan, O yellow, T purple, S green, Z red, J blue, L orange.

A piece is its four cells relative to its own origin, plus a board position. It
has four rotations, clockwise and counterclockwise, except O which doesn't change
when rotated. Rotation is a quarter turn of each cell around the piece's origin,
not the Super Rotation System.

**Wall kicks.** When a rotation would overlap a wall, the floor, or a filled cell,
the piece is tried one cell left, one right, two left, and two right, in that
order. The first that fits is taken. If none fit, the rotation doesn't happen.

**Spawning.** A piece spawns centered on the board's top rows. Spawning into
filled cells ends the game, per spec 0005.

**Pieces come from a bag.** All seven are shuffled, then dealt one at a time.
When the bag empties, a fresh shuffled bag follows. So a piece can't be missing
for more than twelve in a row, and there are never more than two of a kind in a row.

## Acceptance criteria

- Each piece has four cells. — `piece::tests::every_piece_has_four_cells`
- Each piece has its own color. — `piece::tests::every_piece_has_its_own_color`
- Rotating four times returns a piece to where it started. — `piece::tests::four_rotations_come_back_around`
- O doesn't change when rotated. — `piece::tests::o_does_not_change_when_rotated`
- Clockwise and counterclockwise undo each other. — `piece::tests::rotations_undo_each_other`
- A rotation into a wall kicks the piece away from it. — `system::tests::rotation_kicks_off_the_wall`
- A rotation with nowhere to go doesn't happen. — `system::tests::blocked_rotation_does_not_happen`
- A bag deals all seven pieces before repeating. — `bag::tests::a_bag_deals_all_seven`
- A new bag follows the old one. — `bag::tests::bags_keep_coming`

### Verified by hand

- The pieces are easy to tell apart on screen. — run tetris and play.

## Out of scope

The Super Rotation System's kick tables, T-spin detection, and 180 degree spins.
