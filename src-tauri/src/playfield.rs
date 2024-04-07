use std::borrow::BorrowMut;

use crate::engine;
use array2d::Array2D;
use serde::{Serialize, Deserialize};
use serde_json;

use crate::Request;

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CellState {
    Blank,
    Fix,
    Set,
    Error,
    Hint,
}

#[derive(serde::Serialize, Clone)]
struct CellUpdateEvent {
    row: u8,
    col: u8,
    value: u8,
    state: u8,
    notes: [bool; 9],
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
    Editing,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Cell {
    row:usize,
    col:usize,
    value: u8,
    cell_state: CellState,
    solution: Option<u8>,
    notes: [bool; 9],
}

impl Cell {
    fn emit_update_event(&self, request:&Request) {
        let event = CellUpdateEvent {
            row: self.row as u8,
            col: self.col as u8,
            value: self.value,
            state: self.cell_state as u8,
            notes: self.notes,
        };
        request.window.emit(&format!("updateCell-{}-{}", self.row, self.col), event).unwrap();
    }

    fn set_state(&mut self, state:CellState) {
        self.cell_state = state;
    }

    fn set_value(&mut self, value:u8) -> Result<(bool, u8), String> {   
        let changed = value != self.value;
        match self.cell_state {
            CellState::Fix => {
                return Err("Cell is immutable".into());
            },
            CellState::Error => {
                self.value = value;

                if value == 0 {
                    self.set_state(CellState::Blank);
                }
            },
            CellState::Blank => {
                self.value = value;

                if value > 0 {
                    self.set_state(CellState::Set);
                    self.notes.iter_mut().for_each(|note| *note = false);
                }
            }
            CellState::Set => {
                self.value = value;

                if value == 0 {
                    self.set_state(CellState::Blank);
                }
            }
            CellState::Hint => {
                self.value = value;

                if changed {
                    if value == 0 {
                        self.set_state(CellState::Blank);
                    } else {
                        self.set_state(CellState::Set);
                    }
                }
                
            }
        }
        
        Ok((changed, self.value))
    }

