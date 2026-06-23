use std::{cell::RefCell, rc::Rc};

use macroquad::{color, math::vec2, texture, window};
use rand::seq::SliceRandom;

use crate::{ables::{drawable::Drawable, updatable::Updatable}, inside_game::{brush::Brush, pixel::{MaterialType, Pixel}}};

const HEIGHT: usize = 720 / 4;
const WIDTH: usize = HEIGHT / 9 * 16;

pub struct Grid {
    buffer: Vec<u8>,
    cells: Vec<Pixel>,
    texture: texture::Texture2D,
    brush: Rc<RefCell<Brush>>,
    shuffle_order: Vec<usize>,
    random_generator: rand::rngs::ThreadRng
}

impl Grid {
    pub fn new(brush: Rc<RefCell<Brush>>) -> Grid {
        let mut grid = vec!();
        let mut buffer = vec!();

        for _ in 0..HEIGHT * WIDTH {
            grid.push(Pixel::new(MaterialType::Air));
            let l = grid.len();
            buffer.push((grid[l - 1].color.r * 255.) as u8);
            buffer.push((grid[l - 1].color.g * 255.) as u8);
            buffer.push((grid[l - 1].color.b * 255.) as u8);
            buffer.push(255); // Alpha
        }

        let texture = texture::Texture2D::from_rgba8(WIDTH as u16, HEIGHT as u16, &buffer);
        texture.set_filter(texture::FilterMode::Nearest);

        Grid {
            buffer,
            cells : grid,
            texture,
            brush,
            shuffle_order: (0..WIDTH).collect(),
            random_generator: rand::rng()
        }
    }

    fn pixel_update(&mut self, row: usize, column: usize) {
        let index = row * WIDTH + column;
        if !matches!(self.cells[index].material, MaterialType::Sand) {
            self.cells[index].sleeps = false;
            return;
        }
        if row + 1 >= HEIGHT {
            return;
        }
        if self.cells[index].sleeps {
            return;
        }

        let down = (row + 1) * WIDTH + column;
        if matches!(self.cells[down].material, MaterialType::Air) {
            self.swap_pixels(index, down);
            self.wake_up_under_pixels(row, column);
            return;
        }

        let diag_right = (column + 1 < WIDTH && matches!(self.cells[(row + 1) * WIDTH + column + 1].material, MaterialType::Air))
        .then(|| (row + 1) * WIDTH + column + 1);
        let diag_left  = (column > 0 && matches!(self.cells[(row + 1) * WIDTH + column - 1].material, MaterialType::Air))
        .then(|| (row + 1) * WIDTH + column - 1);

        let right = rand::random_bool(0.5);

        let candidates = if right { [diag_right, diag_left] } else { [diag_left, diag_right] };
        for diag in candidates.into_iter().flatten() {
            self.swap_pixels(index, diag);
            self.wake_up_under_pixels(row, column);
            return;
        }

        self.cells[index].sleeps = true;
    }

    fn swap_pixels(&mut self, index1: usize, index2: usize) {
        self.cells.swap(index1, index2);
        for i in 0..4 {
            self.buffer.swap(index1 * 4 + i, index2 * 4 + i);
        }
    }

    fn set_pixel(&mut self, row: usize, column: usize, material: MaterialType) {
        let index = row * WIDTH + column;
        if self.cells[index].material >= material {
            return;
        }
        self.cells[index] = Pixel::new(material);
        let colors = self.cells[index].color.to_vec();
        for i in 0..4 {
            self.buffer[index * 4 + i] = (colors[i] * 255.) as u8;
        }
    }

    fn erase_pixel(&mut self, row: usize, column: usize) {
        let index = row * WIDTH + column;
        if matches!(self.cells[index].material, MaterialType::Air) {
            return;
        }
        self.cells[index] = Pixel::new(MaterialType::Air);
        let colors = self.cells[index].color.to_vec();
        for i in 0..4 {
            self.buffer[index * 4 + i] = (colors[i] * 255.) as u8;
        }
        self.wake_up_under_pixels(row, column);
    }

    fn brush_update(&mut self, material: MaterialType, _erase: bool) {
        let column = self.brush.borrow().x * WIDTH as isize / window::screen_width() as isize;
        if column < 0 || column >= WIDTH as isize {
            return;
        }
        let column = column as usize;

        let row = self.brush.borrow().y * HEIGHT as isize / window::screen_height() as isize;
        if row < 0 || row >= HEIGHT as isize {
            return;
        }
        let row = row as usize;
        
        let radius = self.brush.borrow().radius as usize;

            if radius == 1 {
                if !_erase {
                    self.set_pixel(row, column, material.clone());
                }
                else {
                    self.erase_pixel(row, column);
                }
            return;
        }

        for r in 0..radius {
            let width = radius - r - 1;
            let left = if column > width { column - width } else { 0 };
            let right = if column + width < WIDTH { column + width } else { WIDTH - 1 };
            let up = if row > r { row - r } else { 0 };
            let down = if row + r < HEIGHT { row + r } else { HEIGHT - 1 };
            for c in left..=right {
                for local_row in up..=down {
                    if !_erase {
                        self.set_pixel(local_row, c, material.clone());
                    }
                    else {
                        self.erase_pixel(local_row, c);
                    }
                }
            }
        }
        
        // self.set_pixel(row, column, material);
    }

    fn wake_up_under_pixels(&mut self, row: usize, column: usize) {
        if row > 0 {
            self.cells[(row - 1) * WIDTH + column].sleeps = false;
            if column <= 0 {
                self.cells[(row - 1) * WIDTH + column + 1].sleeps = false;
            }
            else if column + 1 >= WIDTH {
                self.cells[(row - 1) * WIDTH + column - 1].sleeps = false;
            }
            else {
                self.cells[(row - 1) * WIDTH + column - 1].sleeps = false;
                self.cells[(row - 1) * WIDTH + column + 1].sleeps = false;
            }
        }
    }
}

impl Updatable for Grid {
    fn update(&mut self) {
        for row in (0..HEIGHT).rev() {  
            self.shuffle_order.shuffle(&mut self.random_generator);
            for column in 0..WIDTH {
                self.pixel_update(row, self.shuffle_order[column]);
            }
        }

        if self.brush.borrow().need_update {
            let material = self.brush.borrow().material.clone();
            self.brush_update(material, false);
        }
        else if self.brush.borrow().need_erase {
            self.brush_update(MaterialType::Air, true);
        }
    }
}

impl Drawable for Grid {
    fn draw(&self) {
        self.texture.update_from_bytes(WIDTH as u32, HEIGHT as u32, &self.buffer);
        texture::draw_texture_ex(
            &self.texture, 
            0.0, 
            0.0, 
            color::WHITE, 
            texture::DrawTextureParams {
                dest_size: Some(vec2(window::screen_width(), window::screen_height())),
                ..Default::default()
            });
    }
}
