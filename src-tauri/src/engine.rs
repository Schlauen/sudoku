use array2d::Array2D;
use bitvec::{order::Lsb0, view::BitView};
use rand::prelude::*;

// (row, col, quad) triplets
const FIELDS:[(usize, usize, usize); 81] = [
    (0,0,0), (0,1,0), (0,2,0), (0,3,1), (0,4,1), (0,5,1), (0,6,2), (0,7,2), (0,8,2),
    (1,0,0), (1,1,0), (1,2,0), (1,3,1), (1,4,1), (1,5,1), (1,6,2), (1,7,2), (1,8,2),
    (2,0,0), (2,1,0), (2,2,0), (2,3,1), (2,4,1), (2,5,1), (2,6,2), (2,7,2), (2,8,2),
    (3,0,3), (3,1,3), (3,2,3), (3,3,4), (3,4,4), (3,5,4), (3,6,5), (3,7,5), (3,8,5),
    (4,0,3), (4,1,3), (4,2,3), (4,3,4), (4,4,4), (4,5,4), (4,6,5), (4,7,5), (4,8,5),
    (5,0,3), (5,1,3), (5,2,3), (5,3,4), (5,4,4), (5,5,4), (5,6,5), (5,7,5), (5,8,5),
    (6,0,6), (6,1,6), (6,2,6), (6,3,7), (6,4,7), (6,5,7), (6,6,8), (6,7,8), (6,8,8),
    (7,0,6), (7,1,6), (7,2,6), (7,3,7), (7,4,7), (7,5,7), (7,6,8), (7,7,8), (7,8,8),
    (8,0,6), (8,1,6), (8,2,6), (8,3,7), (8,4,7), (8,5,7), (8,6,8), (8,7,8), (8,8,8),
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


#[derive(Debug)]
struct Sudoku {
    values: Array2D<u8>,
    poss_rows: [u16; 9],
    poss_cols: [u16; 9],
    poss_quads: [u16; 9],
}

impl Sudoku {
    fn new(values_option: Option<&Array2D<u8>>) -> Result<Sudoku, String> {
        let mut s = Sudoku {
            values: Array2D::filled_with(0, 9, 9),
            poss_rows: [0b1111111111111111u16; 9],
            poss_cols: [0b1111111111111111u16; 9],
            poss_quads: [0b1111111111111111u16; 9],
        };
        match values_option {
            Some(values) => values.elements_row_major_iter().enumerate().map(|(index, value_ref)| {
                let (r,c,q) = FIELDS[index];
                let value = *value_ref;
                if value > 0 {
                    let mov_zero_based = (value - 1) as usize;
                    return match s.get_possible_moves((r,c,q)) {
                        Some(moves) => {
                            if moves.contains(&mov_zero_based) {
                                s.set_value((r,c,q), mov_zero_based);
                                Ok(())
                            } else {
                                Err("conflict")
                            }
                        },
                        None => Err("conflict")
                    };
                }
                Ok(())
            }).fold(Ok(()), |a, b| {
                if a.is_err() {
                    return Err(a.unwrap_err());
                }
                if b.is_err() {
                    return Err(b.unwrap_err().into());
                }
                return Ok(());
            }).map(|_| s),
            None => Ok(s)
        }
    }

    fn hint(&mut self) -> Option<(usize, usize)> {
        let mut fields = FIELDS.clone().into_iter()
            .filter(|(r,c,_)| self.values[(*r, *c)] == 0)
            .collect::<Vec<(usize, usize, usize)>>();
        fields.shuffle(&mut ThreadRng::default());
        fields.get(self.get_weakest_clue_idx_in(&fields)).map(|clue| {
            let (r,c,_) = *clue;
            (r,c)
        })
    }

    fn solve_random(&mut self, cursor:usize, seed:u64) -> bool {
        if cursor < 81 {
            let rcq = FIELDS[cursor];
            return match self.get_possible_moves_rnd(rcq, &mut StdRng::seed_from_u64(seed)) {
                None => self.solve_random(cursor + 1, seed),
                Some(moves) => {
                    for mov_zero_based in moves {
                        self.set_value(rcq, mov_zero_based);
            
                        if self.solve_random(cursor + 1, seed) {
                            return true;
                        }
    
                        self.reset_value(rcq, mov_zero_based);
                    }
                    false
                }
            }
        }
        true
    }

    fn solve(&mut self, cursor:usize) -> bool {
        if cursor < 81 {
            let rcq = FIELDS[cursor];
            return match self.get_possible_moves(rcq) {
                None => self.solve(cursor + 1),
                Some(moves) => {
                    for mov_zero_based in moves {
                        self.set_value(rcq, mov_zero_based);
            
                        if self.solve(cursor + 1) {
                            return true;
                        }
    
                        self.reset_value(rcq, mov_zero_based);
                    }
                    false
                }
            }
        }
        true
    }

    fn generate(&mut self, cursor:usize, fields:&Vec<(usize, usize, usize)>, removed_count:u8, difficulty:u8) -> bool {
        if cursor >= fields.len() || self.count_solutions(0, 2) > 1 {
            return false;
        }

        if removed_count >= difficulty as u8 {
            return true;
        }

        let rcq = fields[cursor];
        let (r,c,_) = rcq;
        let mov = self.values[(r,c)];
        let mov_zero_based = (mov - 1) as usize;

        self.reset_value(rcq, mov_zero_based);
        
        if self.generate(cursor + 1, fields, removed_count + 1, difficulty) {
            return true;
        }

        self.set_value(rcq, mov_zero_based);
        self.generate(cursor + 1, fields, removed_count, difficulty)
    }

    fn count_solutions(&mut self, cursor:usize, limit:u8) -> u8 {
        if cursor < 81 {
            let rcq = FIELDS[cursor];
            return match self.get_possible_moves(rcq) {
                None => self.count_solutions(cursor + 1, limit),
                Some(moves) => {
                    let mut sum = 0;
                    for mov_zero_based in moves {
                        self.set_value(rcq, mov_zero_based);
            
                        sum += self.count_solutions(cursor + 1, limit);

                        self.reset_value(rcq, mov_zero_based);
                        if sum >= limit {
                            return limit;
                        }
                    }
                    sum
                }
            };
        }
        1
    }

    fn set_value(&mut self, rcq:(usize, usize, usize), mov_zero_based:usize) {
        let mov_bin_inv = VALUES_BIN_INV[mov_zero_based];
        let (r, c, q) = rcq;
        self.poss_rows[r] &= mov_bin_inv;
        self.poss_cols[c] &= mov_bin_inv;
        self.poss_quads[q] &= mov_bin_inv;
        self.values[(r,c)] = (mov_zero_based + 1) as u8;
    }

    fn reset_value(&mut self, rcq:(usize, usize, usize), mov_zero_based:usize) {
        let mov_bin = VALUES_BIN[mov_zero_based];
        let (r, c, q) = rcq;
        self.values[(r,c)] = 0;
        self.poss_rows[r] |= mov_bin;
        self.poss_cols[c] |= mov_bin;
        self.poss_quads[q] |= mov_bin;
    }

    fn get_possible_moves_rnd<R>(&self, rcq:(usize, usize, usize), rng: &mut R) -> Option<Vec<usize>>
    where R: Rng + ?Sized {
        let moves_option = self.get_possible_moves(rcq);
        match self.get_possible_moves(rcq).as_mut() {
            Some(moves) => moves.shuffle(rng),
            None => {}
        }
        moves_option
    }

    fn get_possible_moves(&self, rcq:(usize, usize, usize)) -> Option<Vec<usize>> {
        let (r, c, q) = rcq;
        let val: u8 = self.values[(r,c)];
        if val > 0 {
            return Option::None;
        }
        
        let poss:u16 = self.poss_rows[r] & self.poss_cols[c] & self.poss_quads[q];
        Option::Some(poss.view_bits::<Lsb0>()[0..9].iter_ones().collect())
    }

    fn get_weakest_clue_idx_in(&self, fields:&Vec<(usize, usize, usize)>) -> usize {
        let mut weakest_strength = 10;
        let mut weakest_clue_idx = 0;
        for (clue_idx, rcq_ref) in fields.iter().enumerate() {
            let (r,c,q) = *rcq_ref;
            let value = self.values[(r,c)];

            let poss = match value == 0 {
                true => {
                    self.poss_rows[r] & self.poss_cols[c] & self.poss_quads[q]
                },
                false => {
                    let mov_zero_based = (value - 1) as usize;
                    let mov_bin = VALUES_BIN[mov_zero_based];
                    (self.poss_rows[r] | mov_bin) & (self.poss_cols[c] | mov_bin) & (self.poss_quads[q] | mov_bin)
                }
            };
            
            let strength = poss.view_bits::<Lsb0>()[0..9].count_ones();
            if strength < weakest_strength {
                weakest_strength = strength;
                weakest_clue_idx = clue_idx;
            }
        }
        weakest_clue_idx
    }
}

fn generate_sequence(values: Array2D<u8>, seed:u64, mut fields: Vec<(usize, usize, usize)>) -> Result<Vec<(usize, usize, usize)>, String> {
    // Try to remove weak clues and keep few strong ones
    // The strength of an existing clue is the number of possibilities in the field when the clue is removed.
    // values must be fully filled
    fields.shuffle(&mut StdRng::seed_from_u64(seed));

    let mut cursor_queue: Vec<(usize, usize, usize)> = Vec::new();

    Sudoku::new(Option::Some(&values)).map(|mut s| {
        while fields.len() > 0 {
            let weakest_clue = fields.remove(s.get_weakest_clue_idx_in(&fields));
            let (r,c,_) = weakest_clue;
            let value = s.values[(r,c)];
            s.reset_value(weakest_clue, (value - 1) as usize);
            cursor_queue.push(weakest_clue);
        }
        cursor_queue
    })
}

pub fn hint(values: &Array2D<u8>) -> Result<(usize, usize), String> {
    let result = Sudoku::new(Option::Some(values));
    if result.is_err() {
        return Err(result.unwrap_err());
    }

    let mut sudoku = result.unwrap();
    sudoku.hint().ok_or("No hint found".into())
}

/// Solves a given Sudoku grid.
/// Given values are not changed. If `seed` is provided, solve is performed randomly. 
/// This could lead to inconsistent results when multiple solutions are possible.
/// For optimal performance, provide `Option::None`.
pub fn solve(values: &Array2D<u8>, seed_option:Option<u64>) -> Result<Array2D<u8>, String> {
    Sudoku::new(Option::Some(values)).map(|mut s| {
        match seed_option {
            Some(seed) => s.solve_random(0, seed),
            None => s.solve(0),
        };
        s.values
    })
}

/// Generates a Sudoku grid of the desired difficulty with a unique solution.
/// The difficulty is the number of empty fields. The non-zero fields of the provided 
/// values-grid are preserved. As a result, a tuple of `(clues, solution)` is returned.
pub fn generate(values: &Array2D<u8>, seed:u64, difficulty:u8) -> Result<(Array2D<u8>, Array2D<u8>), String> {
    if difficulty > 58 {
        return Err("Maximum difficulty is 58".into());
    }
    
    let nullable_fields: Vec<(usize, usize, usize)> = FIELDS.into_iter().filter(|rcq_ref| {
        let (r,c,_) = *rcq_ref;
        values[(r,c)] == 0
    }).collect();
    if nullable_fields.len() < difficulty as usize {
        return Err("difficulty must be less than or equal to the number of empty fields".into());
    }

    let solve_result = solve(values, Option::Some(seed));
    if solve_result.is_err() {
        return Err(solve_result.unwrap_err());
    }

    let solution = solve_result.unwrap();
    let fields_sequence_result = generate_sequence(solution.clone(), seed, nullable_fields);
    if fields_sequence_result.is_err() {
        return Err(fields_sequence_result.unwrap_err());
    }

    let fields_sequence = &fields_sequence_result.unwrap();
    let result = Sudoku::new(Option::Some(&solution));
    if result.is_err() {
        return Err(result.unwrap_err());
    }
    
    let mut sudoku = result.unwrap();
    if !sudoku.generate(0, fields_sequence, 0, difficulty){
        return Err("Error occured during solution generation".into());
    }

    Ok((sudoku.values, solution))
}

/// Counts solutions up to a maximum of `limit`.
pub fn count_solutions(values: &Array2D<u8>, limit:u8) -> u8 {
    Sudoku::new(Option::Some(values))
        .map(|mut s| s.count_solutions(0, limit))
        .unwrap_or_else(|_| 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_solutions() {
        let rows = vec![
            vec![0,0,0, 0,0,0, 7,0,9],
            vec![0,0,0, 0,0,0, 1,0,0],
            vec![0,0,0, 1,2,0, 0,0,0],

            vec![0,0,0, 0,6,0, 0,0,0],
            vec![0,0,5, 8,0,0, 2,0,4],
            vec![0,9,7, 2,0,0, 0,6,5],
            
            vec![5,0,1, 0,0,2, 0,0,0],
            vec![0,0,0, 0,0,0, 0,0,0],
            vec![0,0,0, 0,0,0, 0,0,0],
        ];
        
        let values = Array2D::from_rows(&rows).unwrap();

        use std::time::Instant;
        let now = Instant::now();
        assert_eq!(count_solutions(&values, 100), 100);

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }

    #[test]
    fn test_create_invalid_grids() {
        // conflict in col 4
        let mut rows = vec![
            vec![0,0,0, 0,0,0, 7,0,9],
            vec![0,0,0, 0,0,0, 1,0,0],
            vec![0,0,0, 1,2,0, 0,0,0],

            vec![0,0,0, 0,6,0, 0,0,0],
            vec![0,0,5, 8,0,0, 2,0,4],
            vec![0,9,7, 2,0,0, 0,6,5],
            
            vec![5,0,1, 0,0,2, 0,0,0],
            vec![0,0,0, 0,0,0, 0,0,0],
            vec![0,0,0, 0,6,0, 0,0,0],
        ];
        let mut values = Array2D::from_rows(&rows).unwrap();
        assert!(Sudoku::new(Option::Some(&values)).is_err());

        // conflict on row 4
        rows = vec![
            vec![0,0,0, 0,0,0, 7,0,9],
            vec![0,0,0, 0,0,0, 1,0,0],
            vec![0,0,0, 1,2,0, 0,0,0],

            vec![0,0,0, 0,6,0, 0,0,0],
            vec![0,0,5, 8,0,0, 2,8,4],
            vec![0,9,7, 2,0,0, 0,6,5],
            
            vec![5,0,1, 0,0,2, 0,0,0],
            vec![0,0,0, 0,0,0, 0,0,0],
            vec![0,0,0, 0,0,0, 0,0,0],
        ];
        values = Array2D::from_rows(&rows).unwrap();
        assert!(Sudoku::new(Option::Some(&values)).is_err());

        // conflict in quad 3
        rows = vec![
            vec![0,0,0, 0,0,0, 7,0,9],
            vec![0,0,0, 0,0,0, 1,0,0],
            vec![0,0,0, 1,2,0, 0,0,0],

            vec![5,0,0, 0,6,0, 0,0,0],
            vec![0,0,5, 8,0,0, 2,0,4],
            vec![0,9,7, 2,0,0, 0,6,5],
            
            vec![5,0,1, 0,0,2, 0,0,0],
            vec![0,0,0, 0,0,0, 0,0,0],
            vec![0,0,0, 0,0,0, 0,0,0],
        ];
        values = Array2D::from_rows(&rows).unwrap();
        assert!(Sudoku::new(Option::Some(&values)).is_err());
    }

    #[test]
    fn test_solve() {
        let rows = vec![
            vec![0,0,0, 0,0,0, 7,0,9],
            vec![0,0,0, 0,0,0, 1,0,0],
            vec![0,0,0, 1,2,0, 0,0,0],

            vec![0,0,0, 0,6,0, 0,0,0],
            vec![0,0,5, 8,0,0, 2,0,4],
            vec![0,9,7, 2,0,0, 0,6,5],
            
            vec![5,0,1, 0,0,2, 0,0,0],
            vec![0,0,0, 0,0,0, 0,0,0],
            vec![0,0,0, 0,0,0, 0,0,0],
        ];
        let values = Array2D::from_rows(&rows).unwrap();
        
        use std::time::Instant;
        let mut now = Instant::now();
        
        let mut solution = solve(&values, Option::Some(5));

        let mut elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);

        assert!(solution.is_ok());
        assert!(solution.unwrap().elements_row_major_iter().all(|i| *i > 0));

        now = Instant::now();
        
        solution = solve(&values, Option::None);
        
        elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);

        assert!(solution.is_ok());
        assert!(solution.unwrap().elements_row_major_iter().all(|i| *i > 0));
    }

    #[test]
    fn test_generate() {        
        let values = Array2D::filled_with(0, 9, 9);
        
        use std::time::Instant;
        let now = Instant::now();
        
        let solution = generate(&values, 42, 58);

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);

        assert!(solution.is_ok());
    }

    #[test]
    fn test_generate_with_preset() {
        let rows = vec![
            vec![0,0,0, 0,0,0, 7,0,9],
            vec![0,0,0, 0,0,0, 1,0,0],
            vec![0,0,0, 1,2,0, 0,0,0],

            vec![0,0,0, 0,6,0, 0,0,0],
            vec![0,0,5, 8,0,0, 2,0,4],
            vec![0,9,7, 2,0,0, 0,6,5],
            
            vec![5,0,1, 0,0,2, 0,0,0],
            vec![0,0,0, 0,0,0, 0,0,0],
            vec![0,0,0, 0,0,0, 0,0,0],
        ];
        let values = Array2D::from_rows(&rows).unwrap();
        
        use std::time::Instant;
        let now = Instant::now();
        
        let solution = generate(&values, 37, 50);

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);

        assert!(solution.is_ok());

        let s = solution.unwrap().0;
        for i in 0..9 {
            for j in 0..9 {
                let initial_value = values[(i,j)];
                if initial_value > 0 {
                    assert_eq!(initial_value, s[(i,j)])
                }
            }
        }
    }
}