    fn toggle_note(&mut self, value:usize) -> Result<(), String> {
        if value == 0 {
            return Err("value must be greater than 0".into())
        }
        match self.cell_state {
            CellState::Blank => {
                self.notes[value - 1] = !self.notes[value - 1];
                Ok(())
            },
            CellState::Error | CellState::Fix | CellState::Set | CellState::Hint => Err("Notes are only allowed on blank cells".into())
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    cells: Vec<Vec<Cell>>,
    state: GameState,
    difficulty: u8,
    timer_seconds: u32,
    seed: u64,
}

impl Game {
    pub fn from_json(string: &str, request:Option<&Request>) -> Game {
        let mut p:Game = serde_json::from_str(string).unwrap();

        request.inspect(|r| p.emit_update_event(r));
        p
    }

    pub fn get_state(&self) -> GameState {
        self.state
    }

    pub fn hint(&mut self, request:Option<&Request>) -> Result<(), String> {
        match self.state {
            GameState::Error | GameState::Solved => {
                return Err("Hints not possible in current state".into());
            },
            GameState::Editing | GameState::Blank | GameState::Running => {}
        };

        engine::hint(&self.get_values()).map(|(row, col)| {
            let cell = &mut self.cells[row][col];
            cell.solution.inspect(|solution| cell.value = *solution);
            cell.set_state(CellState::Hint);
            request.inspect(|r| cell.emit_update_event(r));
        })
    }

    pub fn unhint(&mut self, request:Option<&Request>) {
        self.cells.iter_mut().flatten().for_each(|cell| {
            match cell.cell_state {
                CellState::Blank | CellState::Error | CellState::Fix | CellState::Set => {},
                CellState::Hint => {
                    let _ = cell.set_value(0);
                    request.inspect(|r| cell.emit_update_event(r));
                },
            };
        });
    }

    pub fn new(difficulty:u8, request:Option<&Request>) -> Game {
        let mut cells:Vec<Vec<Cell>> = Vec::new();
        
        for row in 0..9 {
            let mut row_cells: Vec<Cell> = Vec::new();
            for col in 0..9 {
                row_cells.push(Cell {
                    row,
                    col,
                    value: 0,
                    cell_state: CellState::Blank,
                    solution: Option::None,
                    notes: [false; 9],
                });
            }
            cells.push(row_cells);
        }
        let mut p = Game { 
            cells,
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

    pub fn toggle_note(&mut self, row:usize, col:usize, value:usize, request:Option<&Request>) -> Result<(), String> {
        if value < 1 || value > 9 {
            return Err("Note value must be between 1 and 9".into());
        }
        
        let result = self.cells[row][col].toggle_note(value);
        request.inspect(|r| self.cells[row][col].emit_update_event(*r));

        result
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    pub fn get_clue_count(&self) -> u8 {
        self.cells.iter().flatten().filter(|s| match s.cell_state {
            CellState::Fix | CellState::Error | CellState::Set | CellState::Hint => true,
            CellState::Blank => false,            
        }).count() as u8
    }

    pub fn increment_timer(&mut self) -> Result<u32, String> {
        match self.state {
            GameState::Editing | GameState::Blank | GameState::Solved => {},
            GameState::Error | GameState::Running => self.timer_seconds += 1,      
        };
        
        Ok(self.timer_seconds)
    }

    pub fn reset(&mut self, request:Option<&Request>) -> Result<GameState, String> {
        self.cells.iter_mut().flatten().for_each(|cell_ref| {
            let _ = cell_ref.set_value(0).inspect(|(changed, _)| {
                if *changed {
                    request.inspect(|r| cell_ref.emit_update_event(r));
                }
            });
        });
        Ok(self.state)
    }

    pub fn set_value(&mut self, value:u8, row:usize, col:usize, request:Option<&Request>) -> Result<u8, String> {
        match self.state {
            GameState::Blank => self.state = GameState::Running,
            GameState::Solved => return Err("Already solved".into()),
            GameState::Editing | GameState::Error | GameState::Running => {}        
        };
        
        self.cells[row][col].set_value(value).map(|(changed, new_value)| {
            if changed {
                self.update_states(request)
            }
            new_value
        })
    }

    pub fn get_value(&self, row:usize, col:usize) -> u8 {
        self.cells[row][col].value
    }

    pub fn get_values(&self) -> Array2D<u8> {
        let mut a = Array2D::filled_with(0, 9, 9);
        for row in 0..9 {
            for col in 0..9 {
                a[(row, col)] = self.cells[row][col].value;
            }
        }
        a
    }

    fn is_error(&self, row:usize, col:usize) -> bool {
        let cell = &self.cells[row][col];
        match cell.solution.as_ref() {
            None => {
                if cell.value == 0 {
                    return false;
                }

                for i in 0..9 {
                    if i != col && cell.value == self.get_value(row, i) {
                        return true;
                    }
                    if i != row && cell.value == self.get_value(i, col) {
                        return true;
                    }
                    let r = i / 3 + 3*(row/3);
                    let c = i % 3 + 3*(col/3);
                    if !(r == row && c == col) && cell.value == self.get_value(r, c) {
                        return true;
                    }
                }
                return false;
            },
            Some(solution) => cell.value > 0 && cell.value != *solution
        }
    }

    pub fn generate(&mut self, difficulty:u8, seed:u64, request:Option<&Request>, fix_result:bool) -> Result<GameState, String> {
        self.difficulty = difficulty;
        self.seed = seed;

        let result = engine::generate(&self.get_values(), seed, difficulty);
        if result.is_err() {
            return Err(result.unwrap_err());
        }
        let (clues, solution) = result.unwrap();


        self.cells.iter_mut().flatten().for_each(|cell| {
            let _ = cell.set_value(clues[(cell.row.into(), cell.col.into())]);
            if fix_result {
                cell.solution = Option::Some(solution[(cell.row.into(), cell.col.into())]);
            }
        });

        if fix_result {
            let _ = self.start_solving();
        }
        
        request.inspect(|r| self.emit_update_event(r));
        Ok(self.state)
    }

    pub fn start_solving(&mut self) -> Result<(), String> {
        if self.count_solutions(2) > 1 {
            return Err("Must have a unique solution to start solving".into());
        }

        self.cells.iter_mut().flatten().for_each(|cell| {
            if cell.value > 0 {
                cell.set_state(CellState::Fix);
            }
        });

        let result = engine::solve(&self.get_values(), Option::None);
        if result.is_err() {
            return Err(result.unwrap_err());
        }

        let solution = result.unwrap();
        self.cells.iter_mut().flatten().for_each(|cell| {
            cell.solution = Option::Some(solution[(cell.row.into(), cell.col.into())])
        });

        self.state = GameState::Running;
        Ok(())
    }
    
    pub fn emit_update_event(&mut self, request:&Request) {
        self.cells.iter().flatten().for_each(|cell| cell.emit_update_event(request));
        self.emit_update_game_event(request);
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

    pub fn solve(&mut self, request:Option<&Request>) -> Result<GameState, String> {   
        match self.state {
            GameState::Solved => {
                return Err("Already solved".into());
            },
            GameState::Error => {
                return Err("Conflict detected, can't solve".into())
            },
            GameState::Editing | GameState::Blank | GameState::Running => {}
        };

        engine::solve(&self.get_values(), Option::None).map(|values| {
            for row in 0..9 {
                for col in 0..9 {
                    let cell = self.cells[row][col].borrow_mut();
                    let value = values[(row, col)];
                    let _ = cell.set_value(value);
                    cell.solution = Option::Some(value);
                }
            }
            self.state = GameState::Solved;
            request.inspect(|r| self.emit_update_event(r));
            self.state
        })
    }

    fn update_states(&mut self, request:Option<&Request>) {
        let x = self.cells.iter().flatten().map(|cell| {
            let state = match cell.cell_state {
                CellState::Fix | CellState::Hint => cell.cell_state,
                CellState::Blank | CellState::Set | CellState::Error => {
                    if self.is_error(cell.row.into(), cell.col.into()) {
                        CellState::Error
                    } else {
                        if cell.value > 0 {
                            CellState::Set
                        } else {
                            CellState::Blank
                        }
                    }
                }
            };
            
            ((cell.row, cell.col), state)
        }).collect::<Vec<((usize, usize), CellState)>>();
        
        let cell_states = x.iter().map(|((r,c), state)| {
            let cell = self.cells[*r][*c].borrow_mut();
            cell.set_state(*state);
            request.inspect(|r| cell.emit_update_event(r));
            *state
        }).collect::<Vec<CellState>>();

        let has_any_errors = cell_states.iter().any(|state| {
            match state {
                CellState::Error => true,
                CellState::Blank | CellState::Fix | CellState::Set | CellState::Hint => false,
            }
        });
        let all_empty = !has_any_errors && cell_states.iter().all(|state| {
            match state {
                CellState::Blank => true,
                CellState::Error | CellState::Fix | CellState::Set | CellState::Hint => false,
            }
        });
        let all_set = !has_any_errors && cell_states.iter().all(|state| {
            match state {
                CellState::Fix | CellState::Set => true,
                CellState::Error | CellState::Blank | CellState::Hint => false,
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

    pub fn count_solutions(&mut self, limit:u8) -> u8 {
        if self.state == GameState::Error {
            return 0;
        }
        engine::count_solutions(&self.get_values(), limit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_is_error() {
        let mut playfield = Game::new(50, Option::None);
        let _ = playfield.set_value(1, 1, 2, Option::None);
        assert!(!playfield.is_error(1, 2));
        assert!(!playfield.is_error(7, 2));

        let _ = playfield.set_value(1, 7, 2, Option::None);
        assert_eq!(playfield.cells[1][2].cell_state, CellState::Error);
        assert!(playfield.is_error(7, 2));

        let _ = playfield.set_value(0, 1, 2, Option::None);
        assert!(!playfield.is_error(1, 2));
        assert!(!playfield.is_error(7, 2));

        let _ = playfield.set_value(0, 1, 2, Option::None);
        assert!(!playfield.is_error(1, 2));
        assert!(!playfield.is_error(7, 2));

        let _ = playfield.set_value(1, 6, 0, Option::None);
        assert!(playfield.is_error(6, 0));
        assert!(playfield.is_error(7, 2));

        let _ = playfield.set_value(0, 7, 2, Option::None);
        assert!(!playfield.is_error(6, 0));
        assert!(!playfield.is_error(7, 2));

        let _ = playfield.set_value(1, 6, 4, Option::None);
        assert!(playfield.is_error(6, 0));
        assert!(playfield.is_error(6, 4));
    }

    #[test]
    fn test_generation() {

        use std::time::Instant;

        let mut playfield = Game::new(50, Option::None);
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
}
