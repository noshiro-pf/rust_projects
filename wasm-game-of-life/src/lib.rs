mod utils;

use wasm_bindgen::prelude::*;
// use std::fmt;
// use rand::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

fn create_default_universe() -> Universe {
    let width = 64;
    let height = 64;
    let cells = (0..width * height)
        .map(|i| {
            if i % 2 == 0 || i % 7 == 0 {
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

/**
 * space ship:
 *
 *     _o__o
 *     o____
 *     o___o
 *     oooo_
 */
fn create_single_space_ship_universe() -> Universe {
    let width = 64;
    let height = 64;
    let (offset_left, offset_top) = (30, 30);

    let cells = (0..width * height)
        .map(|i| {
            let row: u32 = i / width;
            let col: u32 = i % width;
            match (row - offset_top, col - offset_left) {
                (0, 1) => Cell::Alive,
                (0, 4) => Cell::Alive,
                (1, 0) => Cell::Alive,
                (2, 0) => Cell::Alive,
                (2, 4) => Cell::Alive,
                (3, 0) => Cell::Alive,
                (3, 1) => Cell::Alive,
                (3, 2) => Cell::Alive,
                (3, 3) => Cell::Alive,
                _ => Cell::Dead,
            }
        })
        .collect();

    Universe {
        width,
        height,
        cells,
    }
}

fn create_random_universe() -> Universe {
    let width = 64;
    let height = 64;

    let cells = (0..width * height)
        .map(|_| {
            if js_sys::Math::random() >= 0.5 {
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

#[wasm_bindgen]
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn get_cell(&self, row: u32, column: u32) -> Cell {
        let row_as_torus = (self.height + row) % self.height;
        let col_as_torus = (self.width + column) % self.width;

        self.cells[self.get_index(row_as_torus, col_as_torus)]
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        /*
         * neighbor cells
         *
         *  +---+---+---+
         *  | * | * | * |
         *  +---+---+---+
         *  | * |   | * |
         *  +---+---+---+
         *  | * | * | * |
         *  +---+---+---+
         */
        [
            // メモ： row, column を入れないと -1 が u32 にならないので却って面倒になる
            (row - 1, column - 1),
            (row - 1, column + 0),
            (row - 1, column + 1),
            (row + 0, column + 1),
            (row + 0, column - 1),
            (row + 1, column - 1),
            (row + 1, column + 0),
            (row + 1, column + 1),
        ]
        .iter()
        .fold(0, |sum, (r, c)| {
            sum + match self.get_cell(*r, *c) {
                Cell::Alive => 1,
                Cell::Dead => 0,
            }
        })
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        // create_default_universe()
        create_single_space_ship_universe()
        // create_random_universe()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}
