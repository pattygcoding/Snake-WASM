use macroquad::prelude::*;

use crate::game::{Cell, Direction, Game, Phase, CELL_SIZE, GRID_SIZE};

const VIEW_WIDTH: f32 = 480.0;
const VIEW_HEIGHT: f32 = 656.0;
const BOARD: Rect = Rect::new(40.0, 170.0, 400.0, 400.0);
const PAUSE_BUTTON: Rect = Rect::new(352.0, 99.0, 40.0, 40.0);
const RESTART_BUTTON: Rect = Rect::new(400.0, 99.0, 40.0, 40.0);
const PRIMARY_BUTTON: Rect = Rect::new(152.0, 400.0, 176.0, 46.0);
const INK: Color = Color::new(0.94, 0.96, 0.95, 1.0);
const MUTED: Color = Color::new(0.69, 0.74, 0.71, 1.0);
const ACCENT: Color = Color::new(0.40, 0.88, 0.64, 1.0);
const SURFACE: Color = Color::new(0.15, 0.18, 0.16, 1.0);
const APPLE: Color = Color::new(0.98, 0.39, 0.42, 1.0);
const FONT_SIZE: u16 = 96;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Restart,
    TogglePause,
}

pub struct Ui {
    font: Font,
}

impl Ui {
    pub fn new() -> Self {
        let mut font = load_ttf_font_from_bytes(include_bytes!("../assets/fonts/Outfit.ttf"))
            .expect("the bundled Outfit font must be valid");
        font.set_filter(FilterMode::Linear);
        Self { font }
    }

    pub fn action_at(&self, phase: Phase, screen_position: Vec2) -> Option<Action> {
        hit_test(
            phase,
            to_layout(screen_position, screen_width(), screen_height()),
        )
    }

    pub fn draw(&self, game: &Game, now: f64) {
        clear_background(DARKGRAY);
        let scale = layout_scale(screen_width(), screen_height());
        set_camera(&Camera2D {
            target: vec2(VIEW_WIDTH / 2.0, VIEW_HEIGHT / 2.0),
            zoom: vec2(2.0 * scale / screen_width(), 2.0 * scale / screen_height()),
            ..Default::default()
        });

        let pointer = to_layout(
            Vec2::from(mouse_position()),
            screen_width(),
            screen_height(),
        );
        self.text("Snake", vec2(40.0, 66.0), 38.0, INK);
        self.right_text("20 x 20", vec2(440.0, 61.0), 15.0, MUTED);
        draw_line(
            40.0,
            83.0,
            440.0,
            83.0,
            1.0,
            Color::new(1.0, 1.0, 1.0, 0.10),
        );
        self.stat("SCORE", game.score, 40.0);
        self.stat("BEST", game.best_score, 144.0);
        self.toolbar(game.phase, pointer);

        rounded_rect(
            Rect::new(38.0, 172.0, 404.0, 404.0),
            8.0,
            Color::new(0.0, 0.0, 0.0, 0.13),
        );
        rounded_rect(
            Rect::new(39.0, 169.0, 402.0, 402.0),
            8.0,
            Color::new(0.55, 0.62, 0.57, 0.40),
        );
        rounded_rect(BOARD, 8.0, SURFACE);
        for row in 1..GRID_SIZE {
            for column in 1..GRID_SIZE {
                draw_circle(
                    BOARD.x + column as f32 * CELL_SIZE,
                    BOARD.y + row as f32 * CELL_SIZE,
                    0.75,
                    Color::new(0.37, 0.43, 0.39, 0.34),
                );
            }
        }

        if game.phase != Phase::Won {
            draw_apple(cell_center(game.apple), now);
        }
        self.snake(game, now);

        if game.phase != Phase::Playing {
            self.overlay(game, pointer);
        }

        draw_circle(
            45.0,
            599.0,
            3.0,
            if game.phase == Phase::Playing {
                ACCENT
            } else {
                MUTED
            },
        );
        let status = match game.phase {
            Phase::Ready => "Ready",
            Phase::Playing => "In play",
            Phase::Paused => "Paused",
            Phase::GameOver => "Round complete",
            Phase::Won => "Perfect run",
        };
        self.text(status, vec2(57.0, 604.0), 14.0, MUTED);
        self.right_text("WASD / Arrow keys", vec2(440.0, 604.0), 14.0, MUTED);
        self.center_text("Space to restart   /   P to pause", 633.0, 13.0, MUTED);
        set_default_camera();
    }

