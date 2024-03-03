// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod state;
use std::sync::Mutex;
use state::{CellState, Playfield};

#[derive(serde::Serialize)]
struct Cell {
    value: u8,
    state: u8,
    game_state: u8,
}

// here we use Mutex to achieve interior mutability
struct PlayfieldState {
    playfield: Mutex<Playfield>,
    show_errors: Mutex<bool>,
}

#[derive(Clone, Copy)]
enum EnhancedCellState {
    Blank,
    Fix,
    Set,
    Error,
    Unknown,
}

fn enhance_cell_state(show_errors:bool, cell_state:CellState) -> EnhancedCellState {
    match cell_state {
        CellState::Blank => EnhancedCellState::Blank,
        CellState::Fix => EnhancedCellState::Fix,
        CellState::Set => EnhancedCellState::Set,
        CellState::Error => {
            if show_errors {
                EnhancedCellState::Error
            } else {
                EnhancedCellState::Unknown
            }
        }
    }
}

#[tauri::command]
fn get_cell_state(
    state: tauri::State<'_, PlayfieldState>,
    row:usize, col:usize
) -> Result<Cell,String> {
    let playfield = state.playfield.lock().unwrap();
    let value = playfield.get_value(row, col);

    Ok(Cell{
        value,
        state:enhance_cell_state(
            *state.show_errors.lock().unwrap(), 
            playfield.get_cell_state(row, col)
        ) as u8,
        game_state:playfield.get_game_state() as u8,
    })
}

#[tauri::command]
fn get_game_state(
    state: tauri::State<'_, PlayfieldState>
) -> Result<u8, String> {
    let playfield = state.playfield.lock().unwrap();
    Ok(playfield.get_game_state() as u8)
}

#[tauri::command]
fn generate(
    state: tauri::State<'_, PlayfieldState>,
    difficulty: u8,
) -> Result<u8, String> {
    state.playfield.lock().unwrap().generate(difficulty).map(|game_state| game_state as u8)
}

#[tauri::command]
fn toggle_show_errors(
    state: tauri::State<'_, PlayfieldState>,
) -> Result<bool,String> {
    let mut show_errors = state.show_errors.lock().unwrap();
    *show_errors = !(*show_errors);
    Ok(*show_errors)
}

#[tauri::command]
fn increment_value(
    state:tauri::State<'_, PlayfieldState>,
    row:usize, col:usize
) -> Result<Cell, String> {
    let mut playfield = state.playfield.lock().unwrap();
    let value = playfield.get_value(row, col);
    playfield.set_value((value + 1) % 10, row, col).map(|new_value| Cell {
        value: new_value,
        state: enhance_cell_state(
            *state.show_errors.lock().unwrap(), 
            playfield.get_cell_state(row, col)
        ) as u8,
        game_state:playfield.get_game_state() as u8,
    })
}

#[tauri::command]
fn set_value(
    state:tauri::State<'_, PlayfieldState>,
    row:usize, col:usize, value:u8
) -> Result<Cell, String> {
    let mut playfield = state.playfield.lock().unwrap();
    playfield.set_value(value, row, col).map(|new_value| Cell {
        value: new_value,
        state: enhance_cell_state(
            *state.show_errors.lock().unwrap(), 
            playfield.get_cell_state(row, col)
        ) as u8,
        game_state:playfield.get_game_state() as u8,
    })
}

#[tauri::command]
fn reset(
    state:tauri::State<'_, PlayfieldState>
) -> Result<u8, String> {
    let mut playfield = state.playfield.lock().unwrap();
    playfield.reset().map(|game_state| game_state as u8)
}

#[tauri::command]
fn solve(
    state:tauri::State<'_, PlayfieldState>
) -> Result<u8, String> {
    let mut playfield = state.playfield.lock().unwrap();
    playfield.solve().map(|game_state| game_state as u8)
}

fn main() {
    tauri::Builder::default()
        .manage(PlayfieldState {
            playfield: Mutex::new(Playfield::new()),
            show_errors: Mutex::new(false),
        })
        .invoke_handler(tauri::generate_handler![
            increment_value,
            generate,
            get_cell_state,
            set_value,
            reset,
            solve,
            get_game_state,
            toggle_show_errors,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
