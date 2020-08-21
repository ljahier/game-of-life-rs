extern crate minifb;

use minifb::{Key, Window, WindowOptions, MouseMode, MouseButton};
use std::mem;
use std::time::Duration;
use std::thread::sleep;


const CELL_SIZE: usize = 16;
const GRID_SIZE: usize = 40;

const WIDTH: usize = CELL_SIZE * GRID_SIZE;
const HEIGHT: usize = CELL_SIZE * GRID_SIZE;

const CELL_WIDTH: usize = WIDTH / CELL_SIZE;
const CELL_HEIGHT: usize = HEIGHT / CELL_SIZE;

struct Grid {
    front_buff: Vec<bool>,
    back_buff: Vec<bool>
}

impl Grid {
    fn compute_neighbour(&self, x: usize, y: usize) -> u32 {
        let mut count: u32 = 0;

        if self.get_cell_value(x.wrapping_sub(1), y.wrapping_sub(1)) { count += 1; }
        if self.get_cell_value(x.wrapping_add(0), y.wrapping_sub(1)) { count += 1; }
        if self.get_cell_value(x.wrapping_add(1), y.wrapping_sub(1)) { count += 1; }
        if self.get_cell_value(x.wrapping_sub(1), y.wrapping_add(0)) { count += 1; }
        if self.get_cell_value(x.wrapping_add(1), y.wrapping_add(0)) { count += 1; }
        if self.get_cell_value(x.wrapping_sub(1), y.wrapping_add(1)) { count += 1; }
        if self.get_cell_value(x.wrapping_add(0), y.wrapping_add(1)) { count += 1; }
        if self.get_cell_value(x.wrapping_add(1), y.wrapping_add(1)) { count += 1; }
        count
    }

    fn get_coordinates(value: usize, x: &mut usize, y: &mut usize) {
        *x = value % GRID_SIZE;
        *y = value / GRID_SIZE;
    }

    fn get_cell_value(&self, x: usize, y:usize) -> bool {
        if !Grid::is_in_bounds(x, y) {
            return false
        }
        self.front_buff[y * GRID_SIZE + x]
    }

    fn is_in_bounds(x: usize, y: usize) -> bool {
        x < GRID_SIZE && y < GRID_SIZE
    }

    fn swap_buffers(&mut self) {
        mem::swap(&mut self.front_buff, &mut self.back_buff)
    }

    fn update(&mut self) {
        for (index, _) in self.front_buff.iter().enumerate() {
            let mut x: usize = 0;
            let mut y: usize = 0;
            Grid::get_coordinates(index, &mut x, &mut y);
            self.back_buff[index] = match self.compute_neighbour(x, y) {
                2 => self.front_buff[index],
                3 => true,
                _ => false
            }
        }
        self.swap_buffers()
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut grid = Grid {
        front_buff: vec![false; CELL_WIDTH * CELL_HEIGHT],
        back_buff: vec![false; CELL_WIDTH * CELL_HEIGHT]
    };

    /*grid.front_buff[3 + GRID_SIZE + 2] = true;
    grid.front_buff[3 + GRID_SIZE + 3] = true;
    grid.front_buff[3 + GRID_SIZE + 4] = true;*/

    grid.update();

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.get_mouse_pos(MouseMode::Discard).map(|mouse| {
            let cell_x = mouse.0 as usize / CELL_SIZE;
            let cell_y = mouse.1 as usize / CELL_SIZE;

            if window.get_mouse_down(MouseButton::Left) {
                grid.front_buff[cell_y * CELL_WIDTH + cell_x] = true
            } else if window.get_mouse_down(MouseButton::Right) {
                grid.front_buff[cell_y * CELL_WIDTH + cell_x] = false
            }
        });
        if window.is_key_down(Key::Space) {
            sleep(Duration::new(0, 50000000));
            grid.update();
        }
        for (index, cell) in buffer.iter_mut().enumerate() {
            let x = index % WIDTH;
            let y = index / WIDTH;

            let cell_x = x / CELL_SIZE;
            let cell_y = y / CELL_SIZE;

            *cell = if grid.front_buff[cell_y * CELL_WIDTH + cell_x] { 0xFFFFFF } else { 0x000000 };
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}