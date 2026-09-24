use crate::bag::Bag;
use crate::board::{Board, HEIGHT, WIDTH};
use crate::layout::Layout;
use crate::piece::{Piece, Shape};
use crate::tetris_game::Event;
use blitzkit::geometry::quad::Quad;
use blitzkit::geometry::Geometry;
use blitzkit::renderer::render_text::{RenderText, TextRenderer, UNBOUNDED_F32};
use glam::Vec2;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver,
    Quitting,
}

pub struct TetrisText {
    pub render_text: RenderText,
    pub visible: bool,
}

impl TetrisText {
    pub fn focused(&self) -> bool {
        self.render_text.focused
    }

    pub fn set_focus(&mut self, focused: bool) {
        self.render_text.focused = focused;
    }
}

fn text(content: &str, size: f32) -> TetrisText {
    TetrisText {
        visible: false,
        render_text: RenderText {
            position: (0.0, 0.0).into(),
            color: (1.0, 1.0, 1.0, 1.0).into(),
            text: String::from(content),
            size,
            ..Default::default()
        },
    }
}

pub struct State {
    pub game_state: GameState,
    pub board: Board,
    /// The falling piece, absent between locking one and spawning the next.
    pub piece: Option<Piece>,
    pub hold: Option<Shape>,
    /// Hold is one use per piece, and this says whether it has been used.
    pub hold_used: bool,
    pub bag: Bag,
    pub score: u32,
    pub rows: u32,
    pub level: u32,
    /// Seconds since the last gravity step.
    pub fall_timer: f32,
    /// Seconds until a held left or right key moves the piece again.
    pub move_timer: f32,
    pub layout: Layout,
    /// Seconds since the previous update.
    pub delta_time: f32,
    /// What happened this frame, drained by the game to play sounds.
    pub events: Vec<Event>,

    pub title_text: TetrisText,
    pub play_button: TetrisText,
    pub quit_button: TetrisText,
    pub score_text: TetrisText,
    pub level_text: TetrisText,
    pub rows_text: TetrisText,
    pub hold_label: TetrisText,
    pub next_label: TetrisText,
    pub game_over_text: TetrisText,
}

impl State {
    pub fn new() -> Self {
        let mut game_over_text = text("", 32.0);
        game_over_text.render_text.bounds = (UNBOUNDED_F32, UNBOUNDED_F32).into();
        game_over_text.render_text.centered = true;

        Self {
            game_state: GameState::MainMenu,
            board: Board::new(),
            piece: None,
            hold: None,
            hold_used: false,
            bag: Bag::new(),
            score: 0,
            rows: 0,
            level: 1,
            fall_timer: 0.0,
            move_timer: 0.0,
            layout: Layout::new((0.0, 0.0).into()),
            delta_time: 0.0,
            events: Vec::new(),

            title_text: text("TETRIS", 64.0),
            play_button: text("Play", 32.0),
            quit_button: text("Quit", 32.0),
            score_text: text("Score: 0", 16.0),
            level_text: text("Level: 1", 16.0),
            rows_text: text("Rows: 0", 16.0),
            hold_label: text("HOLD (C)", 16.0),
            next_label: text("NEXT", 16.0),
            game_over_text,
        }
    }

    /// Places everything for a window of `size` pixels. The board is stored in
    /// cells, so a resize only changes where things are drawn.
    pub fn layout(&mut self, size: Vec2) {
        self.layout = Layout::new(size);
        let layout = self.layout;
        // narrow enough that "Score: 12345" fits the panel even in a small window
        let panel_size = (layout.cell * 0.3).clamp(8.0, 18.0);

        self.title_text.render_text.position = (20.0, 20.0).into();
        self.play_button.render_text.position = (40.0, 100.0).into();
        self.quit_button.render_text.position = (40.0, 160.0).into();

        let left = layout.left_panel();
        self.hold_label.render_text.position = (left.x, left.y).into();
        self.score_text.render_text.position = (left.x, left.y + layout.cell * 6.0).into();
        self.level_text.render_text.position = (left.x, left.y + layout.cell * 7.5).into();
        self.rows_text.render_text.position = (left.x, left.y + layout.cell * 9.0).into();

        let right = layout.right_panel();
        self.next_label.render_text.position = (right.x, right.y).into();

        // centered on the panel, so nothing spills over the board
        for label in [
            &mut self.hold_label,
            &mut self.next_label,
            &mut self.score_text,
            &mut self.level_text,
            &mut self.rows_text,
        ] {
            label.render_text.size = panel_size;
            label.render_text.centered = true;
        }

        self.game_over_text.render_text.position = size * 0.5;
    }

    pub fn update_panel_text(&mut self) {
        self.score_text.render_text.text = format!("Score: {}", self.score);
        self.level_text.render_text.text = format!("Level: {}", self.level);
        self.rows_text.render_text.text = format!("Rows: {}", self.rows);
    }

    pub fn initialize(&mut self, geometry: &mut Geometry, text_renderer: &mut TextRenderer) {
        self.update_geometry(geometry);
        self.update_text(text_renderer);
    }

