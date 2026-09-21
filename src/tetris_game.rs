use blitzkit::geometry::Geometry;
use blitzkit::keyboard::*;
use blitzkit::renderer::render_text::TextRenderer;
use blitzkit::sound::SoundSystem;
use blitzkit::Game;

use crate::input::Input;
use crate::state::*;
use crate::system::*;

use std::io::Cursor;

const BLIP_BYTES: &[u8] = include_bytes!("../res/sounds/4362__noisecollector__pongblipa-4.wav");

/// Something worth a sound, raised by a system and drained each frame.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Event {
  ButtonPressed,
  FocusChanged,
  PieceLocked,
  RowsCleared,
  GameOver,
}

pub struct SoundPack {
  blip: Cursor<&'static [u8]>,
}

impl SoundPack {
  pub fn new() -> Self {
    Self {
      blip: Cursor::new(BLIP_BYTES),
    }
  }

  /// The one sample this game has, played at `speed` so different events sound
  /// different: higher for a line clear, lower for topping out.
  pub fn blip(&self, speed: f32) -> impl rodio::Source + Send + 'static {
    use rodio::Source;
    rodio::Decoder::new(self.blip.clone())
      .expect("the blip is a wav that ships with the game")
      .speed(speed)
  }
}

impl Default for SoundPack {
  fn default() -> Self {
    Self::new()
  }
}

pub struct TetrisGame {
  pub input: Input,
  state: State,
  menu_system: MenuSystem,
  visibility_system: VisibilitySystem,
  play_system: PlaySystem,
  pause_system: PauseSystem,
  game_over_system: GameOverSystem,
  sound_pack: SoundPack,
}

impl TetrisGame {
  pub fn new() -> Self {
    Self {
      input: Input::new(),
      state: State::new(),
      menu_system: MenuSystem,
      visibility_system: VisibilitySystem,
      play_system: PlaySystem,
      pause_system: PauseSystem,
      game_over_system: GameOverSystem::new(),
      sound_pack: SoundPack::new(),
    }
  }
}

impl Default for TetrisGame {
  fn default() -> Self {
    Self::new()
  }
}

impl Game for TetrisGame {
  fn initialize(
    &mut self,
    geometry: &mut Geometry,
    text_renderer: &mut TextRenderer,
    _sound_system: &SoundSystem,
    window_size: (f32, f32),
  ) {
    self.resized(window_size);
    self.menu_system.start(&mut self.state);
    self.state.initialize(geometry, text_renderer);
  }

  fn update(
    &mut self,
    dt: f32,
    geometry: &mut Geometry,
    text_renderer: &mut TextRenderer,
    sound_system: &SoundSystem,
  ) {
    self.state.delta_time = dt;

    self
      .visibility_system
      .update_state(&mut self.input, &mut self.state);

    match self.state.game_state {
      GameState::MainMenu => {
        self
          .menu_system
          .update_state(&mut self.input, &mut self.state);
        if self.state.game_state == GameState::Playing {
          self.play_system.start(&mut self.state);
        }
      }
      GameState::Playing => {
        self
          .play_system
          .update_state(&mut self.input, &mut self.state);
        if self.state.game_state == GameState::MainMenu {
          self.menu_system.start(&mut self.state);
        } else if self.state.game_state == GameState::GameOver {
          self.game_over_system.start(&mut self.state);
        }
      }
      GameState::Paused => {
        self
          .pause_system
          .update_state(&mut self.input, &mut self.state);
      }
      GameState::GameOver => {
        self
          .game_over_system
          .update_state(&mut self.input, &mut self.state);
        if self.state.game_state == GameState::MainMenu {
          self.menu_system.start(&mut self.state);
        }
      }
      GameState::Quitting => {}
    }

    for event in self.state.events.drain(..) {
      let speed = match event {
        Event::RowsCleared => 1.6,
        Event::PieceLocked => 0.8,
        Event::GameOver => 0.5,
        Event::ButtonPressed | Event::FocusChanged => 1.0,
      };
      sound_system.queue(self.sound_pack.blip(speed));
    }

    self.input.clear_presses();

    geometry.reset();
    text_renderer.reset();
    self.state.update(geometry, text_renderer);
  }

  fn process_keyboard(&mut self, input: KeyboardInput) {
    self.input.update(input);
  }

  fn is_quitting(&self) -> bool {
    self.state.game_state == GameState::Quitting
  }

  fn focus_changed(&mut self, focus: bool) {
    // only a game in progress can be paused; losing focus on the menu or the
    // game over screen leaves the screen alone
    if !focus && self.state.game_state == GameState::Playing {
      self.pause_system.start(&mut self.state);
      self.state.game_state = GameState::Paused;
    }
  }

  fn resized(&mut self, window_size: (f32, f32)) {
    self.state.layout(window_size.into());
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use blitzkit::geometry::Geometry;

  fn game_in(game_state: GameState) -> TetrisGame {
    let mut game = TetrisGame::new();
    game.resized((800.0, 600.0));
    game.state.game_state = game_state;
    game
  }

  #[test]
  fn losing_focus_while_playing_pauses() {
    let mut game = game_in(GameState::Playing);
    game.focus_changed(false);

    assert_eq!(game.state.game_state, GameState::Paused);
    assert_eq!(game.state.play_button.render_text.text, "Resume");
  }

  #[test]
  fn losing_focus_on_the_menu_does_nothing() {
    let mut game = game_in(GameState::MainMenu);
    game.focus_changed(false);

    assert_eq!(game.state.game_state, GameState::MainMenu);
    assert_eq!(game.state.title_text.render_text.text, "TETRIS");
  }

  #[test]
  fn nothing_falls_while_paused() {
    let mut game = game_in(GameState::Playing);
    game.play_system.start(&mut game.state);
    let before = game.state.piece.expect("a piece is falling").position;

    game.focus_changed(false);

    let mut geometry = Geometry::new();
    let mut text_renderer = TextRenderer::new();
    let sound_system = SoundSystem::new();
    for _ in 0..10 {
      game.update(1.0, &mut geometry, &mut text_renderer, &sound_system);
    }

    assert_eq!(game.state.game_state, GameState::Paused);
    assert_eq!(game.state.piece.expect("still there").position, before);
  }
}
