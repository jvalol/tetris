# 0006 Game flow

**Status:** draft
**Date:** 2026-09-20

## Goal

Getting in and out of a game works the way pong and snake do, so all three feel
like the same family.

## Behavior

**Menu.** TETRIS with Play and Quit. Up and Down move between them, Enter
chooses, and Escape quits.

**Playing.** The game, per specs 0002 through 0005. Escape returns to the menu
and throws the board away.

**Paused.** Losing window focus during a game pauses it and shows Paused with
Resume. Enter resumes. Nothing falls while paused. Losing focus anywhere else
changes nothing.

**Game over.** The final score is shown for five seconds, then the game returns
to the menu. Escape quits from here.

Starting a game clears the board, the score, the level, the hold slot, and the
bag.

Escape is acted on once per press, so holding it doesn't carry from a game into
the menu and quit.

## Acceptance criteria

- Escape during a game returns to the menu. — `system::tests::escape_returns_to_the_menu`
- Escape from the menu quits. — `system::tests::escape_quits_from_the_menu`
- Starting a game clears everything. — `system::tests::starting_a_game_clears_the_board`
- Losing focus while playing pauses. — `tetris_game::tests::losing_focus_while_playing_pauses`
- Losing focus on the menu changes nothing. — `tetris_game::tests::losing_focus_on_the_menu_does_nothing`
- Nothing falls while paused. — `tetris_game::tests::nothing_falls_while_paused`
- Escape is ignored on key repeat. — `input::tests::escape_ignores_key_repeat`

### Verified by hand

- The five second game over wait feels right. — run tetris and top out.

## Out of scope

A pause key, and restarting without going through the menu. Sound is spec 0007.
