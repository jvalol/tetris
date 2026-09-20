use crate::board::HIDDEN_ROWS;
use crate::board::WIDTH;
use crate::input::Input;
use crate::piece::{Piece, Shape};
use crate::state::*;
use crate::tetris_game::Event;
use crate::util;

/// Seconds a left or right key is held before the piece starts moving again.
pub const MOVE_DELAY: f32 = 0.17;
/// Seconds between moves once a held key has started repeating.
pub const MOVE_REPEAT: f32 = 0.05;

/// Sideways offsets tried when a rotation does not fit where it is.
const KICKS: [i32; 4] = [-1, 1, -2, 2];

pub trait System {
  #[allow(unused_variables)]
  fn start(&mut self, state: &mut State) {}
  fn update_state(&self, input: &mut Input, state: &mut State);
}

pub struct VisibilitySystem;

impl System for VisibilitySystem {
  fn update_state(&self, _input: &mut Input, state: &mut State) {
    let menu = state.game_state == GameState::MainMenu;
    let paused = state.game_state == GameState::Paused;
    let in_game = state.game_state == GameState::Playing
      || paused
      || state.game_state == GameState::GameOver;

    state.title_text.visible = menu || paused;
    state.play_button.visible = menu || paused;
    state.quit_button.visible = menu;

    state.score_text.visible = in_game;
    state.level_text.visible = in_game;
    state.rows_text.visible = in_game;
    state.hold_label.visible = in_game;
    state.next_label.visible = in_game;

    state.game_over_text.visible = state.game_state == GameState::GameOver;
  }
}

pub struct MenuSystem;

impl System for MenuSystem {
  fn start(&mut self, state: &mut State) {
    state.title_text.render_text.text = String::from("TETRIS");
    state.play_button.render_text.text = String::from("Play");
    state.play_button.set_focus(true);
    state.quit_button.set_focus(false);
  }

  fn update_state(&self, input: &mut Input, state: &mut State) {
    if input.esc_pressed {
      state.game_state = GameState::Quitting;
      input.esc_pressed = false;
    }

    if state.play_button.focused() && input.ui_down_pressed() {
      state.events.push(Event::FocusChanged);
      state.play_button.set_focus(false);
      state.quit_button.set_focus(true);
    } else if state.quit_button.focused() && input.ui_up_pressed() {
      state.events.push(Event::FocusChanged);
      state.quit_button.set_focus(false);
      state.play_button.set_focus(true);
    }

    if input.enter_pressed {
      state.events.push(Event::ButtonPressed);
      state.game_state = if state.play_button.focused() {
        GameState::Playing
      } else {
        GameState::Quitting
      };
      input.enter_pressed = false;
    }
  }
}

pub struct PlaySystem;

impl System for PlaySystem {
  fn start(&mut self, state: &mut State) {
    state.board.clear();
    state.score = 0;
    state.rows = 0;
    state.level = 1;
    state.hold = None;
    state.hold_used = false;
    state.fall_timer = 0.0;
    state.move_timer = 0.0;
    state.piece = None;
    state.update_panel_text();
    spawn(state);
  }

  fn update_state(&self, input: &mut Input, state: &mut State) {
    if input.esc_pressed {
      input.clear();
      state.game_state = GameState::MainMenu;
      return;
    }

    if state.piece.is_none() {
      spawn(state);
    }

    if input.hold_pressed {
      hold(state);
    }

    if input.rotate_cw_pressed {
      rotate(state, true);
    }
    if input.rotate_ccw_pressed {
      rotate(state, false);
    }

    step_sideways(input, state);

    if input.hard_drop_pressed {
      hard_drop(state);
    } else {
      apply_gravity(input, state);
    }

    state.update_panel_text();
    input.clear_presses();
  }
}

pub struct PauseSystem;

impl System for PauseSystem {
  fn start(&mut self, state: &mut State) {
    state.title_text.render_text.text = String::from("Paused");
    state.play_button.render_text.text = String::from("Resume");
    state.play_button.set_focus(true);
  }

  fn update_state(&self, input: &mut Input, state: &mut State) {
    if input.enter_pressed {
      state.events.push(Event::ButtonPressed);
      state.game_state = GameState::Playing;
      state.title_text.render_text.text = String::from("TETRIS");
      state.play_button.render_text.text = String::from("Play");
      input.enter_pressed = false;
    }
  }
}

