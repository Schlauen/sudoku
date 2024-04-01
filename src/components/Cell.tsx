import { MutableRefObject, forwardRef, useEffect, useImperativeHandle, useRef, useState } from "react";
import MiniCell from "./MiniCell";
import { AppState, useStore } from '../store';
import { listen } from '@tauri-apps/api/event';
import { CellUpdateEvent, incrementCellValue } from "../Interface";

const State = {
    Blank: 0,
    Fix: 1,
    Set: 2,
    Error: 3,
}

interface Props {
    row: number;
    col: number;
}

interface Cell {
    value: number,
    state: number,
}

function getClassName(state:number, showErrors:boolean, focus:boolean) {
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
        if (showErrors) {
            className = 'cell error';
        } else {
            className = 'cell enabled';
        }
    }
    else {
        className = '';
    }

    if (focus) {
        className += ' selected';
    }

    return className;
}

function toValue(value:number, miniCells:Array<MutableRefObject<any>>) {
    if (value > 0) {
        return value.toString();
    }
    
    return Array(9).fill(undefined).map((_,i) => (<MiniCell digit={i+1} ref={miniCells[i]} />));
}

const Cell = forwardRef(({ row, col }: Props, ref) => {
    const [state, setState] = useState(State.Blank);
    const showErrors = useStore(state => state.showError);
    const appState = useStore(state => state.appState);
    const [value, setValue] = useState(0);
    const [focus, setFocus] = useState(false);
    const onError = useStore(state => state.changeMessage);
    const includeCounts = useStore(state => state.appState) == AppState.Editing; 

    const miniCells = Array(9).fill(undefined).map(_ => useRef<any>(null));

    useImperativeHandle(ref, () => {
        return {
            focus:setFocus,
            toggleNote:(digit:number) => {
                let current = miniCells[digit - 1].current;
                if (current) {
                    current.toggle();
                }
            }
        };
    });

    useEffect(() => {
        const unlisten = listen<CellUpdateEvent>('updateCell-'+row+'-'+col, event => {
            console.log(event.payload);
            setState(event.payload.state);
            setValue(event.payload.value);
        });
    
        return () => {
            unlisten.then(f => f());
        };
    });

    return (
        <div 
            id={row  + "," + col }
            key={row + "," + col }
            className={getClassName(state, showErrors || appState == AppState.Editing, focus)}
            onClick={() => incrementCellValue(row, col, includeCounts, includeCounts, onError)}
        >
            {toValue(value, miniCells)}
        </div>
    );
});

export default Cell;
