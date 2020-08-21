mod Grid;

extern crate minifb;

use minifb::{Key, Window, WindowOptions, MouseMode, MouseButton};
use std::time::Duration;
use std::thread::sleep;


const CELL_SIZE: usize = 16;
const GRID_SIZE: usize = 40;

const WIDTH: usize = CELL_SIZE * GRID_SIZE;
const HEIGHT: usize = CELL_SIZE * GRID_SIZE;

const CELL_WIDTH: usize = WIDTH / CELL_SIZE;
const CELL_HEIGHT: usize = HEIGHT / CELL_SIZE;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut grid = Grid::Grid {
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