pub struct GameOverSystem {
  last_time: std::time::Instant,
}

impl GameOverSystem {
  pub fn new() -> Self {
    Self {
      last_time: std::time::Instant::now(),
    }
  }
}

impl Default for GameOverSystem {
  fn default() -> Self {
    Self::new()
  }
}

impl System for GameOverSystem {
  fn start(&mut self, state: &mut State) {
    self.last_time = std::time::Instant::now();
    state.game_over_text.render_text.text = format!("Game Over  {}", state.score);
  }

  fn update_state(&self, input: &mut Input, state: &mut State) {
    if input.esc_pressed {
      state.game_state = GameState::Quitting;
      input.esc_pressed = false;
      return;
    }

    if self.last_time.elapsed().as_secs_f32() > 5.0 {
      state.game_state = GameState::MainMenu;
    }
  }
}

/// Puts the next piece from the bag at the top of the board. Spawning into the
/// stack means the board is full, which ends the game.
pub fn spawn(state: &mut State) {
  let shape = state.bag.next();
  spawn_shape(state, shape);
}

fn spawn_shape(state: &mut State, shape: Shape) {
  let piece = Piece::at(shape, (WIDTH / 2 - 1, HIDDEN_ROWS - 1));

  if state.board.fits(&piece) {
    state.piece = Some(piece);
    state.fall_timer = 0.0;
  } else {
    state.piece = None;
    state.events.push(Event::GameOver);
    state.game_state = GameState::GameOver;
  }
}

/// Puts the falling piece in the hold slot, once per piece.
pub fn hold(state: &mut State) {
  if state.hold_used {
    return;
  }

  if let Some(piece) = state.piece {
    let held = state.hold.replace(piece.shape);
    match held {
      Some(shape) => spawn_shape(state, shape),
      None => spawn(state),
    }
    state.hold_used = true;
  }
}

/// Turns the piece, trying a few sideways nudges when it doesn't fit.
pub fn rotate(state: &mut State, clockwise: bool) {
  if let Some(piece) = state.piece {
    let turned = if clockwise {
      piece.rotated_cw()
    } else {
      piece.rotated_ccw()
    };

    if state.board.fits(&turned) {
      state.piece = Some(turned);
      return;
    }

    for kick in KICKS.iter() {
      let kicked = turned.moved(*kick, 0);
      if state.board.fits(&kicked) {
        state.piece = Some(kicked);
        return;
      }
    }
  }
}

/// Moves the piece one column, if the cells it wants are free.
pub fn try_move(state: &mut State, dx: i32) -> bool {
  if let Some(piece) = state.piece {
    let moved = piece.moved(dx, 0);
    if state.board.fits(&moved) {
      state.piece = Some(moved);
      return true;
    }
  }

  false
}

fn step_sideways(input: &Input, state: &mut State) {
  let direction = match (input.left_pressed, input.right_pressed) {
    (true, false) => -1,
    (false, true) => 1,
    _ => {
      state.move_timer = 0.0;
      return;
    }
  };

  if state.move_timer <= 0.0 {
    // the first move happens at once, then the key has to be held to repeat
    let first = state.move_timer == 0.0;
    try_move(state, direction);
    state.move_timer = if first { MOVE_DELAY } else { MOVE_REPEAT };
  } else {
    state.move_timer -= state.delta_time;
    if state.move_timer <= 0.0 {
      try_move(state, direction);
      state.move_timer = MOVE_REPEAT;
    }
  }
}

fn apply_gravity(input: &Input, state: &mut State) {
  let interval = if input.down_pressed {
    util::fall_interval(state.level) / util::SOFT_DROP_MULTIPLIER
  } else {
    util::fall_interval(state.level)
  };

  state.fall_timer += state.delta_time;

  while state.fall_timer >= interval {
    state.fall_timer -= interval;

    if fall_one_row(state) {
      if input.down_pressed {
        state.score += 1;
      }
    } else {
      lock_piece(state);
      return;
    }
  }
}

/// Drops the piece as far as it goes and locks it there.
pub fn hard_drop(state: &mut State) {
  let mut rows = 0;
  while fall_one_row(state) {
    rows += 1;
  }

  state.score += rows * 2;
  lock_piece(state);
}

