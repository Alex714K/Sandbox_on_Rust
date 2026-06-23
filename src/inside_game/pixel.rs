use std::fmt;
use macroquad::{color::{self, Color}, color_u8};
use rand::{self, random_range};
use strum_macros::{EnumCount, EnumIter};

pub struct Pixel {
    pub material: MaterialType,
    pub color: Color,
    pub sleeps: bool
}

impl Pixel {
    pub fn new(material: MaterialType) -> Pixel {
        Pixel { color: Pixel::choose_color(&material), material: material, sleeps: false }
    }


    pub fn choose_color(material: &MaterialType) -> Color {
        match material {
            MaterialType::Air => color::BLACK,
            MaterialType::Sand => {
                let blue = random_range(0.0..50.0);
                color_u8!(220.0 - blue / 2.0, 200.0 - blue / 2.0, blue, 255.0)
            },
            MaterialType::Stone => color_u8!(100, 100, 100, 255),
        }
    }

    pub fn button_color(material: &MaterialType) -> Color {
        match material {
            MaterialType::Air => color::WHITE,
            MaterialType::Sand => color_u8!(220, 200, 0, 255),
            MaterialType::Stone => color_u8!(100, 100, 100, 255),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, EnumCount, EnumIter)]
pub enum MaterialType {
    Air,
    Sand,
    Stone
}

impl fmt::Display for MaterialType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MaterialType::Air => "Air",
            MaterialType::Sand => "Sand",
            MaterialType::Stone => "Stone",
        };
        write!(f, "{}", s)
    }
}

impl Clone for MaterialType {
    fn clone(&self) -> Self {
        match self {
            Self::Air => Self::Air,
            Self::Sand => Self::Sand,
            Self::Stone => Self::Stone,
        }
    }
}