    pub fn update(&self, geometry: &mut Geometry, text_renderer: &mut TextRenderer) {
        self.update_geometry(geometry);
        self.update_text(text_renderer);
    }

    fn in_game(&self) -> bool {
        self.game_state == GameState::Playing
            || self.game_state == GameState::Paused
            || self.game_state == GameState::GameOver
    }

    fn update_geometry(&self, geometry: &mut Geometry) {
        if !self.in_game() {
            return;
        }

        self.push_border(geometry);

        for y in 0..HEIGHT {
            if !self.layout.is_visible_row(y) {
                continue;
            }
            for x in 0..WIDTH {
                if let Some(color) = self.board.cell(x, y) {
                    geometry.push_quad(&self.block(x, y, color));
                }
            }
        }

        if let Some(piece) = self.piece {
            for (x, y) in piece.board_cells().iter() {
                if self.layout.is_visible_row(*y) {
                    geometry.push_quad(&self.block(*x, *y, piece.color()));
                }
            }
        }

        if let Some(shape) = self.hold {
            self.push_preview(geometry, shape, self.layout.left_panel());
        }
        self.push_preview(geometry, self.bag.peek(), self.layout.right_panel());
    }

    /// One cell of the stack, inset a little so the blocks read separately.
    fn block(&self, x: i32, y: i32, color: glam::Vec4) -> Quad {
        let size = self.layout.cell - (self.layout.cell * 0.1).max(1.0);
        Quad::colored(self.layout.cell_center(x, y), (size, size).into(), color)
    }

    fn push_border(&self, geometry: &mut Geometry) {
        let board = self.layout.board_size();
        let origin = self.layout.origin;
        let thickness = (self.layout.cell * 0.15).max(2.0);
        let border = glam::vec4(0.6, 0.6, 0.65, 1.0);

        let edges = [
            (
                (origin.x + board.x * 0.5, origin.y),
                (board.x + thickness, thickness),
            ),
            (
                (origin.x + board.x * 0.5, origin.y + board.y),
                (board.x + thickness, thickness),
            ),
            (
                (origin.x, origin.y + board.y * 0.5),
                (thickness, board.y + thickness),
            ),
            (
                (origin.x + board.x, origin.y + board.y * 0.5),
                (thickness, board.y + thickness),
            ),
        ];

        for (position, size) in edges.iter() {
            geometry.push_quad(&Quad::colored(
                (position.0, position.1).into(),
                (size.0, size.1).into(),
                border,
            ));
        }
    }

    /// Draws a piece in a panel, under its label.
    fn push_preview(&self, geometry: &mut Geometry, shape: Shape, anchor: Vec2) {
        let cell = self.layout.cell * 0.8;
        let size = cell - (cell * 0.1).max(1.0);
        let center = Vec2::new(anchor.x, anchor.y + self.layout.cell * 2.5);

        for (x, y) in Piece::new(shape).board_cells().iter() {
            let position = Vec2::new(center.x + *x as f32 * cell, center.y + *y as f32 * cell);
            geometry.push_quad(&Quad::colored(position, (size, size).into(), shape.color()));
        }
    }

    fn update_text(&self, text_renderer: &mut TextRenderer) {
        for text in [
            &self.title_text,
            &self.play_button,
            &self.quit_button,
            &self.score_text,
            &self.level_text,
            &self.rows_text,
            &self.hold_label,
            &self.next_label,
            &self.game_over_text,
        ] {
            if text.visible {
                text_renderer.push_render_text(text.render_text.clone());
            }
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state_of(width: f32, height: f32) -> State {
        let mut state = State::new();
        state.layout((width, height).into());
        state
    }

    #[test]
    fn a_new_state_starts_on_the_menu() {
        let state = State::new();

        assert_eq!(state.game_state, GameState::MainMenu);
        assert_eq!(state.level, 1);
        assert!(state.piece.is_none());
        assert!(state.hold.is_none());
    }

    #[test]
    fn layout_places_the_panels_beside_the_board() {
        let state = state_of(800.0, 600.0);

        assert!(state.hold_label.render_text.position.x < state.layout.origin.x);
        assert!(
            state.next_label.render_text.position.x
                > state.layout.origin.x + state.layout.board_size().x
        );
    }

    #[test]
    fn the_hold_label_names_its_key() {
        let state = State::new();

        assert_eq!(state.hold_label.render_text.text, "HOLD (C)");
    }

    #[test]
    fn panel_text_follows_the_score() {
        let mut state = state_of(800.0, 600.0);
        state.score = 1200;
        state.level = 3;
        state.rows = 24;
        state.update_panel_text();

        assert_eq!(state.score_text.render_text.text, "Score: 1200");
        assert_eq!(state.level_text.render_text.text, "Level: 3");
        assert_eq!(state.rows_text.render_text.text, "Rows: 24");
    }

    #[test]
    fn game_over_text_is_centered_in_the_window() {
        let state = state_of(800.0, 600.0);

        assert_eq!(state.game_over_text.render_text.position.x, 400.0);
        assert_eq!(state.game_over_text.render_text.position.y, 300.0);
    }
}
