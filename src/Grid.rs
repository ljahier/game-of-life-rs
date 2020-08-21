use std::mem;

const GRID_SIZE: usize = 40;

pub struct _Grid {
    pub front_buff: Vec<bool>,
    pub back_buff: Vec<bool>,
}

impl _Grid {
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

    fn get_cell_value(&self, x: usize, y: usize) -> bool {
        if !_Grid::is_in_bounds(x, y) {
            return false;
        }
        self.front_buff[y * GRID_SIZE + x]
    }

    fn is_in_bounds(x: usize, y: usize) -> bool {
        x < GRID_SIZE && y < GRID_SIZE
    }

    fn swap_buffers(&mut self) {
        mem::swap(&mut self.front_buff, &mut self.back_buff)
    }

    pub fn update(&mut self) {
        for (index, _) in self.front_buff.iter().enumerate() {
            let mut x: usize = 0;
            let mut y: usize = 0;
            _Grid::get_coordinates(index, &mut x, &mut y);
            self.back_buff[index] = match self.compute_neighbour(x, y) {
                2 => self.front_buff[index],
                3 => true,
                _ => false
            }
        }
        self.swap_buffers()
    }
}