    fn text(&self, text: &str, position: Vec2, size: f32, color: Color) {
        draw_text_ex(
            text,
            position.x,
            position.y,
            TextParams {
                font: Some(&self.font),
                font_size: FONT_SIZE,
                font_scale: size / FONT_SIZE as f32,
                color,
                ..Default::default()
            },
        );
    }

    fn text_width(&self, text: &str, size: f32) -> f32 {
        measure_text(text, Some(&self.font), FONT_SIZE, size / FONT_SIZE as f32).width
    }

    fn center_text(&self, text: &str, baseline: f32, size: f32, color: Color) {
        self.text(
            text,
            vec2((VIEW_WIDTH - self.text_width(text, size)) / 2.0, baseline),
            size,
            color,
        );
    }

    fn right_text(&self, text: &str, position: Vec2, size: f32, color: Color) {
        self.text(
            text,
            vec2(position.x - self.text_width(text, size), position.y),
            size,
            color,
        );
    }

    fn stat(&self, label: &str, value: u32, left: f32) {
        self.text(label, vec2(left, 111.0), 12.0, MUTED);
        self.text(&format!("{value:02}"), vec2(left, 145.0), 30.0, INK);
    }

    fn toolbar(&self, phase: Phase, pointer: Vec2) {
        for (bounds, is_pause) in [(PAUSE_BUTTON, true), (RESTART_BUTTON, false)] {
            let enabled = !is_pause || matches!(phase, Phase::Playing | Phase::Paused);
            let hovered = enabled && bounds.contains(pointer);
            rounded_rect(
                bounds,
                8.0,
                Color::new(1.0, 1.0, 1.0, if hovered { 0.16 } else { 0.06 }),
            );
            let center = bounds.center();
            let color = if enabled {
                INK
            } else {
                Color::new(0.62, 0.65, 0.63, 0.45)
            };
            if is_pause && phase != Phase::Paused {
                rounded_rect(
                    Rect::new(center.x - 6.0, center.y - 7.0, 4.0, 14.0),
                    1.5,
                    color,
                );
                rounded_rect(
                    Rect::new(center.x + 2.0, center.y - 7.0, 4.0, 14.0),
                    1.5,
                    color,
                );
            } else if is_pause {
                play_icon(center, color);
            } else {
                let mut previous = center + vec2(7.0, 0.0);
                for step in 1..=24 {
                    let angle = step as f32 / 24.0 * std::f32::consts::PI * 1.65;
                    let next = center + vec2(angle.cos(), angle.sin()) * 7.0;
                    draw_line(previous.x, previous.y, next.x, next.y, 1.8, color);
                    previous = next;
                }
                draw_triangle(
                    previous + vec2(-4.0, -1.0),
                    previous + vec2(3.5, -3.0),
                    previous + vec2(1.0, 4.0),
                    color,
                );
            }
            if hovered {
                let label = if !is_pause {
                    "Restart"
                } else if phase == Phase::Paused {
                    "Resume"
                } else {
                    "Pause"
                };
                let width = self.text_width(label, 13.0) + 16.0;
                rounded_rect(
                    Rect::new(center.x - width / 2.0, 145.0, width, 23.0),
                    4.0,
                    SURFACE,
                );
                self.text(
                    label,
                    vec2(center.x - (width - 16.0) / 2.0, 161.0),
                    13.0,
                    INK,
                );
            }
        }
    }

