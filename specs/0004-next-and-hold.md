# 0004 Next and hold

**Status:** draft
**Date:** 2026-09-20

## Goal

See what's coming, and keep a piece for when you need it.

## Behavior

**Next.** The piece after the current one is drawn in the right panel, under the
word NEXT. It comes from the bag in spec 0002 and is the piece that spawns when
the current one locks.

**Hold.** C puts the current piece into the hold slot, drawn in the left panel
under the label HOLD (C), which names the key so the panel explains itself. If
the slot was empty, the next piece spawns. If it held a piece, that piece spawns
and the current one takes its place. A held piece spawns in its starting
rotation, not the rotation it was held in.

Hold can be used once per piece. Using it again before the current piece locks
does nothing, and locking a piece makes hold available again.

## Acceptance criteria

- Holding with an empty slot takes the next piece. — `system::tests::holding_with_an_empty_slot_takes_the_next_piece`
- Holding again swaps the held piece in. — `system::tests::holding_again_swaps_the_pieces`
- A held piece comes back in its starting rotation. — `system::tests::a_held_piece_comes_back_upright`
- Hold only works once per piece. — `system::tests::hold_only_works_once_per_piece`
- Locking a piece makes hold available again. — `system::tests::locking_frees_the_hold`
- The hold panel names its key. — `state::tests::the_hold_label_names_its_key`

### Verified by hand

- The next and hold panels show the right pieces. — run tetris, hold a piece and
  watch the panels.

## Out of scope

Showing more than one piece ahead, and swapping the held piece directly with the
next one.
