use macroquad::prelude::*;

mod game;
mod input;
mod ui;

use game::Game;
use ui::Ui;

#[macroquad::main("Snake")]
async fn main() {
    let mut game = Game::new(get_time());
    let ui = Ui::new();

    loop {
        let now = get_time();
        input::handle(&mut game, &ui, now);
        game.update(now);
        ui.draw(&game, now);
        next_frame().await;
    }
}
