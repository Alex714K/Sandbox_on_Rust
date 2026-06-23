use std::cell::RefCell;
use std::rc::Rc;

use crate::ables;
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
    }
}