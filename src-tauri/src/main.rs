// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod playfield;
mod engine;
use std::sync::Mutex;
use playfield::Game;
use tauri::Window;

#[derive(serde::Serialize, Clone, Copy)]
struct Cell {
    value: u8,
    state: u8,
}

// Mutex for interior mutability
struct PlayfieldState {
    playfield: Mutex<Game>,
}

struct Request {
    window: Window,
    include_clue_count: bool,
    include_solution_count: bool,
}

#[tauri::command]
fn serialize(
    state: tauri::State<'_, PlayfieldState>,
) -> Result<String,String> {
    let playfield = state.playfield.lock().unwrap();
    playfield.to_json().map_err(|_| "serialization failed".into())
}

#[tauri::command]
fn toggle_note(
    state: tauri::State<'_, PlayfieldState>,
    window: Window,
    row:usize, col:usize, 
    value:usize
) -> Result<(),String> {
    let mut playfield = state.playfield.lock().unwrap();
    playfield.toggle_note(
        row, col, value,
        Option::Some(&Request {
            window,
            include_clue_count:false,
            include_solution_count:false,
        }), 
    )
}

#[tauri::command]
fn increment_timer(
    state: tauri::State<'_, PlayfieldState>,
) -> Result<u32,String> {
    let mut playfield = state.playfield.lock().unwrap();
    playfield.increment_timer()
}

#[tauri::command]
fn hint(
    state: tauri::State<'_, PlayfieldState>,
    window: Window,
    include_clue_count: bool,
    include_solution_count: bool,
) -> Result<(),String> {
    let mut playfield = state.playfield.lock().unwrap();
    playfield.hint(Option::Some(&Request {
        window,
        include_clue_count,
        include_solution_count,
    }))
}

#[tauri::command]
fn unhint(
    state: tauri::State<'_, PlayfieldState>,
    window: Window,
    include_clue_count: bool,
    include_solution_count: bool,
) -> Result<(),String> {
    let mut playfield = state.playfield.lock().unwrap();
    playfield.unhint(Option::Some(&Request {
        window,
        include_clue_count,
        include_solution_count,
    }));
    Ok(())
}

#[tauri::command]
fn deserialize(
    state: tauri::State<'_, PlayfieldState>,
    window: Window,
    msg: String,
    include_clue_count: bool,
    include_solution_count: bool,
) -> Result<(), String> {
    let mut playfield = state.playfield.lock().unwrap();
    *playfield = Game::from_json(&msg, Option::Some(&Request {
        window,
        include_clue_count,
        include_solution_count,
    }));
    Ok(())
}

#[tauri::command]
fn generate(
    state: tauri::State<'_, PlayfieldState>,
    window: Window,
    difficulty: u8, 
    seed: u64,
    include_clue_count: bool,
    include_solution_count: bool,
    fix_result: bool,
) -> Result<(), String> {
    state.playfield.lock().unwrap().generate(
        difficulty, 
        seed, 
        Option::Some(&Request {
            window,
            include_clue_count,
            include_solution_count,
        }),
        fix_result,
    ).map(|_| ())
}

#[tauri::command]
fn fix_current(
    state: tauri::State<'_, PlayfieldState>, 
    window: Window,
    include_clue_count: bool,
    include_solution_count: bool,
) -> Result<(), String> {
    let mut playfield = state.playfield.lock().unwrap();
    let result = playfield.start_solving();
    playfield.emit_update_event(
        &Request {
            window,
            include_clue_count,
            include_solution_count,
        }
    );
    result
}

#[tauri::command]
fn trigger_update(
    state: tauri::State<'_, PlayfieldState>,
    window: Window,
    include_clue_count: bool,
    include_solution_count: bool,
) -> Result<(), String> {
    state.playfield.lock().unwrap().emit_update_event(
        &Request {
            window,
            include_clue_count,
            include_solution_count,
        }
    );
    Ok(())
}

#[tauri::command]
fn increment_value(
    state:tauri::State<'_, PlayfieldState>,
    window: Window,
    row:usize, col:usize,
    include_clue_count: bool,
    include_solution_count: bool,
) -> Result<(), String> {
    let mut playfield = state.playfield.lock().unwrap();
    let value = playfield.get_value(row, col);
    playfield.set_value(
        (value + 1) % 10, 
        row, 
        col, 
        Option::Some(&Request {
            window,
            include_clue_count,
            include_solution_count,
        })
    ).map(|_| ())
}

#[tauri::command]
fn set_value(
    state:tauri::State<'_, PlayfieldState>,
    window: Window,
    row:usize, col:usize, value:u8,
    include_clue_count: bool,
    include_solution_count: bool,
) -> Result<(), String> {
    let mut playfield = state.playfield.lock().unwrap();
    playfield.set_value(
        value, 
        row, 
        col, 
        Option::Some(&Request {
            window,
            include_clue_count,
            include_solution_count,
        })
    ).map(|_| ())
}

#[tauri::command]
fn reset(
    state:tauri::State<'_, PlayfieldState>,
    window: Window,
    include_clue_count: bool,
    include_solution_count: bool,
    hard: bool, // if this is set to true, a completely new game is created
) -> Result<(), String> {
    let mut playfield = state.playfield.lock().unwrap();
    if hard {
        *playfield = Game::new(0, Option::Some(&Request {
            window,
            include_clue_count,
            include_solution_count,
        }));
        Ok(())
    } else {
        playfield.reset(Option::Some(&Request {
            window,
            include_clue_count,
            include_solution_count,
        })).map(|_| ())
    }
}

#[tauri::command]
fn solve(
    state:tauri::State<'_, PlayfieldState>,
    window: Window,
    include_clue_count: bool,
    include_solution_count: bool,
) -> Result<(), String> {
    let mut playfield = state.playfield.lock().unwrap();
    playfield.solve(Option::Some(&Request {
        window,
        include_clue_count,
        include_solution_count,
    })).map(|_| ())
}

fn main() {
    tauri::Builder::default()
        .manage(PlayfieldState {
            playfield: Mutex::new(Game::new(0, Option::None)),
        })
        .invoke_handler(tauri::generate_handler![
            increment_value,
            generate,
            set_value,
            reset,
            solve,
            serialize, deserialize,
            increment_timer,
            trigger_update,
            fix_current,
            toggle_note,
            hint, unhint,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
