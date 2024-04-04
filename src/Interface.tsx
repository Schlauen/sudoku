import { invoke } from "@tauri-apps/api";
import { EventCallback, UnlistenFn, listen } from "@tauri-apps/api/event";

export interface CellUpdateEvent {
    row: number,
    col: number,
    value: number,
    state: number,
    notes: [boolean],
}

export interface GameUpdateEvent {
    state: number,
    clue_count: number,
    solution_count: number,
}

export const GameState = {
    Blank: 0,
    Running: 1,
    Solved: 2,
    Error: 3,
}

export async function serialize(
    onSuccess: (value:string) => void, 
    onError: (msg:string) => void
) {
    invoke<string>('serialize').then(onSuccess).catch(onError);
}

export function incrementTimer(
    onSuccess: (seconds:number) => void,
    onError: (msg:string) => void
) {
    invoke<number>('increment_timer').then(onSuccess).catch(onError);
}

export function deserialize(
    content:string,
    includeClueCount:boolean,
    includeSolutionCount:boolean,
    onError: (msg:string) => void
) {
    invoke('deserialize', {
        msg:content,
        includeClueCount: includeClueCount,
        includeSolutionCount: includeSolutionCount
    }).then(_ => {}).catch(onError)
}

export function setCellValue(
    digit:number, 
    row:number, 
    col:number, 
    includeClueCount:boolean,
    includeSolutionCount:boolean,
    onError: (msg:string) => void
) {
    invoke(
        'set_value', 
        {
            row:row,
            col:col,
            value:digit,
            includeClueCount: includeClueCount,
            includeSolutionCount: includeSolutionCount,
        }
    ).then(_ => {})
    .catch(onError);
}

export function incrementCellValue(
    row:number, 
    col:number,
    includeClueCount:boolean,
    includeSolutionCount:boolean,
    onError: (msg:string) => void
) {
    invoke(
        'increment_value', 
        {
            row: row,
            col: col,
            includeClueCount: includeClueCount,
            includeSolutionCount: includeSolutionCount,
        }
    ).then(_ => {})
    .catch(onError);
}

export function solve(
    includeClueCount:boolean,
    includeSolutionCount:boolean,
    onError: (msg:string) => void
) {
    invoke(
        'solve',
        {
            includeClueCount: includeClueCount,
            includeSolutionCount: includeSolutionCount,
        }
    ).then((_) => {}).catch(onError);
}

export function reset(
    includeClueCount:boolean,
    includeSolutionCount:boolean,
    hard:boolean,
    onError: (msg:string) => void
) {
    invoke(
        'reset',
        {
            includeClueCount: includeClueCount,
            includeSolutionCount: includeSolutionCount,
            hard: hard,
        }
    ).then((_) => {}).catch(onError);
}

export function generate(
    difficulty:number, 
    seed:number,
    includeClueCount:boolean,
    includeSolutionCount:boolean,
    fixResult:boolean,
    onSuccess: () => void, 
    onError: (msg:string) => void
) {
    invoke('generate', {
        difficulty: difficulty,
        seed: seed,
        includeClueCount: includeClueCount,
        includeSolutionCount: includeSolutionCount,
        fixResult: fixResult,
      }).then(onSuccess)
      .catch(onError)
}

export function triggerUpdate(
    includeClueCount:boolean,
    includeSolutionCount:boolean,
    onError: (msg:string) => void
) {
    console.log('update triggered by frontend');
    invoke('trigger_update', {
        includeClueCount: includeClueCount,
        includeSolutionCount: includeSolutionCount,
      }).then(_ => {})
      .catch(onError)
}

export function fixResult(
    includeClueCount:boolean,
    includeSolutionCount:boolean,
    onError: (msg:string) => void
) {
    invoke('fix_current', {
        includeClueCount: includeClueCount,
        includeSolutionCount: includeSolutionCount,
      }).then(_ => {})
      .catch(onError)
}

export function toggleNote(
    row:number, col:number, value:number,
    onError: (msg:string) => void
) {
    invoke('toggle_note', {
        row: row,
        col: col,
        value: value
      }).then(_ => {})
      .catch(onError)
}

export function onUpdateCell(row:number, col:number, onTrigger: (event:CellUpdateEvent) => void): Promise<UnlistenFn> {
    return listen<CellUpdateEvent>('updateCell-' + row + '-' + col, event => onTrigger(event.payload));
}

export function onUpdateGame(onTrigger: (event:GameUpdateEvent) => void): Promise<UnlistenFn> {
    return listen<GameUpdateEvent>('updateGame', event => onTrigger(event.payload));
}