use blitzkit::start;

mod bag;
mod board;
mod input;
mod layout;
mod piece;
mod state;
mod system;
mod tetris_game;
mod util;

use tetris_game::TetrisGame;

fn main() {
  start("Tetris", Box::new(TetrisGame::new()));
}
