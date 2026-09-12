use macroquad::prelude::*;

use crate::game::{Direction, Game, Phase};
use crate::ui::{Action, Ui};

pub fn handle(game: &mut Game, ui: &Ui, now: f64) {
    let action = if is_key_pressed(KeyCode::Space)
        || (game.phase == Phase::Ready && is_key_pressed(KeyCode::Enter))
    {
        Some(Action::Restart)
    } else if is_key_pressed(KeyCode::P) {
        Some(Action::TogglePause)
    } else if is_mouse_button_pressed(MouseButton::Left) {
        ui.action_at(game.phase, Vec2::from(mouse_position()))
    } else {
        None
    };

    match action {
        Some(Action::Restart) => game.restart(now),
        Some(Action::TogglePause) => game.toggle_pause(now),
        None => {}
    }

    let direction = if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
        Some(Direction::Up)
    } else if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
        Some(Direction::Down)
    } else if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
        Some(Direction::Left)
    } else if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
        Some(Direction::Right)
    } else {
        None
    };
    if let Some(direction) = direction {
        game.turn(direction);
    }
}
