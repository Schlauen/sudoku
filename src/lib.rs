use wasm_bindgen::prelude::*;
use array2d::Array2D;
use bitvec::{field, prelude::*};
use rand::prelude::*;

// (row, col, quad) triplets
const FIELDS:[((usize, usize), usize); 81] = [
    ((0,0),0), ((0,1),0), ((0,2),0), ((0,3),1), ((0,4),1), ((0,5),1), ((0,6),2), ((0,7),2), ((0,8),2),
    ((1,0),0), ((1,1),0), ((1,2),0), ((1,3),1), ((1,4),1), ((1,5),1), ((1,6),2), ((1,7),2), ((1,8),2),
    ((2,0),0), ((2,1),0), ((2,2),0), ((2,3),1), ((2,4),1), ((2,5),1), ((2,6),2), ((2,7),2), ((2,8),2),
    ((3,0),3), ((3,1),3), ((3,2),3), ((3,3),4), ((3,4),4), ((3,5),4), ((3,6),5), ((3,7),5), ((3,8),5),
    ((4,0),3), ((4,1),3), ((4,2),3), ((4,3),4), ((4,4),4), ((4,5),4), ((4,6),5), ((4,7),5), ((4,8),5),
    ((5,0),3), ((5,1),3), ((5,2),3), ((5,3),4), ((5,4),4), ((5,5),4), ((5,6),5), ((5,7),5), ((5,8),5),
    ((6,0),6), ((6,1),6), ((6,2),6), ((6,3),7), ((6,4),7), ((6,5),7), ((6,6),8), ((6,7),8), ((6,8),8),
    ((7,0),6), ((7,1),6), ((7,2),6), ((7,3),7), ((7,4),7), ((7,5),7), ((7,6),8), ((7,7),8), ((7,8),8),
    ((8,0),6), ((8,1),6), ((8,2),6), ((8,3),7), ((8,4),7), ((8,5),7), ((8,6),8), ((8,7),8), ((8,8),8),
];
// const FIELDS:[((usize, usize), usize); 81] = [
//     ((0,0),0), ((0,1),0), ((0,2),0), ((0,3),1), ((0,4),1), (0,5,1), (0,6,2), (0,7,2), (0,8,2),
//     (1,0,0), (1,1,0), (1,2,0), (1,3,1), (1,4,1), (1,5,1), (1,6,2), (1,7,2), (1,8,2),
//     (2,0,0), (2,1,0), (2,2,0), (2,3,1), (2,4,1), (2,5,1), (2,6,2), (2,7,2), (2,8,2),
//     (3,0,3), (3,1,3), (3,2,3), (3,3,4), (3,4,4), (3,5,4), (3,6,5), (3,7,5), (3,8,5),
//     (4,0,3), (4,1,3), (4,2,3), (4,3,4), (4,4,4), (4,5,4), (4,6,5), (4,7,5), (4,8,5),
//     (5,0,3), (5,1,3), (5,2,3), (5,3,4), (5,4,4), (5,5,4), (5,6,5), (5,7,5), (5,8,5),
//     (6,0,6), (6,1,6), (6,2,6), (6,3,7), (6,4,7), (6,5,7), (6,6,8), (6,7,8), (6,8,8),
//     (7,0,6), (7,1,6), (7,2,6), (7,3,7), (7,4,7), (7,5,7), (7,6,8), (7,7,8), (7,8,8),
//     (8,0,6), (8,1,6), (8,2,6), (8,3,7), (8,4,7), (8,5,7), (8,6,8), (8,7,8), (8,8,8),
// ];
const QUADS:[[usize;9];9] = [
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
        }
    }

    pub fn reset(&mut self) {
        self.values = Array2D::filled_with(0, 9, 9);
        self.errors = Array2D::filled_with(false, 9, 9);
        self.solution = Option::None;
        self.fixed = Array2D::filled_with(false, 9, 9);
        self.poss_rows = [0b1111111111111111u16; 9];
        self.poss_cols = [0b1111111111111111u16; 9];
        self.poss_quads = [0b1111111111111111u16; 9];
        self.solved = false;
        self.show_errors = false;
        self.status_text = format!("");
    }

    pub fn set_value(&mut self, value:u8, row:usize, col:usize) {
        let rc = (row, col);
        if self.fixed[rc] {
            return;
        }

        if value == 0 {
            self.reset_value(row, col);
            return;
        }

        let current_val = self.values[rc];
        if current_val > 0 {
            self.reset_value(row, col);
        }

        let mov_zero_based = (value - 1) as usize;
        match self.get_possible_moves(rc) {
            Some(moves) => {
                if moves.contains(&mov_zero_based) {
                    self.set_value_((rc, QUADS[row][col]), mov_zero_based);
                } else {
                    self.errors[rc] = true;
                    self.values[rc] = value;
                }
            },
            None => {
                self.errors[rc] = true;
                self.values[rc] = value;
            }
        }
    }

    pub fn get_value(&self, row:usize, col:usize) -> u8 {
        self.values[(row, col)]
    }

    pub fn is_error(&self, row:usize, col:usize) -> bool {
        let error = self.errors[(row, col)];
        match self.solution.as_ref() {
            None => error,
            Some(s) => {
                let val = self.values[(row, col)];
                if val == 0 {
                    error
                } else {
                    if s[(row, col)] == val {
                        error
                    } else {
                        true
                    }
                }
            }
        }
    }

    pub fn is_fix(&self, row:usize, col:usize) -> bool {
        self.fixed[(row, col)]
    }

    pub fn get_show_errors(&self) -> bool {
        self.show_errors
    }

    pub fn toggle_show_errors(&mut self) {
        self.show_errors = !self.show_errors; 
    }

    pub fn generate(&mut self, difficulty:u8) {
        self.reset();

        let mut values_random_mask: [u8; 9] = core::array::from_fn(|i| (i + 1) as u8);
        values_random_mask.shuffle(&mut thread_rng());

        let mut cursor_random_mask: [usize; 81] = [0; 81];
        for i in 0..81 {
            cursor_random_mask[i] = i;
        }
        cursor_random_mask.shuffle(&mut thread_rng());

        if !self.solve_random_(0) {
            panic!("No solution found");
        }
        let mut solution = self.values.clone();
        if !self.generate_(cursor_random_mask, 0, 0, difficulty) {
            panic!("No solution generated");
        }

        let values = self.values.clone();
        for i in 0..9 {
            for j in 0..9 {
                solution[(i,j)] = values_random_mask[(solution[(i,j)] - 1) as usize];
                self.reset_value(i,j);
            }
        }

        self.solution = Option::Some(solution);

        for i in 0..9 {
            for j in 0..9 {
                let val = values[(i,j)];
                if val == 0 {
                    self.fixed[(i, j)] = false;
                    continue;
                }
                let random_val = values_random_mask[(val - 1) as usize];
                self.set_value(random_val, i, j);
                self.fixed[(i, j)] = true;
            }
        }
    }

    pub fn reset_value(&mut self, row:usize, col:usize) {
        let rc = (row, col);
        if self.fixed[rc] {
            return;
        }

        let current_val = self.values[rc];

        if current_val == 0 {
            return;
        }

        if self.errors[rc] {
            self.errors[rc] = false;
            self.fixed[rc] = false;
            self.values[rc] = 0;
            return;
        }

        let quad = QUADS[row][col] as usize;
        let mov_zero_based = (current_val - 1) as usize;
        self.reset_value_((rc, quad), mov_zero_based)
    }

    pub fn solve(&mut self) {   
        if self.check_error() {
            return;
        };     
        self.solved = self.solve_(0);
    }

    fn check_error(&mut self) -> bool {
        if self.has_error() {
            self.status_text = format!("Fehler gefunden!");
            true
        } else {
            self.status_text = format!("");
            false
        }
    }

    fn has_error(&self) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if self.is_error(i, j) {
                    return true;
                }
            }
        }
        false
    }

    fn solve_(&mut self, cursor:usize) -> bool {
        if cursor >= 81 {
            return true;
        }
        let rcq = FIELDS[cursor];
        let (rc, _) = rcq;

        match self.get_possible_moves(rc) {
            None => self.solve_(cursor + 1),
            Some(moves) => {
                for mov_zero_based in moves {
                    self.set_value_(rcq, mov_zero_based);
        
                    if self.solve_(cursor + 1) {
                        return true;
                    }

                    self.reset_value_(rcq, mov_zero_based);
                }
                false
            }
        }
    }

    fn solve_random_(&mut self, cursor:usize) -> bool {
        if cursor >= 81 {
            return true;
        }
        let rcq = FIELDS[cursor];
        let (rc, _) = rcq;

        match self.get_possible_moves(rc) {
            None => self.solve_(cursor + 1),
            Some(mut moves) => {
                moves.shuffle(&mut thread_rng());
                for mov_zero_based in moves {
                    self.set_value_(rcq, mov_zero_based);
        
                    if self.solve_(cursor + 1) {
                        return true;
                    }

                    self.reset_value_(rcq, mov_zero_based);
                }
                false
            }
        }
    }

    fn generate_(&mut self, fields_queue: [usize; 81], cursor:usize, removed_count:u8, difficulty:u8) -> bool {
        if cursor >= 81 || self.multiple_solutions_(0) > 1 {
            return false;
        }
        
        if removed_count >= difficulty as u8 {
            return true;
        }

        let rcq = FIELDS[fields_queue[cursor]];
        let (rc, _) = rcq;

        let mov = self.values[rc];
        let mov_zero_based = (mov - 1) as usize;

        self.reset_value_(rcq, mov_zero_based);

        if self.generate_(fields_queue, cursor + 1, removed_count + 1, difficulty) {
            return true;
        }

        self.set_value_(rcq, mov_zero_based);
        self.generate_(fields_queue, cursor + 1, removed_count, difficulty)
    }

    fn multiple_solutions_(&mut self, cursor:usize) -> u8 {
        if cursor >= 81 {
            return 1;
        }
        let rcq = FIELDS[cursor];
        let (rc, _) = rcq;

        match self.get_possible_moves(rc) {
            None => self.multiple_solutions_(cursor + 1),
            Some(moves) => {
                let mut sum = 0;
                for mov_zero_based in moves {
                    self.set_value_(rcq, mov_zero_based);
        
                    sum += self.multiple_solutions_(cursor + 1);

                    self.reset_value_(rcq, mov_zero_based);
                    if sum > 1 {
                        return 2;
                    }
                }
                sum
            }
        }
    }

    fn reset_value_(&mut self, rcq:((usize, usize), usize), mov_zero_based:usize) {
        let mov_bin = VALUES_BIN[mov_zero_based];
        let (rc, quad) = rcq;
        self.values[rc] = 0;
        let (row, col) = rc;
        self.poss_rows[row] |= mov_bin;
        self.poss_cols[col] |= mov_bin;
        self.poss_quads[quad] |= mov_bin;
    }

    fn set_value_(&mut self, rcq:((usize, usize), usize), mov_zero_based:usize) {
        let mov_bin_inv = VALUES_BIN_INV[mov_zero_based];
        let (rc, quad) = rcq;
        let (row, col) = rc;
        self.poss_rows[row] &= mov_bin_inv;
        self.poss_cols[col] &= mov_bin_inv;
        self.poss_quads[quad] &= mov_bin_inv;
        self.values[rc] = (mov_zero_based + 1) as u8;
    }

    fn get_possible_moves(&mut self, rc:(usize,usize)) -> Option<Vec<usize>> {
        let val: u8 = self.values[rc];
        if val > 0 {
            return Option::None;
        }
        
        let (row, col) = rc;
        let quad = QUADS[row][col] as usize;
        let poss:u16 = self.poss_rows[row] & self.poss_cols[col] & self.poss_quads[quad];
        Option::Some(poss.view_bits::<Lsb0>()[0..9].iter_ones().collect())
    }

    fn get_seed(&mut self, difficulty:u8) {
        // We try to remove weak clues and keep few strong ones
        // The strength of an existing clue is the number of possibilities in the field when the clue is removed.
        // For the first 9 clues that are removed from a full solution, the weakest possible clues have strength 1. 
        // The next 9 have at least strength 2 and so on

        let mut cursor_random_mask: Vec<usize> = (0..81).collect();
        cursor_random_mask.shuffle(&mut thread_rng());

        let mut cursor_queue: Vec<usize> = Vec::new();


        for cursor in 0..81 {


        }
        


    }

    fn get_weakest_clue(&mut self) -> usize {
        let mut weakest_strength = 10;
        let mut weakest_clue = 82;
        for clue in 0..81 {
            let rcq = FIELDS[clue];
            let (rc, _) = rcq;

            let value = self.values[rc];
            if value == 0 {
                continue;
            }
            let mov_zero_based = (value - 1) as usize;

            self.reset_value_(rcq, mov_zero_based);
            let strength = self.get_possible_moves(rc).map(|x| x.len())
                .expect("should have possible values after removing clue.");
            self.set_value_(rcq, mov_zero_based);

            if strength < weakest_strength {
                weakest_strength = strength;
                weakest_clue = clue;
            }
        }
        weakest_clue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_counter_empty() {
        let mut playfield = PlayfieldState::new();
        assert!(playfield.multiple_solutions_(0) > 1);
    }

    #[test]
    fn test_solution_counter_full() {
        let mut playfield = PlayfieldState::new();
        playfield.solve();
        assert!(playfield.multiple_solutions_(0) == 1);
    }

    #[test]
    fn x() {
        let a: Vec<u8> = (0..11).map(|i| (i / 3) + 1).collect();
        assert_eq!(a, vec![1,1,1,2,2,2,3,3,3,4,4]);
        // assert_eq!(1, playfield.count_solutions());
    }

    #[test]
    fn test_solution_counter_partial() {
        let mut playfield = PlayfieldState::new();
        playfield.solve();

        let mut cursor_random_mask: [usize; 81] = [0; 81];
        for i in 0..81 {
            cursor_random_mask[i] = i;
        }
        cursor_random_mask.shuffle(&mut thread_rng());

        for i in 0..20 {
            let ((row, col), _) = FIELDS[cursor_random_mask[i]];
            playfield.set_value(0, row, col);
        }

        let values_before = playfield.values.clone();
        assert!(playfield.multiple_solutions_(0) == 1);

        assert_eq!(values_before, playfield.values);
    }

    #[test]
    fn test_solution_counter_partial_2() {
        let mut playfield = PlayfieldState::new();
        playfield.set_value(0, 6, 7);
        playfield.set_value(0, 8, 9);
        playfield.set_value(1, 6, 1);
        playfield.set_value(2, 3, 1);
        playfield.set_value(2, 4, 2);
        playfield.set_value(3, 4, 6);
        playfield.set_value(4, 2, 5);
        playfield.set_value(4, 3, 8);
        playfield.set_value(4, 6, 2);
        playfield.set_value(4, 8, 4);
        playfield.set_value(5, 1, 9);
        playfield.set_value(5, 2, 7);
        playfield.set_value(5, 3, 2);
        playfield.set_value(5, 7, 6);
        playfield.set_value(5, 8, 5);
        playfield.set_value(6, 0, 5);
        playfield.set_value(6, 2, 1);
        playfield.set_value(6, 5, 2);

        assert!(playfield.multiple_solutions_(0) > 1);
    }

    #[test]
    fn test_set_value() {
        let playfield = &mut PlayfieldState::new();

        playfield.set_value(0, 0, 1);
        check(playfield, 0, 0, 1);

        playfield.set_value(0, 0, 2);
        check(playfield, 0, 0, 2);

        playfield.set_value(0, 0, 0);
        check(playfield, 0, 0, 0)
    }

    #[test]
    fn test_solution_counter_partial_3() {
        let mut playfield = PlayfieldState::new();
        playfield.solve();
        playfield.reset_value(0, 8);
        playfield.reset_value(1, 5);
        playfield.reset_value(2, 2);
        playfield.reset_value(2, 2);
        playfield.reset_value(3, 7);
        playfield.reset_value(4, 4);
        playfield.reset_value(5, 1);
        playfield.reset_value(6, 6);
        playfield.reset_value(7, 3);
        playfield.reset_value(8, 0);

        playfield.set_value(7, 1, 5);
        playfield.set_value(7, 1, 6);
        playfield.set_value(7, 1, 7);
        playfield.set_value(7, 1, 8);
        playfield.set_value(7, 1, 9);
        playfield.set_value(7, 1, 0);

        assert_eq!(playfield.get_possible_moves((7, 1)).unwrap(), vec![3,8]);

        assert_eq!(playfield.multiple_solutions_(0), 1);
    }

    fn check(playfield:&mut PlayfieldState, row:usize, col:usize, val:usize) {
        let mut v:Vec<usize> = vec![];
        let val_zero_based = val as i32 - 1;

        for i in 0..9 {
            if i == val_zero_based {
                continue;
            }
            v.push(i as usize);
        }

        for i in 0..9 {
            if row != i {
                assert_eq!(playfield.get_possible_moves((row, i)).unwrap(), v.clone());
            }

            if col != i {
                assert_eq!(playfield.get_possible_moves((i, col)).unwrap(), v.clone());
            }

            if !(row == i && col == i) {
                let r = i/3;
                let c = i%3;

                assert_eq!(playfield.get_possible_moves((r, c)).unwrap(), v.clone());
            }
        }
    }
}
