
use array2d::Array2D;
use bitvec::prelude::*;
use rand::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json;

use crate::Request;

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

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CellState {
    Blank,
    Fix,
    Set,
    Error,
}

#[derive(serde::Serialize, Clone)]
struct CellUpdateEvent {
    row: u8,
    col: u8,
    value: u8,
    state: u8,
}

#[derive(serde::Serialize, Clone)]
struct GameUpdateEvent {
    state: u8,
    clue_count: Option<u8>,
    solution_count: Option<u8>,
}

impl TryFrom<u8> for CellState {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, <CellState as TryFrom<u8>>::Error> {
        match v {
            x if x == CellState::Blank as u8 => Ok(CellState::Blank),
            x if x == CellState::Fix as u8 => Ok(CellState::Fix),
            x if x == CellState::Set as u8 => Ok(CellState::Set),
            x if x == CellState::Error as u8 => Ok(CellState::Error),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum GameState {
    Blank,
    Running,
    Solved,
    Error,
}

#[derive(Serialize, Deserialize)]
struct SerializableArray2D(#[serde(with = "array2d_as_string")] Array2D<u8>);
mod array2d_as_string {
    use array2d::Array2D;
    use serde::{Deserialize, Serializer, Deserializer};

    pub fn serialize<S>(arr: &Array2D<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{:?}", arr.as_row_major()).replace(" ", ""))
    }
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Array2D<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Array2D::from_row_major(
            &s
                .replace("[", "")
                .replace("]", "")
                .split(',')
                .map(|x| x.parse::<u8>().unwrap())
                .collect::<Vec<u8>>(),
                9,
                9
            ).map_err(serde::de::Error::custom)
    }
}

#[derive(Serialize, Deserialize)]
struct SerializableArray2DOption(#[serde(with = "array2d_option_as_string")] Option<Array2D<u8>>);
mod array2d_option_as_string {
    use array2d::Array2D;
    use serde::{Deserialize, Serializer, Deserializer};

    pub fn serialize<S>(arr: &Option<Array2D<u8>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = arr.clone().map(|a| format!("{:?}", a.as_row_major()).replace(" ", "")).unwrap_or("".into());
        serializer.serialize_str(&s)
    }
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Array2D<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer).map(|s| Array2D::from_row_major(
            &s
                .replace("[", "")
                .replace("]", "")
                .split(',')
                .map(|x| x.parse::<u8>().unwrap())
                .collect::<Vec<u8>>(),
                9,
                9
            ).ok()).map_err(serde::de::Error::custom)
    }
}

#[derive(Serialize, Deserialize)]
struct SerializableArray2DE(#[serde(with = "array2d_e_as_string")] Array2D<CellState>);
mod array2d_e_as_string {
    use array2d::Array2D;
    use serde::{Deserialize, Serializer, Deserializer};

    use super::CellState;
    
    pub fn serialize<S>(arr: &Array2D<CellState>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!(
            "{:?}", 
            arr.as_row_major().iter().map(|e| *e as u8).collect::<Vec<u8>>()
        ).replace(" ", ""))
    }
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Array2D<CellState>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Array2D::from_row_major(
            &s
                .replace("[", "")
                .replace("]", "")
                .split(',')
                .map(|x| CellState::try_from(x.parse::<u8>().unwrap()).unwrap())
                .collect::<Vec<CellState>>(),
                9,
                9
            ).map_err(serde::de::Error::custom)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Playfield {

    #[serde(with = "array2d_as_string")]
    values: Array2D<u8>,

    #[serde(with = "array2d_option_as_string", skip_serializing_if = "Option::is_none")]
    solution: Option<Array2D<u8>>,

    #[serde(with = "array2d_e_as_string")]
    states: Array2D<CellState>,

    poss_rows: [u16; 9],
    poss_cols: [u16; 9],
    poss_quads: [u16; 9],
    show_errors: bool,
    state: GameState,
    difficulty: u8,
    timer_seconds: u32,
    seed: u64,
}

impl Playfield {
    pub fn from_json(string: &str, request:Option<&Request>) -> Playfield {
        let mut p:Playfield = serde_json::from_str(string).unwrap();
        if let Some(r) = request {
            p.emit_update_event(r);
        }
        
        p
    }

    pub fn new(difficulty:u8, request:Option<&Request>) -> Playfield {
        let mut p = Playfield { 
            values: Array2D::filled_with(0, 9, 9),
            solution: Option::None,
            states: Array2D::filled_with(CellState::Blank, 9, 9),
            poss_rows: [0b1111111111111111u16; 9],
            poss_cols: [0b1111111111111111u16; 9],
            poss_quads: [0b1111111111111111u16; 9],
            show_errors: false,
            state: GameState::Blank,
            difficulty,
            timer_seconds: 0,
            seed: 42,
        };
        if let Some(r) = request {
            p.emit_update_event(r);
        }

        p
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    pub fn get_clue_count(&self) -> u8 {
        self.states.elements_row_major_iter().filter(|s| match s {
            CellState::Fix | CellState::Error | CellState::Set => true,
            CellState::Blank => false,            
        }).count() as u8
    }

    pub fn increment_timer(&mut self) -> Result<u32, String> {
        match self.state {
            GameState::Blank | GameState::Solved => {},
            GameState::Error | GameState::Running => self.timer_seconds += 1,      
        };
        
        Ok(self.timer_seconds)
    }

    pub fn reset(&mut self, window:Option<&Request>) -> Result<GameState, String> {
        FIELDS.iter().for_each(|((r, c), _)| match self.states[(*r, *c)] {
            CellState::Blank | CellState::Error | CellState::Set => {
                self.reset_value(*r, *c, window);
            },
            CellState::Fix => ()
        });
        Ok(self.state)
    }

    pub fn set_value(&mut self, value:u8, row:usize, col:usize, request:Option<&Request>) -> Result<u8, String> {
        let rc = (row, col);

        match self.state {
            GameState::Blank => self.state = GameState::Running,
            GameState::Solved => return Err("Already solved".into()),
            GameState::Error | GameState::Running => {}        
        };
        
        match self.states[rc] {
            CellState::Fix => {
                return Err("Cell is immutable".into());
            },
            CellState::Error | CellState::Blank | CellState::Set => {
                if value == 0 {
                    self.reset_value(row, col, Option::None);
                } else {
                    let current_val = self.values[rc];
                    if current_val > 0 {
                        self.reset_value(row, col, Option::None);
                    }
            
                    let mov_zero_based = (value - 1) as usize;
                    match self.get_possible_moves(rc) {
                        Some(moves) => {
                            if moves.contains(&mov_zero_based) {
                                self.set_value_((rc, QUADS[row][col]), mov_zero_based);
                            } else {
                                self.values[rc] = value;
                            }
                        },
                        None => {
                            self.values[rc] = value;
                        }
                    }
                }
            }
        }

        self.update_states(request);
        
        Ok(self.values[rc])
    }

    pub fn get_value(&self, row:usize, col:usize) -> u8 {
        self.values[(row, col)]
    }

    fn is_error(&mut self, row:usize, col:usize) -> bool {
        match self.solution.as_ref() {
            None => {
                let value = self.values[(row, col)];
                if value == 0 {
                    return false;
                }

                for i in 0..9 {
                    if i != col && value == self.values[(row, i)] {
                        return true;
                    }
                    if i != row && value == self.values[(i, col)] {
                        return true;
                    }
                    let r = i / 3 + 3*(row/3);
                    let c = i % 3 + 3*(col/3);
                    if !(r == row && c == col) && value == self.values[(r,c)] {
                        return true;
                    }
                }
                return false;
            },
            Some(s) => {
                let val = self.values[(row, col)];
                if val == 0 {
                    false
                } else {
                    if s[(row, col)] == val {
                        false
                    } else {
                        true
                    }
                }
            }
        }
    }

    pub fn generate(&mut self, difficulty:u8, seed:u64, request:Option<&Request>, fix_result:bool) -> Result<GameState, String> {
        self.difficulty = difficulty;
        self.seed = seed;

        if !self.solve_random_(0, seed) {
            return Err("Error occured during seed generation".into());
        }

        let cursor_random_mask = self.generate_seed(seed);
        
        if !self.generate_(&cursor_random_mask, 0, 0, difficulty) {
            return Err("Error occured during solution generation".into());
        }

        for row in 0..9 {
            for col in 0..9 {
                let rc = (row, col);
                let val = self.values[rc];
                if val == 0 {
                    self.states[rc] = CellState::Blank;
                } else {
                    self.states[rc] = CellState::Set;
                }
            }
        }

        if fix_result {
            self.fix_current();
        }
        
        self.state = GameState::Running;
        if let Some(r) = request {
            self.emit_update_event(r);
        }
        Ok(self.state)
    }

    pub fn fix_current(&mut self) {
        self.solution = Option::Some(self.values.clone());
        for row in 0..9 {
            for col in 0..9 {
                let rc = (row, col);
                let val = self.values[rc];
                if val > 0 {
                    self.states[rc] = CellState::Fix;
                }
            }
        }
    }

    
    pub fn emit_update_event(&mut self, request:&Request) {
        for row in 0..9 {
            for col in 0..9 {
                self.emit_update_cell_event(row, col, request);
            }
        }
        self.emit_update_game_event(request);
    }

    fn emit_update_cell_event(&self, row:usize, col:usize, request:&Request) {
        let event = CellUpdateEvent {
            row: row as u8,
            col: col as u8,
            value: self.values[(row, col)],
            state: self.states[(row, col)] as u8,
        };
        request.window.emit(&format!("updateCell-{}-{}", row, col), event).unwrap();
    }

    fn emit_update_game_event(&mut self, request:&Request) {
        let event = GameUpdateEvent {
            state: self.state as u8,
            clue_count: match request.include_clue_count {
                true => Option::Some(self.get_clue_count()),
                false => Option::None,
            },
            solution_count: match request.include_solution_count {
                true => Option::Some(self.count_solutions(5)),
                false => Option::None,
            }
        };
        request.window.emit("updateGame", event).unwrap();
    }

    fn reset_value(&mut self, row:usize, col:usize, request:Option<&Request>) {
        let rc = (row, col);
        match self.states[rc] {
            CellState::Blank | CellState::Fix => {},
            CellState::Error | CellState::Set => {
                let current_val = self.values[rc];

                if current_val == 0 {
                    return;
                }

                let quad = QUADS[row][col] as usize;
                let mov_zero_based = (current_val - 1) as usize;
                self.reset_value_((rc, quad), mov_zero_based);
                self.update_states(request);
            }
        };
    }

    pub fn solve(&mut self, request:Option<&Request>) -> Result<GameState, String> {   
        match self.state {
            GameState::Solved => {
                return Err("Already solved".into());
            },
            GameState::Error => {
                return Err("Conflict detected, can't solve".into())
            },
            GameState::Blank | GameState::Running => {}
        };

        self.solve_(0);
        
        self.solution = Option::Some(self.values.clone());
        self.update_states(request);

        Ok(self.state)
    }

    fn update_states(&mut self, request:Option<&Request>) {
        let cell_states = FIELDS.iter().map(|(rc, q)| {
            let (row, col) = *rc;
            let rc = (row, col);
            let current_state = self.states[rc];
            let new_state = match current_state {
                CellState::Error => {
                    if !self.is_error(row, col) {
                        if self.values[rc] > 0 {
                            CellState::Set
                        } else {
                            CellState::Blank
                        }
                    } else {
                        current_state
                    }
                }
                CellState::Fix => current_state,
                CellState::Blank | CellState::Set => {
                    if self.is_error(row, col) {
                        CellState::Error
                    } else {
                        if self.values[rc] > 0 {
                            CellState::Set
                        } else {
                            CellState::Blank
                        }
                    }
                }
            };
            
            self.states[rc] = new_state;
            
            if let Some(w) = request {
                self.emit_update_cell_event(row, col, w);
            }
            new_state
        }).collect::<Vec<CellState>>();

        let has_any_errors = cell_states.iter().any(|state| {
            match state {
                CellState::Error => true,
                CellState::Blank | CellState::Fix | CellState::Set => false,
            }
        });
        let all_empty = !has_any_errors && cell_states.iter().all(|state| {
            match state {
                CellState::Blank => true,
                CellState::Error | CellState::Fix | CellState::Set => false,
            }
        });
        let all_set = !has_any_errors && cell_states.iter().all(|state| {
            match state {
                CellState::Fix | CellState::Set => true,
                CellState::Error | CellState::Blank => false,
            }
        }); 

        let new_state:GameState;
        if has_any_errors {
            new_state = GameState::Error;
        } else if all_set {
            new_state = GameState::Solved;
        } else if all_empty {
            new_state = GameState::Blank;
        } else {
            new_state = GameState::Running;
        }
        self.state = new_state;

        if let Some(req) = request {
            self.emit_update_game_event(req);
        }
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

    fn solve_random_(&mut self, cursor:usize, seed:u64) -> bool {
        if cursor >= 81 {
            return true;
        }
        let rcq = FIELDS[cursor];
        let (rc, _) = rcq;

        match self.get_possible_moves(rc) {
            None => self.solve_(cursor + 1),
            Some(mut moves) => {
                moves.shuffle(&mut StdRng::seed_from_u64(seed));
                for mov_zero_based in moves {
                    self.set_value_(rcq, mov_zero_based);
        
                    if self.solve_random_(cursor + 1, seed) {
                        return true;
                    }

                    self.reset_value_(rcq, mov_zero_based);
                }
                false
            }
        }
    }

    fn generate_(&mut self, fields_queue: &Vec<usize>, cursor:usize, removed_count:u8, difficulty:u8) -> bool {
        if cursor >= fields_queue.len() || self.count_solutions_(0, 2) > 1 {
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

    pub fn count_solutions(&mut self, limit:u8) -> u8 {
        if self.state == GameState::Error {
            println!("error state, no solutions");
            return 0;
        }
        self.count_solutions_(0, limit)
    }

    fn count_solutions_(&mut self, cursor:usize, limit:u8) -> u8 {
        if cursor >= 81 {
            return 1;
        }
        let rcq = FIELDS[cursor];
        let (rc, _) = rcq;

        match self.get_possible_moves(rc) {
            None => self.count_solutions_(cursor + 1, limit),
            Some(moves) => {
                let mut sum = 0;
                for mov_zero_based in moves {
                    self.set_value_(rcq, mov_zero_based);
        
                    sum += self.count_solutions_(cursor + 1, limit);

                    self.reset_value_(rcq, mov_zero_based);
                    if sum >= limit {
                        return limit;
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

    fn get_possible_moves(&self, rc:(usize,usize)) -> Option<Vec<usize>> {
        let val: u8 = self.values[rc];
        if val > 0 {
            return Option::None;
        }
        
        let (row, col) = rc;
        let quad = QUADS[row][col] as usize;
        let poss:u16 = self.poss_rows[row] & self.poss_cols[col] & self.poss_quads[quad];
        Option::Some(poss.view_bits::<Lsb0>()[0..9].iter_ones().collect())
    }

    fn generate_seed(&mut self, seed:u64) -> Vec<usize> {
        // We try to remove weak clues and keep few strong ones
        // The strength of an existing clue is the number of possibilities in the field when the clue is removed.
        // For the first 9 clues that are removed from a full solution, the weakest possible clues have strength 1. 
        // For the first 9 clues that are removed from a full solution, the weakest possible clues have strength 1. 
        // The next 9 have at least strength 2 and so on
        // For the first 9 clues that are removed from a full solution, the weakest possible clues have strength 1.
        // The next 9 have at least strength 2 and so on

        let values = self.values.clone();
        let mut fields: Vec<usize> = (0..81).collect();
        fields.shuffle(&mut StdRng::seed_from_u64(seed));

        let mut cursor_queue: Vec<usize> = Vec::new();

        while fields.len() > 0 {
            let weakest_clue_idx = self.get_weakest_clue_idx_in(&fields);
            let weakest_clue = fields.remove(weakest_clue_idx);
            let rcq = FIELDS[weakest_clue];
            let (rc, _) = rcq;
            let value = self.values[rc];
            self.reset_value_(rcq, (value - 1) as usize);
            cursor_queue.push(weakest_clue);
        }

        for (cursor, value) in values.elements_row_major_iter().enumerate() {
            let rcq = FIELDS[cursor];
            self.set_value_(rcq, (*value - 1) as usize);
        }
        cursor_queue
    }

    fn get_weakest_clue_idx_in(&self, fields:&Vec<usize>) -> usize {
        let mut weakest_strength = 10;
        let mut weakest_clue_idx = 0;
        for (clue_idx, clue) in fields.iter().enumerate() {
            let rcq = FIELDS[*clue];
            let (rc, _) = rcq;

            if match self.states[rc] {
                CellState::Blank => false,
                CellState::Error | CellState::Fix | CellState::Set => true,
            } {
                continue;
            }
            
            let value = self.values[rc];
            if value == 0 {
                continue;
            }

            let strength = self.get_strength(rcq);
            if strength < weakest_strength {
                weakest_strength = strength;
                weakest_clue_idx = clue_idx;
            }
        }
        weakest_clue_idx
    }

    fn get_strength(&self, rcq:((usize, usize), usize)) -> usize {
        let (rc, quad) = rcq;
        let (row, col) = rc;

        let value = self.values[rc];

        let poss = match value == 0 {
            true => {
                self.poss_rows[row] & self.poss_cols[col] & self.poss_quads[quad]
            },
            false => {
                let mov_zero_based = (value - 1) as usize;
                let mov_bin = VALUES_BIN[mov_zero_based];
                let (row, col) = rc;
                (self.poss_rows[row] | mov_bin) & (self.poss_cols[col] | mov_bin) & (self.poss_quads[quad] | mov_bin)
            }
        };
        poss.view_bits::<Lsb0>()[0..9].count_ones()
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_is_error() {
        let mut playfield = Playfield::new(50, Option::None);
        let _ = playfield.set_value(1, 1, 2, Option::None);
        assert!(!playfield.is_error(1, 2));
        assert!(!playfield.is_error(7, 2));

        let _ = playfield.set_value(1, 7, 2, Option::None);
        assert!(matches!(playfield.states[(1,2)], CellState::Error));
        assert!(playfield.is_error(7, 2));

        let _ = playfield.reset_value(1, 2, Option::None);
        assert!(!playfield.is_error(1, 2));
        assert!(!playfield.is_error(7, 2));

        let _ = playfield.reset_value(1, 2, Option::None);
        assert!(!playfield.is_error(1, 2));
        assert!(!playfield.is_error(7, 2));

        let _ = playfield.set_value(1, 6, 0, Option::None);
        assert!(playfield.is_error(6, 0));
        assert!(playfield.is_error(7, 2));

        let _ = playfield.reset_value(7, 2, Option::None);
        assert!(!playfield.is_error(6, 0));
        assert!(!playfield.is_error(7, 2));

        let _ = playfield.set_value(1, 6, 4, Option::None);
        assert!(playfield.is_error(6, 0));
        assert!(playfield.is_error(6, 4));
    }

    #[test]
    fn test_generation() {

        use std::time::Instant;

        let mut playfield = Playfield::new(50, Option::None);
        let now = Instant::now();
        let _ = playfield.generate(58, 42, Option::None, true);
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }

    #[test]
    fn test_format() {
        let arr = Array2D::filled_with(0, 3, 3);
        let serialized = format!("{:?}", arr.as_row_major()).replace(" ", "");
        assert_eq!(serialized, "[0,0,0,0,0,0,0,0,0]");
        let deserialized = Array2D::from_row_major(
            &serialized
            .replace("[", "")
            .replace("]", "")
            .split(',')
            .map(|x| x.parse::<u8>().unwrap())
            .collect::<Vec<u8>>(),
            3,3).unwrap();
        assert_eq!(deserialized, Array2D::from_row_major(&vec![0,0,0,0,0,0,0,0,0], 3, 3).unwrap());
    }

    #[test]
    fn test_solution_counter_empty() {
        let mut playfield = Playfield::new(50, Option::None);
        assert!(playfield.count_solutions_(0, 2) > 1);
    }

    #[test]
    fn test_solution_counter_full() {
        let mut playfield = Playfield::new(50, Option::None);
        let _ = playfield.solve(Option::None);
        assert!(playfield.count_solutions_(0, 2) == 1);
    }

    #[test]
    fn test_solution_counter_partial() {
        let mut playfield = Playfield::new(50, Option::None);
        let _ = playfield.solve(Option::None);

        let mut cursor_random_mask: [usize; 81] = [0; 81];
        for i in 0..81 {
            cursor_random_mask[i] = i;
        }
        cursor_random_mask.shuffle(&mut thread_rng());

        for i in 0..20 {
            let ((row, col), _) = FIELDS[cursor_random_mask[i]];
            let _ = playfield.set_value(0, row, col, Option::None);
        }

        let values_before = playfield.values.clone();
        assert!(playfield.count_solutions_(0, 2) == 1);

        assert_eq!(values_before, playfield.values);
    }

    #[test]
    fn test_solution_counter_partial_2() {
        let mut playfield = Playfield::new(50, Option::None);
        let _ = playfield.set_value(7, 0, 6, Option::None);
        let _ = playfield.set_value(9, 0, 8, Option::None);
        let _ = playfield.set_value(1, 1, 6, Option::None);
        let _ = playfield.set_value(1, 2, 3, Option::None);
        let _ = playfield.set_value(2, 2, 4, Option::None);
        let _ = playfield.set_value(6, 3, 4, Option::None);
        let _ = playfield.set_value(5, 4, 2, Option::None);
        let _ = playfield.set_value(8, 4, 3, Option::None);
        let _ = playfield.set_value(2, 4, 6, Option::None);
        let _ = playfield.set_value(4, 4, 8, Option::None);
        let _ = playfield.set_value(9, 5, 1, Option::None);
        let _ = playfield.set_value(7, 5, 2, Option::None);
        let _ = playfield.set_value(2, 5, 3, Option::None);
        let _ = playfield.set_value(6, 5, 7, Option::None);
        let _ = playfield.set_value(5, 5, 8, Option::None);
        let _ = playfield.set_value(5, 6, 0, Option::None);
        let _ = playfield.set_value(1, 6, 2, Option::None);
        let _ = playfield.set_value(2, 6, 5, Option::None);

        assert!(playfield.count_solutions_(0, 2) > 1);
    }

    #[test]
    fn test_set_value() {
        let playfield = &mut Playfield::new(50, Option::None);

        let _ = playfield.set_value(1, 0, 0, Option::None);
        check(playfield, 0, 0, 1);

        let _ = playfield.set_value(2, 0, 0, Option::None);
        check(playfield, 0, 0, 2);

        let _ = playfield.set_value(0, 0, 0, Option::None);
        check(playfield, 0, 0, 0)
    }

    #[test]
    fn test_solution_counter_partial_3() {
        let mut playfield = Playfield::new(50, Option::None);
        
        let _ = playfield.set_value(1, 0, 0, Option::None);
        let _ = playfield.set_value(2, 1, 3, Option::None);
        let _ = playfield.set_value(3, 3, 1, Option::None);
        let _ = playfield.set_value(4, 5, 4, Option::None);

        assert_eq!(playfield.get_possible_moves((1, 1)).unwrap(), vec![3, 4, 5, 6, 7, 8]);
        assert_eq!(playfield.get_possible_moves((3, 3)).unwrap(), vec![0, 4, 5, 6, 7, 8]);
        assert_eq!(playfield.get_possible_moves((5, 0)).unwrap(), vec![1, 4, 5, 6, 7, 8]);
        assert!(playfield.count_solutions_(0, 2) > 1);
    }

    fn check(playfield:&mut Playfield, row:usize, col:usize, val:usize) {
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
