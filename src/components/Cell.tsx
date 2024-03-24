import { forwardRef, useImperativeHandle, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import MiniCell from "./MiniCell";
import { useStore } from '../store';

const State = {
    Blank: 0,
    Fix: 1,
    Set: 2,
    Error: 3,
    Unknown: 4,
}

interface Props {
    row: number;
    col: number;
}

interface CellState {
    value:number;
    state:number;
    game_state:number;
}

const Cell = forwardRef(({ row, col }: Props, ref) => {
    const [state, setState] = useState(State.Blank);
    const [value, setValue] = useState(0);
    const [focus, setFocus] = useState(false);

    const miniCells = Array(9).fill(undefined).map(_ => useRef<any>(null));

    const getClassName = () => {
        let className:string;
        if (state == State.Blank) {
            className = 'cell box enabled';
        }
        else if (state == State.Fix) {
            className = 'cell disabled';
        }
        else if (state == State.Set) {
            className = 'cell enabled';
        }
        else if (state == State.Error) {
            className = 'cell error';
        }
        else if (state == State.Unknown) {
            className = 'cell enabled';
        }
        else {
            className = '';
        }

        if (focus) {
            className += ' selected';
        }

        return className;
    }

    const changeGameState = useStore((state) => state.changeGameState);
    const onError = useStore((state) => state.changeMessage);

    const updateCell = (cell:CellState) => {
        setValue(cell.value);
        setState(cell.state);
        changeGameState(cell.game_state);
    }

    const update = () => invoke<CellState>('get_cell_state', {row:row, col:col})
        .then(updateCell)
        .catch(onError);
    const toggleNote = (digit:number) => {
        console.log('toggle-note', miniCells[digit - 1]);
        let current = miniCells[digit - 1].current;
        if (current) {
            current.toggle();
        }
    }
    const setAndUpdate = (digit:number) => invoke<CellState>(
        'set_value', 
        {row:row, col:col, value:digit}
    ).then(updateCell)
    .catch(onError);

    useImperativeHandle(ref, () => {
        return {
            update:update,
            focus:setFocus,
            toggleNote:toggleNote,
            setValue:setAndUpdate,
        };
    });

    const toValue = (value:number) => {
        if (value > 0) {
            return value.toString();
        }
        
        return Array(9).fill(undefined).map((_,i) => (<MiniCell digit={i+1} ref={miniCells[i]} />))
    }

    return (
        <div 
            id={row  + "," + col }
            key={row + "," + col }
            className={getClassName()}
            onClick={
                () => invoke<CellState>(
                    "increment_value", 
                    {row:row, col:col}
                )
                .then(updateCell)
                .catch(onError)
            }
        >
            {toValue(value)}
        </div>
    );
});

export default Cell;
