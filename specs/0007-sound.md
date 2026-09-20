# 0007 Sound

**Status:** implemented
**Date:** 2026-09-20

## Goal

The game makes a noise when something happens, the way pong and snake do.

## Behavior

There is one sample, the same blip pong and snake use. It's played at different
speeds so events are told apart by pitch rather than by having more samples:

- A menu choice or moving between menu entries: normal speed.
- A piece locking: slower, so it reads as a thud.
- Rows clearing: faster, the brightest sound in the game.
- Topping out: slowest.

Systems raise events on the state as they run, and the game drains them each
frame and queues a sound for each. A frame that locks a piece and clears rows
makes both sounds.

Sound goes through the engine's `SoundSystem`, so a machine with no audio device
plays nothing and the game runs on, per the engine's spec 0004.

## Acceptance criteria

- Locking a piece raises its event. — `system::tests::locking_a_piece_raises_an_event`
- Clearing rows raises its event. — `system::tests::clearing_rows_raises_an_event`
- Topping out raises its event. — `system::tests::topping_out_raises_an_event`

### Verified by hand

- Each event is audible and tells itself apart. — run tetris, lock a piece,
  clear a line, and top out.

## Out of scope

Music, separate samples per event, and a volume control.
