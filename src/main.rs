#![windows_subsystem = "windows"]
mod game;
mod ables;
pub mod inside_game;

// use macroquad::prelude::*;
use crate::ables::{drawable::Drawable, updatable::Updatable};

fn window_conf() -> macroquad::prelude::Conf {
    macroquad::prelude::Conf {
        window_title: "Sandbox".to_owned(),
        high_dpi: false,
        window_resizable: true,
        platform: macroquad::miniquad::conf::Platform {
            // swap_interval: Some(0),
            ..Default::default()
        },
        ..Default::default()
    }
}

const TICK_RATE: f32 = 120.0;
const TICK_DURATION: f32 = 1.0 / TICK_RATE;

#[macroquad::main(window_conf)]
async fn main() {
    let height = 720.0;
    let width = height / 9.0 * 16.0;
    macroquad::window::request_new_screen_size(width, height);
    let mut game = game::Game::new();    

    let mut accumulator = 0.0;
    
    loop {        
        accumulator += macroquad::time::get_frame_time().min(TICK_DURATION * 2.0);

        let mut steps = 0;
        while accumulator >= TICK_DURATION && steps < 5 {
            game.update();
            accumulator -= TICK_DURATION;
            steps += 1;
        }

        game.draw();        
        
        macroquad::time::draw_fps();
        macroquad::prelude::next_frame().await;
    }
}