    fn snake(&self, game: &Game, now: f64) {
        let progress = game.animation_progress(now);
        let centers: Vec<Vec2> = game
            .snake
            .iter()
            .enumerate()
            .map(|(index, &cell)| {
                let previous = game
                    .previous_snake
                    .get(index)
                    .copied()
                    .unwrap_or_else(|| *game.previous_snake.last().unwrap_or(&cell));
                cell_center(previous).lerp(cell_center(cell), progress)
            })
            .collect();

        for pair in centers.windows(2) {
            draw_line(
                pair[0].x,
                pair[0].y + 2.0,
                pair[1].x,
                pair[1].y + 2.0,
                16.0,
                Color::new(0.0, 0.0, 0.0, 0.18),
            );
        }
        for pair in centers.windows(2) {
            draw_line(pair[0].x, pair[0].y, pair[1].x, pair[1].y, 15.0, ACCENT);
        }
        for center in &centers {
            draw_circle(center.x, center.y, 7.5, ACCENT);
        }
        if let Some(&head) = centers.first() {
            draw_eyes(head, game.direction);
        }
    }

    fn overlay(&self, game: &Game, pointer: Vec2) {
        rounded_rect(BOARD, 8.0, Color::new(0.12, 0.15, 0.13, 0.96));
        let center = vec2(VIEW_WIDTH / 2.0, 258.0);
        let emblem = [
            center + vec2(-29.0, 8.0),
            center + vec2(-8.0, 8.0),
            center + vec2(-8.0, -9.0),
            center + vec2(26.0, -9.0),
        ];
        for pair in emblem.windows(2) {
            draw_line(pair[0].x, pair[0].y, pair[1].x, pair[1].y, 15.0, ACCENT);
        }
        for point in emblem {
            draw_circle(point.x, point.y, 7.5, ACCENT);
        }
        draw_eyes(emblem[3], Direction::Right);

        let (title, button, detail) = match game.phase {
            Phase::Ready => (
                "Snake",
                "Play",
                format!("Best run  /  {:02}", game.best_score),
            ),
            Phase::Paused => (
                "Paused",
                "Resume",
                format!("Current score  /  {:02}", game.score),
            ),
            Phase::GameOver => (
                "Game over",
                "Play again",
                format!(
                    "Score {:02}    /    Best {:02}",
                    game.score, game.best_score
                ),
            ),
            Phase::Won => (
                "Board complete",
                "Play again",
                format!("Perfect score  /  {}", game.score),
            ),
            Phase::Playing => return,
        };
        self.center_text(title, 329.0, 38.0, INK);
        self.center_text(&detail, 364.0, 17.0, MUTED);
        rounded_rect(
            PRIMARY_BUTTON,
            8.0,
            if PRIMARY_BUTTON.contains(pointer) {
                Color::new(0.55, 0.96, 0.74, 1.0)
            } else {
                ACCENT
            },
        );
        let width = self.text_width(button, 19.0);
        let left = (VIEW_WIDTH - width - 26.0) / 2.0;
        play_icon(vec2(left + 5.0, 423.0), SURFACE);
        self.text(button, vec2(left + 24.0, 430.0), 19.0, SURFACE);
        self.center_text(
            if game.phase == Phase::Paused {
                "P to resume"
            } else {
                "Space to play"
            },
            475.0,
            13.0,
            MUTED,
        );
    }
}

fn layout_scale(width: f32, height: f32) -> f32 {
    (width / VIEW_WIDTH).min(height / VIEW_HEIGHT).max(0.001)
}

fn to_layout(position: Vec2, width: f32, height: f32) -> Vec2 {
    let scale = layout_scale(width, height);
    (position - vec2(width, height) / 2.0) / scale + vec2(VIEW_WIDTH, VIEW_HEIGHT) / 2.0
}

fn hit_test(phase: Phase, position: Vec2) -> Option<Action> {
    if RESTART_BUTTON.contains(position) {
        Some(Action::Restart)
    } else if PAUSE_BUTTON.contains(position) && matches!(phase, Phase::Playing | Phase::Paused) {
        Some(Action::TogglePause)
    } else if phase != Phase::Playing && PRIMARY_BUTTON.contains(position) {
        Some(if phase == Phase::Paused {
            Action::TogglePause
        } else {
            Action::Restart
        })
    } else {
        None
    }
}