fn fall_one_row(state: &mut State) -> bool {
  if let Some(piece) = state.piece {
    let moved = piece.moved(0, 1);
    if state.board.fits(&moved) {
      state.piece = Some(moved);
      return true;
    }
  }

  false
}

/// Locks the piece into the stack, clears any full rows, and brings the next
/// piece in.
pub fn lock_piece(state: &mut State) {
  if let Some(piece) = state.piece {
    state.board.lock(&piece);
  }
  state.piece = None;

  state.events.push(Event::PieceLocked);

  let cleared = state.board.clear_full_rows();
  if cleared > 0 {
    state.events.push(Event::RowsCleared);
    state.rows += cleared;
    state.score += util::score_for_rows(cleared, state.level);
    state.level = util::level_for_rows(state.rows);
  }

  state.hold_used = false;
  state.fall_timer = 0.0;
  spawn(state);
  state.update_panel_text();
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::board::{HEIGHT, WIDTH};
  use crate::piece::Shape;

  fn playing_state() -> State {
    let mut state = State::new();
    state.layout((800.0, 600.0).into());
    state.game_state = GameState::Playing;
    state.delta_time = 1.0 / 60.0;
    state.piece = Some(Piece::at(Shape::T, (4, 5)));
    state
  }

  fn play(input: &mut Input, state: &mut State) {
    PlaySystem.update_state(input, state);
  }

  fn piece(state: &State) -> Piece {
    state.piece.expect("a piece is falling")
  }

  fn fill_row(state: &mut State, y: i32) {
    for x in 0..WIDTH {
      state.board.fill(x, y, Shape::I.color());
    }
  }

  #[test]
  fn gravity_drops_a_row_per_interval() {
    let mut state = playing_state();
    let start = piece(&state).position.1;
    state.delta_time = util::fall_interval(state.level);

    play(&mut Input::new(), &mut state);

    assert_eq!(piece(&state).position.1, start + 1);
  }

  #[test]
  fn left_and_right_move_the_piece() {
    let mut state = playing_state();
    let start = piece(&state).position.0;

    let mut input = Input::new();
    input.left_pressed = true;
    play(&mut input, &mut state);
    assert_eq!(piece(&state).position.0, start - 1);

    let mut state = playing_state();
    let mut input = Input::new();
    input.right_pressed = true;
    play(&mut input, &mut state);
    assert_eq!(piece(&state).position.0, start + 1);
  }

  #[test]
  fn a_move_into_a_wall_does_not_happen() {
    let mut state = playing_state();
    state.piece = Some(Piece::at(Shape::T, (2, 5)));

    let mut input = Input::new();
    input.left_pressed = true;
    play(&mut input, &mut state);
    assert_eq!(piece(&state).position.0, 1);

    // the T's left cell is against the wall now, so another move changes nothing
    state.move_timer = 0.0;
    play(&mut input, &mut state);
    assert_eq!(piece(&state).position.0, 1);
  }

  #[test]
  fn a_move_into_a_filled_cell_does_not_happen() {
    let mut state = playing_state();
    for (x, y) in piece(&state).board_cells().iter() {
      state.board.fill(x - 1, *y, Shape::I.color());
    }

    let mut input = Input::new();
    input.left_pressed = true;
    play(&mut input, &mut state);

    assert_eq!(piece(&state).position.0, 4);
  }

  #[test]
  fn holding_left_repeats_after_a_delay() {
    let mut state = playing_state();
    let start = piece(&state).position.0;
    let mut input = Input::new();
    input.left_pressed = true;

    play(&mut input, &mut state);
    assert_eq!(piece(&state).position.0, start - 1);

    // not yet: the key has to be held past the delay
    state.delta_time = MOVE_DELAY * 0.5;
    play(&mut input, &mut state);
    assert_eq!(piece(&state).position.0, start - 1);

    play(&mut input, &mut state);
    assert_eq!(piece(&state).position.0, start - 2);
  }

  #[test]
  fn rotation_kicks_off_the_wall() {
    let mut state = playing_state();
    // an I piece against the left wall has nowhere to turn without a kick
    state.piece = Some(Piece::at(Shape::I, (0, 5)).rotated_cw());

    let mut input = Input::new();
    input.rotate_cw_pressed = true;
    play(&mut input, &mut state);

    assert!(state.board.fits(&piece(&state)));
    assert!(piece(&state).board_cells().iter().all(|(x, _)| *x >= 0));
  }

  #[test]
  fn blocked_rotation_does_not_happen() {
    let mut state = playing_state();
    let upright = Piece::at(Shape::T, (4, 5));
    state.piece = Some(upright);

    // fill the whole board except the cells the piece itself covers, so no
    // rotation and no kick has anywhere to go
    let own = upright.board_cells();
    for y in 0..HEIGHT {
      for x in 0..WIDTH {
        if !own.contains(&(x, y)) {
          state.board.fill(x, y, Shape::I.color());
        }
      }
    }

    let before = piece(&state).board_cells();
    let mut input = Input::new();
    input.rotate_cw_pressed = true;
    play(&mut input, &mut state);

    assert_eq!(piece(&state).board_cells(), before);
  }

  #[test]
  fn soft_drop_falls_faster() {
    let mut state = playing_state();
    let start = piece(&state).position.1;
    // a tenth of a normal interval, which soft drop turns into two rows
    state.delta_time = util::fall_interval(state.level) / 10.0;

    let mut input = Input::new();
    input.down_pressed = true;
    play(&mut input, &mut state);

    assert_eq!(piece(&state).position.1, start + 2);
  }

  #[test]
  fn soft_drop_scores_a_point_a_row() {
    let mut state = playing_state();
    state.delta_time = util::fall_interval(state.level) / 10.0;

    let mut input = Input::new();
    input.down_pressed = true;
    play(&mut input, &mut state);

    assert_eq!(state.score, 2);
  }

  #[test]
  fn hard_drop_lands_on_the_stack() {
    let mut state = playing_state();
    fill_row(&mut state, HEIGHT - 1);

    let mut input = Input::new();
    input.hard_drop_pressed = true;
    play(&mut input, &mut state);

    // the T locked on top of the full row, which then cleared
    assert_eq!(state.rows, 1);
    assert!(state.board.cell(4, HEIGHT - 1).is_some());
  }

  #[test]
  fn hard_drop_scores_two_a_row() {
    let mut state = playing_state();
    let start = piece(&state).position.1;

    let mut input = Input::new();
    input.hard_drop_pressed = true;
    play(&mut input, &mut state);

    // the T bottoms out with its wide row on the floor
    let rows = (HEIGHT - 1) - start;
    assert_eq!(state.score, (rows as u32) * 2);
  }

  #[test]
  fn a_piece_that_cannot_fall_locks() {
    let mut state = playing_state();
    state.piece = Some(Piece::at(Shape::T, (4, HEIGHT - 1)));
    state.delta_time = util::fall_interval(state.level);

    play(&mut Input::new(), &mut state);

    assert!(state.board.cell(4, HEIGHT - 1).is_some());
    // and the next piece is already falling
    assert_eq!(piece(&state).position.1, HIDDEN_ROWS - 1);
  }

  #[test]
  fn holding_with_an_empty_slot_takes_the_next_piece() {
    let mut state = playing_state();
    let held = piece(&state).shape;
    let next = state.bag.peek();

    let mut input = Input::new();
    input.hold_pressed = true;
    play(&mut input, &mut state);

    assert_eq!(state.hold, Some(held));
    assert_eq!(piece(&state).shape, next);
  }

  #[test]
  fn holding_again_swaps_the_pieces() {
    let mut state = playing_state();
    let first = piece(&state).shape;

    let mut input = Input::new();
    input.hold_pressed = true;
    play(&mut input, &mut state);

    let second = piece(&state).shape;
    state.hold_used = false;
    input.hold_pressed = true;
    play(&mut input, &mut state);

    assert_eq!(state.hold, Some(second));
    assert_eq!(piece(&state).shape, first);
  }

  #[test]
  fn a_held_piece_comes_back_upright() {
    let mut state = playing_state();
    state.piece = Some(Piece::at(Shape::T, (4, 5)).rotated_cw());

    let mut input = Input::new();
    input.hold_pressed = true;
    play(&mut input, &mut state);

    state.hold_used = false;
    input.hold_pressed = true;
    play(&mut input, &mut state);

    let back = piece(&state);
    assert_eq!(back.shape, Shape::T);
    assert_eq!(
      back.board_cells(),
      Piece::at(Shape::T, back.position).board_cells()
    );
  }

  #[test]
  fn hold_only_works_once_per_piece() {
    let mut state = playing_state();

    let mut input = Input::new();
    input.hold_pressed = true;
    play(&mut input, &mut state);
    let after_first = piece(&state).shape;
    let held = state.hold;

    input.hold_pressed = true;
    play(&mut input, &mut state);

    assert_eq!(state.hold, held);
    assert_eq!(piece(&state).shape, after_first);
  }

  #[test]
  fn locking_frees_the_hold() {
    let mut state = playing_state();
    let mut input = Input::new();
    input.hold_pressed = true;
    play(&mut input, &mut state);
    assert!(state.hold_used);

    input.hard_drop_pressed = true;
    play(&mut input, &mut state);

    assert!(!state.hold_used);
  }

  #[test]
  fn spawning_into_the_stack_ends_the_game() {
    let mut state = playing_state();
    // block the spawn cells without filling whole rows, which would just clear
    for y in 0..HIDDEN_ROWS + 1 {
      for x in 3..=5 {
        state.board.fill(x, y, Shape::I.color());
      }
    }
    state.piece = Some(Piece::at(Shape::T, (4, HEIGHT - 1)));

    lock_piece(&mut state);

    assert_eq!(state.game_state, GameState::GameOver);
    assert!(state.piece.is_none());
  }

  #[test]
  fn locking_a_piece_raises_an_event() {
    let mut state = playing_state();
    state.piece = Some(Piece::at(Shape::T, (4, HEIGHT - 1)));

    lock_piece(&mut state);

    assert!(state.events.contains(&Event::PieceLocked));
    assert!(!state.events.contains(&Event::RowsCleared));
  }

  #[test]
  fn clearing_rows_raises_an_event() {
    let mut state = playing_state();
    // fill the bottom row except where the piece will land, so locking completes it
    let piece = Piece::at(Shape::T, (4, HEIGHT - 1));
    let own = piece.board_cells();
    for x in 0..WIDTH {
      if !own.contains(&(x, HEIGHT - 1)) {
        state.board.fill(x, HEIGHT - 1, Shape::I.color());
      }
    }
    state.piece = Some(piece);

    lock_piece(&mut state);

    assert!(state.events.contains(&Event::RowsCleared));
  }

  #[test]
  fn topping_out_raises_an_event() {
    let mut state = playing_state();
    for y in 0..HIDDEN_ROWS + 1 {
      for x in 3..=5 {
        state.board.fill(x, y, Shape::I.color());
      }
    }
    state.piece = Some(Piece::at(Shape::T, (4, HEIGHT - 1)));

    lock_piece(&mut state);

    assert!(state.events.contains(&Event::GameOver));
  }

  #[test]
  fn escape_returns_to_the_menu() {
    let mut state = playing_state();
    let mut input = Input::new();
    input.esc_pressed = true;

    play(&mut input, &mut state);

    assert_eq!(state.game_state, GameState::MainMenu);
  }

  #[test]
  fn escape_quits_from_the_menu() {
    let mut state = State::new();
    state.layout((800.0, 600.0).into());
    let mut input = Input::new();
    input.esc_pressed = true;

    MenuSystem.update_state(&mut input, &mut state);

    assert_eq!(state.game_state, GameState::Quitting);
  }

  #[test]
  fn starting_a_game_clears_the_board() {
    let mut state = playing_state();
    state.score = 900;
    state.rows = 12;
    state.level = 4;
    state.hold = Some(Shape::I);
    state.hold_used = true;
    fill_row(&mut state, HEIGHT - 1);

    PlaySystem.start(&mut state);

    assert_eq!(state.score, 0);
    assert_eq!(state.rows, 0);
    assert_eq!(state.level, 1);
    assert!(state.hold.is_none());
    assert!(!state.hold_used);
    assert!(state.board.cell(0, HEIGHT - 1).is_none());
    assert!(state.piece.is_some());
  }

  #[test]
  fn resuming_restores_the_menu_text() {
    let mut state = playing_state();
    PauseSystem.start(&mut state);
    state.game_state = GameState::Paused;
    assert_eq!(state.play_button.render_text.text, "Resume");

    let mut input = Input::new();
    input.enter_pressed = true;
    PauseSystem.update_state(&mut input, &mut state);

    assert_eq!(state.game_state, GameState::Playing);
    assert_eq!(state.title_text.render_text.text, "TETRIS");
    assert_eq!(state.play_button.render_text.text, "Play");
  }
}
