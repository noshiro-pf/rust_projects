mod utils;

use wasm_bindgen::prelude::*;
// use std::fmt;
// use rand::prelude::*;
use bitvec::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub type Cell = bool;

enum CurrentCells {
    Cell0 = 0,
    Cell1 = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells_0: BitVec,
    cells_1: BitVec,
    current_cells_index: CurrentCells,
}

fn initialize_universe_base(width: u32, height: u32, cells_0: BitVec) -> Universe {
    let cells_1: BitVec = (0..width * height).map(|_| false).collect();

    Universe {
        width,
        height,
        cells_0,
        cells_1,
        current_cells_index: CurrentCells::Cell0,
    }
}

fn create_default_universe() -> Universe {
    let width = 64;
    let height = 64;

    let cells_0: BitVec = (0..width * height)
        .map(|i| i % 2 == 0 || i % 7 == 0)
        .collect();

    initialize_universe_base(width, height, cells_0)
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

    let cells_0: BitVec = (0..width * height)
        .map(|i| {
            let row: u32 = i / width;
            let col: u32 = i % width;
            let (a, b) = (row - offset_top, col - offset_left);
            false
                || (a, b) == (0, 1)
                || (a, b) == (0, 4)
                || (a, b) == (1, 0)
                || (a, b) == (2, 0)
                || (a, b) == (2, 4)
                || (a, b) == (3, 0)
                || (a, b) == (3, 1)
                || (a, b) == (3, 2)
                || (a, b) == (3, 3)
        })
        .collect();

    initialize_universe_base(width, height, cells_0)
}

fn create_random_universe() -> Universe {
    let width = 64;
    let height = 64;

    let cells_0: BitVec = (0..width * height)
        .map(|_| js_sys::Math::random() >= 0.5)
        .collect();

    initialize_universe_base(width, height, cells_0)
}

#[wasm_bindgen]
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn get_cell(&self, row: u32, column: u32) -> Cell {
        let row_as_torus = (self.height + row) % self.height;
        let col_as_torus = (self.width + column) % self.width;

        self.current_cells()[self.get_index(row_as_torus, col_as_torus)]
    }

    fn current_cells(&self) -> &BitVec {
        match self.current_cells_index {
            CurrentCells::Cell0 => &self.cells_0,
            CurrentCells::Cell1 => &self.cells_1,
        }
    }

    fn next_cells(&mut self) -> &mut BitVec {
        match self.current_cells_index {
            CurrentCells::Cell0 => &mut self.cells_1,
            CurrentCells::Cell1 => &mut self.cells_0,
        }
    }

    fn set_next_cell(&mut self, idx: usize, value: Cell) {
        self.next_cells().set(idx, value);
    }

    fn increment_current_cells_index(&mut self) {
        self.current_cells_index = match self.current_cells_index {
            CurrentCells::Cell0 => CurrentCells::Cell1,
            CurrentCells::Cell1 => CurrentCells::Cell0,
        }
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
                true => 1,
                false => 0,
            }
        })
    }

    pub fn tick(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.current_cells()[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (true, x) if x < 2 => false,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (true, 2) | (true, 3) => true,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (true, x) if x > 3 => false,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (false, 3) => true,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                self.set_next_cell(idx, next_cell);
            }
        }

        self.increment_current_cells_index();
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

    pub fn cells(&self) -> *const usize {
        self.current_cells().as_raw_slice().as_ptr()
    }
}
