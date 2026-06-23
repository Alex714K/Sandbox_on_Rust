use macroquad::input::{KeyCode, MouseButton, is_key_down, is_mouse_button_down, mouse_position};

use crate::{ables::updatable::Updatable, inside_game::pixel::MaterialType};

pub struct Brush {
    pub material: MaterialType,
    pub radius: u8,
    pub need_update: bool,
    pub need_erase: bool,
    pub x: isize,
    pub y: isize
}

impl Brush {
    pub fn new(material: MaterialType, radius: u8) -> Brush {
        Brush { material, radius, need_update: false, need_erase: false, x: 0, y: 0 }
    }

    fn mouse_update(&mut self) {
        let (x, y) = mouse_position();
        
        if x < 0.0 || y < 0.0 {
            self.need_update = false;
            self.need_erase = false;
            return;
        }

        if is_mouse_button_down(MouseButton::Left) {
            self.x = x as isize;
            self.y = y as isize;
            self.need_update = true;
        }
        else {
            self.need_update = false;
        }

        if is_mouse_button_down(MouseButton::Right) {
            self.x = x as isize;
            self.y = y as isize;
            self.need_erase = true;
        }
        else {
            self.need_erase = false;
        }
    }
}

impl Updatable for Brush {
    fn update(&mut self) {
        self.mouse_update();

        if is_key_down(KeyCode::Key1) {
            self.radius = 1;
        }
        else if is_key_down(KeyCode::Key2) {
            self.radius = 2;
        }
        else if is_key_down(KeyCode::Key3) {
            self.radius = 3;
        }
        else if is_key_down(KeyCode::Key4) {
            self.radius = 75
        }
    }
}
