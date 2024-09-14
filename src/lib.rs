mod utils;
mod bitmap;

use std::fmt;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Universe {
    width: u8,
    height: u8,
    cells: bitmap::Bitmap,
}

#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> u8 {
        self.width
    }

    pub fn height(&self) -> u8 {
        self.height
    }

    pub fn cells(&self) -> *const bitmap::Bitset {
        self.cells.as_slice().as_ptr()
    }

    fn get_index(&self, row: u8, column: u8) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u8, column: u8) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neigbor_row = (row + delta_row) % self.height;
                let neigbor_col = (column + delta_col) % self.width;
                count += self.cells.get(neigbor_col, neigbor_row);
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let cell = self.cells.get(col, row);
                let live_neigbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neigbors) {
                    (1, x) if x < 2 => 0,
                    (1, 2) | (1, 3) => 1,
                    (1, x) if x > 3 => 0,
                    (0, 3) => 1,
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 9 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl Default for Universe {
    fn default() -> Self {
        Universe::new()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
