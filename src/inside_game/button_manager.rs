use std::{cell::RefCell, rc::Rc};

use macroquad::{color::{BLACK, GREEN, WHITE}, math::vec2, ui::{Skin, root_ui, widgets::Button}};
use strum::IntoEnumIterator;

use crate::{ables::drawable::Drawable, inside_game::{brush::Brush, pixel::{MaterialType, Pixel}}};

pub const UNDER_GAP: f32 = 10.0;
pub const DISTANCE_BETWEEN_BUTTONS_OF_ELEMENTS: f32 = 10.0;

pub const BUTTON_WIDTH: f32 = 100.0;
pub const BUTTON_HEIGHT: f32 = 30.0;

pub struct ButtonManager {
    brush: Rc<RefCell<Brush>>
}

impl ButtonManager {
    pub fn new(brush: Rc<RefCell<Brush>>) -> ButtonManager {
        ButtonManager { 
            brush
        }
    }
}

impl Drawable for ButtonManager {
    fn draw(&self) {
        for (i, material) in MaterialType::iter().enumerate() {
            if i == 0 {
                continue;
            }
            let color = Pixel::button_color(&material);
            let text_color = if color.r + color.g + color.b < 1.5 { WHITE } else { BLACK };
            let button_style = root_ui()
                .style_builder()
                .color(color)
                .color_hovered(BLACK)
                .color_clicked(GREEN)
                .text_color(text_color)
                .text_color_clicked(WHITE)
                .text_color_hovered(WHITE)
                .font_size(30)
                .build();
            let skin = Skin {
                button_style,
                ..root_ui().default_skin()
            };
            root_ui().push_skin(&skin);
            
            let clicked = Button::new(material.to_string())
                .position(vec2(10.0 + i as f32 * (DISTANCE_BETWEEN_BUTTONS_OF_ELEMENTS + BUTTON_WIDTH) as f32, 10.0))
                .size(vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
                .ui(&mut root_ui());
            if clicked {
                self.brush.borrow_mut().material = material;
            }
            root_ui().pop_skin();
        }
    }
}
