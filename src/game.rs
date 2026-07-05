use std::cell::RefCell;
use std::rc::Rc;

use macroquad::color::WHITE;
use macroquad::text::draw_text;

use crate::{SCREEN_WIDTH, ables};
use crate::inside_game::button_manager::ButtonManager;
use crate::inside_game::pixel::MaterialType;
use crate::inside_game::{grid, brush::Brush};

pub struct Game {
    brush: Rc<RefCell<Brush>>,
    grid: grid::Grid,
    button_manager: ButtonManager
}

impl Game {
    pub fn new() -> Game {
        let brush = Rc::new(RefCell::new(Brush::new(MaterialType::Air, 1)));
        Game {
            grid : grid::Grid::new(brush.clone()),
            button_manager : ButtonManager::new(brush.clone()),
            brush,
        }
    }

    fn draw_description(&self) {
        let text_start = SCREEN_WIDTH - 240.0;
        draw_text("1-4 - brush sizes", text_start, 24.0 * 1.0, 24.0, WHITE);
        draw_text("Shift - brush size * 2", text_start, 24.0 * 2.0, 24.0, WHITE);
        draw_text("LMB - place material", text_start, 24.0 * 3.0, 24.0, WHITE);
        draw_text("RMB - clear", text_start, 24.0 * 4.0, 24.0, WHITE);
    }
}

impl ables::updatable::Updatable for Game {
    fn update(&mut self) {
        self.brush.borrow_mut().update();
        self.grid.update();
    }
}

impl ables::drawable::Drawable for Game {
    fn draw(&self) {
        self.button_manager.draw();
        self.grid.draw();
        self.draw_description();
    }
}