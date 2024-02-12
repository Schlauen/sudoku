use wasm_bindgen::prelude::*;
use array2d::Array2D;
use bitvec::prelude::*;
use rand::prelude::*;

const FIELDS:[(u8, u8); 81] = [
    (0,0), (0,1), (0,2), (0,3), (0,4), (0,5), (0,6), (0,7), (0,8),
    (1,0), (1,1), (1,2), (1,3), (1,4), (1,5), (1,6), (1,7), (1,8),
    (2,0), (2,1), (2,2), (2,3), (2,4), (2,5), (2,6), (2,7), (2,8),
    (3,0), (3,1), (3,2), (3,3), (3,4), (3,5), (3,6), (3,7), (3,8),
    (4,0), (4,1), (4,2), (4,3), (4,4), (4,5), (4,6), (4,7), (4,8),
    (5,0), (5,1), (5,2), (5,3), (5,4), (5,5), (5,6), (5,7), (5,8),
    (6,0), (6,1), (6,2), (6,3), (6,4), (6,5), (6,6), (6,7), (6,8),
    (7,0), (7,1), (7,2), (7,3), (7,4), (7,5), (7,6), (7,7), (7,8),
    (8,0), (8,1), (8,2), (8,3), (8,4), (8,5), (8,6), (8,7), (8,8),
];
const QUADS:[[u8;9];9] = [
    [0,0,0,1,1,1,2,2,2,],
    [0,0,0,1,1,1,2,2,2,],
    [0,0,0,1,1,1,2,2,2,],
    [3,3,3,4,4,4,5,5,5,],
    [3,3,3,4,4,4,5,5,5,],
    [3,3,3,4,4,4,5,5,5,],
    [6,6,6,7,7,7,8,8,8,],
    [6,6,6,7,7,7,8,8,8,],
    [6,6,6,7,7,7,8,8,8,],
];
const VALUES_BIN:[u16;9] = [1,2,4,8,16,32,64,128,256];
const VALUES_BIN_INV:[u16;9] = [
    0b1111111111111110,
    0b1111111111111101,
    0b1111111111111011,
    0b1111111111110111,
    0b1111111111101111,
    0b1111111111011111,
    0b1111111110111111,
    0b1111111101111111,
    0b1111111011111111,
];

// if changed, apply 'wasm-pack build'
#[wasm_bindgen]
pub struct PlayfieldState {
    values: Array2D<u8>,
    errors: Array2D<bool>,
    solution: Option<Array2D<u8>>,
    fixed: Array2D<bool>,
    poss_rows: [u16; 9],
    poss_cols: [u16; 9],
    poss_quads: [u16; 9],
    solved: bool,
    show_errors: bool,
    status_text: String,
    difficulty: f32,
}

#[wasm_bindgen]
impl PlayfieldState {
    pub fn new() -> PlayfieldState {
        PlayfieldState { 
            values: Array2D::filled_with(0, 9, 9),
            errors: Array2D::filled_with(false, 9, 9),
            solution: Option::None,
            fixed: Array2D::filled_with(false, 9, 9),
            poss_rows: [0b1111111111111111u16; 9],
            poss_cols: [0b1111111111111111u16; 9],
            poss_quads: [0b1111111111111111u16; 9],
            solved: false,
            show_errors: false,
            status_text: format!(""),
            difficulty: 40.0,
        }
    }

    pub fn set_value(&mut self, value:u8, row:usize, col:usize) {
        self.values[(row, col)] = value;
    }

    pub fn get_value(&self, row:usize, col:usize) -> u8 {
        self.values[(row, col)]
    }
}
