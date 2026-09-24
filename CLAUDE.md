# tetris

Tetris on `blitzkit`, which owns the window, rendering, input, and sound. The
third game on that engine, after pong and snake. The dependency is the published
crate, overridden by the engine checkout at `../blitzkit` when built inside this
project folder.

## Build and test

Requires Rust 1.87 or newer, the engine's MSRV.

```
cargo build
cargo test
cargo run
cargo clippy
cargo fmt
```

## How work happens here

Behavior changes are spec driven:

1. **Write the spec first.** Copy `specs/TEMPLATE.md` to `specs/NNNN-short-name.md`
   and fill it in. Say what the game does, not how the code does it.
2. **Make the acceptance criteria testable.** Each one names the test that proves
   it, or goes under "Verified by hand" when it needs a window.
3. **Write the tests, then the code.** `cargo test` passes before a commit.
4. **Update the spec when behavior changes.** A spec that disagrees with the game
   is a bug in the spec.

## Layout

- `src/main.rs` — hands a `TetrisGame` to `blitzkit::start`.
- `src/tetris_game.rs` — the `Game` impl and the state machine.
- `src/board.rs` — the grid of cells, collision, and line clears.
- `src/piece.rs` — the seven tetrominoes and rotation.
- `src/bag.rs` — the seven piece bag.
- `src/layout.rs` — window pixels to board cells and back.
- `src/state.rs` — everything the game knows.
- `src/system.rs` — one system per game state, each stepping the state.
- `src/input.rs` — engine key events to held flags.
- `src/util.rs` — scoring, levels, and fall speed.

## Conventions

- **Cells for rules, pixels for drawing.** Everything the game decides works in
  board cells. Only `Layout` converts to pixels, and only for drawing.
- **Speeds are per second**, multiplied by `State::delta_time`.
- **Systems are pure game logic.** They take input, state, and events, and touch
  no GPU, window, or audio. That is what makes them testable, so keep it that way.
- Tests live next to the code in `#[cfg(test)] mod tests`.
