use blitzkit::keyboard::*;

/// Held flags for the keys the game watches. Moves and rotations happen on the
/// press, so those are cleared once a system has acted on them.
#[derive(Debug, Default)]
pub struct Input {
    pub left_pressed: bool,
    pub right_pressed: bool,
    pub down_pressed: bool,
    pub rotate_cw_pressed: bool,
    pub rotate_ccw_pressed: bool,
    pub hard_drop_pressed: bool,
    pub hold_pressed: bool,
    pub up_pressed: bool,
    pub enter_pressed: bool,
    pub esc_pressed: bool,
}

impl Input {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn update(&mut self, input: KeyboardInput) {
        let pressed = input.state == KeyboardKeyState::Pressed;
        let first_press = pressed && !input.repeat;

        match input.key {
            KeyboardKey::Left | KeyboardKey::A => self.left_pressed = pressed,
            KeyboardKey::Right | KeyboardKey::D => self.right_pressed = pressed,
            KeyboardKey::Down | KeyboardKey::S => self.down_pressed = pressed,
            KeyboardKey::Up | KeyboardKey::X => {
                self.up_pressed = pressed;
                if first_press {
                    self.rotate_cw_pressed = true;
                }
            }
            KeyboardKey::Z => {
                if first_press {
                    self.rotate_ccw_pressed = true;
                }
            }
            KeyboardKey::Space => {
                if first_press {
                    self.hard_drop_pressed = true;
                }
            }
            KeyboardKey::C => {
                if first_press {
                    self.hold_pressed = true;
                }
            }
            KeyboardKey::Return if first_press => self.enter_pressed = true,
            KeyboardKey::Escape if first_press => self.esc_pressed = true,
            _ => (),
        }
    }

    /// Menus move with up and down, whichever keys the player uses.
    pub fn ui_up_pressed(&self) -> bool {
        self.up_pressed
    }

    pub fn ui_down_pressed(&self) -> bool {
        self.down_pressed
    }

    /// Forgets the one-shot presses, which a system calls once it has acted.
    pub fn clear_presses(&mut self) {
        self.rotate_cw_pressed = false;
        self.rotate_ccw_pressed = false;
        self.hard_drop_pressed = false;
        self.hold_pressed = false;
        self.enter_pressed = false;
        self.esc_pressed = false;
    }

    pub fn clear(&mut self) {
        *self = Input::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(key: KeyboardKey, state: KeyboardKeyState, repeat: bool) -> KeyboardInput {
        KeyboardInput::new(key, state, repeat)
    }

    #[test]
    fn escape_ignores_key_repeat() {
        let mut input = Input::new();
        input.update(key(KeyboardKey::Escape, KeyboardKeyState::Pressed, true));

        assert!(!input.esc_pressed);
    }

    #[test]
    fn escape_ignores_release() {
        let mut input = Input::new();
        input.update(key(KeyboardKey::Escape, KeyboardKeyState::Released, false));
        assert!(!input.esc_pressed);

        input.update(key(KeyboardKey::Escape, KeyboardKeyState::Pressed, false));
        assert!(input.esc_pressed);
    }

    #[test]
    fn movement_keys_are_held() {
        let mut input = Input::new();
        input.update(key(KeyboardKey::Left, KeyboardKeyState::Pressed, false));
        assert!(input.left_pressed);

        input.update(key(KeyboardKey::Left, KeyboardKeyState::Released, false));
        assert!(!input.left_pressed);
    }

    #[test]
    fn rotation_happens_once_per_press() {
        let mut input = Input::new();
        input.update(key(KeyboardKey::Up, KeyboardKeyState::Pressed, false));
        assert!(input.rotate_cw_pressed);

        input.clear_presses();
        input.update(key(KeyboardKey::Up, KeyboardKeyState::Pressed, true));
        assert!(!input.rotate_cw_pressed);
    }
}