fn cell_center(cell: Cell) -> Vec2 {
    vec2(
        BOARD.x + (cell.0 as f32 + 0.5) * CELL_SIZE,
        BOARD.y + (cell.1 as f32 + 0.5) * CELL_SIZE,
    )
}

fn rounded_rect(bounds: Rect, radius: f32, color: Color) {
    let center = bounds.center();
    let first = vec2(bounds.x, bounds.y + radius);
    let mut previous = first;
    for (corner, start_angle) in [
        (vec2(bounds.x + radius, bounds.y + radius), 180.0_f32),
        (vec2(bounds.x + bounds.w - radius, bounds.y + radius), 270.0),
        (
            vec2(bounds.x + bounds.w - radius, bounds.y + bounds.h - radius),
            0.0,
        ),
        (vec2(bounds.x + radius, bounds.y + bounds.h - radius), 90.0),
    ] {
        for step in 0..=8 {
            let angle = (start_angle + step as f32 * 90.0 / 8.0).to_radians();
            let next = corner + vec2(angle.cos(), angle.sin()) * radius;
            draw_triangle(center, previous, next, color);
            previous = next;
        }
    }
    draw_triangle(center, previous, first, color);
}

fn play_icon(center: Vec2, color: Color) {
    draw_triangle(
        center + vec2(-4.0, -7.0),
        center + vec2(-4.0, 7.0),
        center + vec2(6.0, 0.0),
        color,
    );
}

fn draw_eyes(head: Vec2, direction: Direction) {
    let (horizontal, vertical) = direction.offset();
    let forward = vec2(horizontal as f32, vertical as f32);
    let sideways = vec2(-forward.y, forward.x);
    for side in [-1.0, 1.0] {
        let eye = head + forward * 3.5 + sideways * side * 3.2;
        draw_circle(eye.x, eye.y, 2.2, INK);
        draw_circle(
            eye.x + forward.x * 0.6,
            eye.y + forward.y * 0.6,
            1.05,
            SURFACE,
        );
    }
}

fn draw_apple(center: Vec2, now: f64) {
    let radius = 7.0 + (now as f32 * 3.0).sin() * 0.35;
    draw_circle(
        center.x,
        center.y + 2.0,
        radius + 0.5,
        Color::new(0.0, 0.0, 0.0, 0.18),
    );
    draw_circle(center.x, center.y, radius, APPLE);
    draw_ellipse(center.x + 3.0, center.y - 7.0, 3.0, 1.6, -35.0, ACCENT);
    draw_circle(
        center.x - 2.3,
        center.y - 2.3,
        1.6,
        Color::new(1.0, 0.87, 0.87, 0.75),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primary_action_matches_phase() {
        let center = PRIMARY_BUTTON.center();
        assert_eq!(hit_test(Phase::Ready, center), Some(Action::Restart));
        assert_eq!(hit_test(Phase::Paused, center), Some(Action::TogglePause));
        assert_eq!(hit_test(Phase::GameOver, center), Some(Action::Restart));
        assert_eq!(hit_test(Phase::Playing, center), None);
    }

    #[test]
    fn pause_is_disabled_outside_a_round() {
        assert_eq!(hit_test(Phase::Ready, PAUSE_BUTTON.center()), None);
        assert_eq!(hit_test(Phase::GameOver, PAUSE_BUTTON.center()), None);
        assert_eq!(
            hit_test(Phase::Playing, PAUSE_BUTTON.center()),
            Some(Action::TogglePause)
        );
    }

    #[test]
    fn pointer_mapping_accounts_for_letterboxing() {
        for (width, height) in [(1280.0, 800.0), (390.0, 844.0), (844.0, 390.0)] {
            let scale = layout_scale(width, height);
            let expected = PRIMARY_BUTTON.center();
            let screen = (expected - vec2(VIEW_WIDTH, VIEW_HEIGHT) / 2.0) * scale
                + vec2(width, height) / 2.0;
            assert!(to_layout(screen, width, height).distance(expected) < 0.001);
        }
    }
}
