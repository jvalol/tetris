# 0005 Scoring

**Status:** draft
**Date:** 2026-09-20

## Goal

A score that rewards clearing more lines at once, and a level that makes that
harder.

## Behavior

**Line clears** score by how many go at once, multiplied by the level: one row
100, two 300, three 500, four 800.

**Drops** score too. A soft drop scores 1 per row fallen, a hard drop 2 per row.
Neither is multiplied by the level.

**Levels.** The game starts at level 1 and goes up one for every 10 rows cleared,
so 10 rows is level 2. The level sets the falling speed, per spec 0003.

**Game over** happens when a new piece spawns into filled cells. The score stays
on screen until the game returns to the menu.

The score, level, and row count are drawn in the left panel while playing.

## Acceptance criteria

- One row scores 100 times the level. — `util::tests::a_single_row_scores_a_hundred`
- Four rows score 800 times the level. — `util::tests::four_rows_score_eight_hundred`
- The level multiplies a clear's score. — `util::tests::the_level_multiplies_the_score`
- Ten rows raise the level. — `util::tests::ten_rows_raise_the_level`
- A soft drop scores one per row. — `system::tests::soft_drop_scores_a_point_a_row`
- A hard drop scores two per row. — `system::tests::hard_drop_scores_two_a_row`
- Spawning into filled cells ends the game. — `system::tests::spawning_into_the_stack_ends_the_game`

### Verified by hand

- The score and level read correctly while playing. — run tetris and clear lines.

## Out of scope

Combos, back to back bonuses, T-spin scoring, and saving high scores.
