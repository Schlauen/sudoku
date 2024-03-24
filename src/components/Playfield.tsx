import GenericBox from './GenericBox'
import Cell from './Cell'
import { forwardRef, useImperativeHandle, useRef, useState } from 'react'
import useEventListener from '@use-it/event-listener';


interface Key {
    key:string;
}

const Playfield = forwardRef((_, ref) => {
    const [focus, setFocus] = useState(40);
    const [spacePressed, setSpacePressed] = useState(false);
    
    const cells = Array(81).fill(undefined).map(_ => useRef<any>(null));

    const update = () => cells.forEach(c => c.current.update());
    
    useImperativeHandle(ref, () => ({update}));

    const setFocusTo = (newFocus:number) => {
        if (focus < 0) {
            newFocus = 40;
        } else {
            cells[focus].current.focus(false);
        }
        setFocus(newFocus);
        console.log(newFocus);
        if (newFocus < 0) {
            return;
        }
        cells[newFocus].current.focus(true);
    }
    const focusRow = () => Math.floor(focus / 9);
    const focusCol = () => focus % 9;
    const digitPessed = (key:string, digit:number) => key === digit.toString();
    const toggleNote = (digit:number) => cells[focus].current.toggleNote(digit);
    const setValue = (digit:number) => cells[focus].current.setValue(digit);

    function keyDownHandler({key}:Key) {
        if (key === "Control") setSpacePressed(true);
    };

    function keyUpHandler({key}:Key) {
        console.log(key);
        if (key === "Control") setSpacePressed(false);

        if (key === "ArrowDown" || key === "s") {
            let newFocus = focus + (focusRow() >= 8 ? 0 : 9);
            setFocusTo(newFocus);
        } else if (key === "ArrowUp" || key === "w") {
            let newFocus =  Math.max(focusCol(), focus - 9);
            setFocusTo(newFocus);
        } else if (key === "ArrowLeft" || key === "a") {
            let newFocus =  focus - (focusCol() <= 0 ? 0 : 1);
            setFocusTo(newFocus);
        } else if (key === "ArrowRight" || key === "d") {
            let newFocus =  focus + (focusCol() >= 8 ? 0 : 1);
            setFocusTo(newFocus);
        } else if (key === "Escape") {
            setFocusTo(-1);
        }
        else if (spacePressed && digitPessed(key, 1)) toggleNote(1);
        else if (spacePressed && digitPessed(key, 2)) toggleNote(2);
        else if (spacePressed && digitPessed(key, 3)) toggleNote(3);
        else if (spacePressed && digitPessed(key, 4)) toggleNote(4);
        else if (spacePressed && digitPessed(key, 5)) toggleNote(5);
        else if (spacePressed && digitPessed(key, 6)) toggleNote(6);
        else if (spacePressed && digitPessed(key, 7)) toggleNote(7);
        else if (spacePressed && digitPessed(key, 8)) toggleNote(8);
        else if (spacePressed && digitPessed(key, 9)) toggleNote(9);
        else if (digitPessed(key, 1)) setValue(1);
        else if (digitPessed(key, 2)) setValue(2);
        else if (digitPessed(key, 3)) setValue(3);
        else if (digitPessed(key, 4)) setValue(4);
        else if (digitPessed(key, 5)) setValue(5);
        else if (digitPessed(key, 6)) setValue(6);
        else if (digitPessed(key, 7)) setValue(7);
        else if (digitPessed(key, 8)) setValue(8);
        else if (digitPessed(key, 9)) setValue(9);
        else if (key === "Delete" || digitPessed(key, 0)) setValue(0);
    }

    useEventListener('keyup', keyUpHandler, document);
    useEventListener('keydown', keyDownHandler, document);

    return (
        <div id='playfield'>
            <GenericBox
                parentRow={0}
                parentCol={0}
                keyPrefix='playfield:'
                childFactory={
                    (boxRow,boxCol) => <GenericBox 
                        parentRow={boxRow} 
                        parentCol={boxCol}
                        keyPrefix='box:'
                        childFactory={
                            (row, col) => <Cell 
                                row={row} col={col}
                                ref={cells[row*9 + col]}
                            />
                        }
                    />
                }
            />
        </div>
    )
});

export default Playfield
