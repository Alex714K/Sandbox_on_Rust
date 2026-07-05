use macroquad::input::{KeyCode, MouseButton, is_key_down, is_mouse_button_down, mouse_position};
use macroquad::math::{Vec4, vec4};

use crate::ables::updatable::Updatable;
use crate::inside_game::pixel::MaterialType;
use strum::IntoEnumIterator;
use crate::inside_game::button_manager::{BUTTON_HEIGHT, BUTTON_WIDTH, UNDER_GAP, DISTANCE_BETWEEN_BUTTONS_OF_ELEMENTS};

pub struct Brush {
    pub material: MaterialType,
    pub radius: u8,
    pub need_update: bool,
    pub need_erase: bool,
    pub x: isize,
    pub y: isize,
    buttons_coords: Vec<Vec4>
}

impl Brush {
    pub fn new(material: MaterialType, radius: u8) -> Brush {
        Brush { material, radius, need_update: false, need_erase: false, x: 0, y: 0, buttons_coords: Brush::calculate_buttons_coords() }
    }

    fn calculate_buttons_coords() -> Vec<Vec4> {
        let mut coords = vec!();
        for (i, _) in MaterialType::iter().enumerate() {
            coords.push(vec4(
                DISTANCE_BETWEEN_BUTTONS_OF_ELEMENTS + (BUTTON_WIDTH + DISTANCE_BETWEEN_BUTTONS_OF_ELEMENTS) * i as f32,
                UNDER_GAP,
                (DISTANCE_BETWEEN_BUTTONS_OF_ELEMENTS + BUTTON_WIDTH) * (i + 1) as f32,
                UNDER_GAP + BUTTON_HEIGHT
            ));
        }
        return coords;
    }

    fn is_over_button(&self, x: f32, y: f32) -> bool {
        for coords in &self.buttons_coords {
            if x >= coords.x && x <= coords.z && y >= coords.y && y <= coords.w {
                return true;
            }
        }
        return false;
    }

    fn mouse_update(&mut self) {
        let (x, y) = mouse_position();

        if self.is_over_button(x, y) {
            self.need_update = false;
            self.need_erase = false;
            return;
        }
        
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
