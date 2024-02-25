import { PlayfieldState } from "wasm-sudoku";
import Button from "./Button";
import Playfield from "./Playfield";
import Footer from "./Footer";
import GenericBox from "./GenericBox";

const CellState = {
    Blank: 0,
    Fix: 1,
    Set: 2,
    Error: 3,
}
const state = PlayfieldState.new();
var selectedCell = {
    row: 0,
    col: 0
}
var spacePressed = false;
console.log('new playfield');

document.addEventListener('keydown', (e) => {
    if (e.code === "Space") spacePressed = true;
});

document.addEventListener('keyup', (e) => { 
    if (e.code === "Space") spacePressed = false;

    if (e.code === "ArrowUp" || e.code === "KeyW") {
        let oldRow = selectedCell.row;
        selectedCell.row = Math.max(0, selectedCell.row - 1);
        updateCell(oldRow, selectedCell.col);
        updateSelectedCell();
    }
    else if (e.code === "ArrowDown" || e.code === "KeyS") {
        let oldRow = selectedCell.row;
        selectedCell.row = Math.min(8, selectedCell.row + 1);
        updateCell(oldRow, selectedCell.col);
        updateSelectedCell();
    }
    else if (e.code === "ArrowLeft" || e.code === "KeyA") {
        let oldCol = selectedCell.col;
        selectedCell.col = Math.max(0, selectedCell.col - 1);
        updateCell(selectedCell.row, oldCol);
        updateSelectedCell();
    }
    else if (e.code === "ArrowRight" || e.code === "KeyD") {
        let oldCol = selectedCell.col;
        selectedCell.col = Math.min(8, selectedCell.col + 1);
        updateCell(selectedCell.row, oldCol);
        updateSelectedCell();
    }
    else if (spacePressed && digitPessed(e, 1)) toggleNote(1);
    else if (spacePressed && digitPessed(e, 2)) toggleNote(2);
    else if (spacePressed && digitPessed(e, 3)) toggleNote(3);
    else if (spacePressed && digitPessed(e, 4)) toggleNote(4);
    else if (spacePressed && digitPessed(e, 5)) toggleNote(5);
    else if (spacePressed && digitPessed(e, 6)) toggleNote(6);
    else if (spacePressed && digitPessed(e, 7)) toggleNote(7);
    else if (spacePressed && digitPessed(e, 8)) toggleNote(8);
    else if (spacePressed && digitPessed(e, 9)) toggleNote(9);
    else if (digitPessed(e, 1)) setValue(1);
    else if (digitPessed(e, 2)) setValue(2);
    else if (digitPessed(e, 3)) setValue(3);
    else if (digitPessed(e, 4)) setValue(4);
    else if (digitPessed(e, 5)) setValue(5);
    else if (digitPessed(e, 6)) setValue(6);
    else if (digitPessed(e, 7)) setValue(7);
    else if (digitPessed(e, 8)) setValue(8);
    else if (digitPessed(e, 9)) setValue(9);
    else if (e.code === "Delete" || digitPessed(e, 0)) setValue(0);
});

function toggleNote(value) {
    let note = document.getElementById(selectedCell.row + "," + selectedCell.col + "," + (value-1));
    if (!note) {
        return;
    }
    if (note.innerHTML === '') {
        note.innerHTML = value;
    } else {
        note.innerHTML = '';
    }
}

function setValue(value) {
    state.set_value(value, selectedCell.row, selectedCell.col);
    updateSelectedCell();
}

function digitPessed(event, digit) {
    return event.code === "Digit" + digit || event.code === "Numpad" + digit;
}

function getCell(row, col) {
    let id = row + "," + col;
    return document.getElementById(id);
}

function updateCells() {
    for (let i = 0; i < 9; i += 1) {
        for (let j = 0; j < 9; j += 1) {
            updateCell(i, j);
        }   
    }
}

function updateSelectedCell() {
    updateCell(selectedCell.row, selectedCell.col);
}

function updateCell(row, col) {
    let cell = getCell(row, col);
    let nextCellState = state.get_cell_state(row, col);
    let currentCellState = cell.cellState;
    let updateValue = () => cell.innerHTML = state.get_value(row, col) || "";

    if (nextCellState == CellState.Blank) {
        cell.className = 'cell box enabled';

        if (currentCellState != CellState.Blank) {
            cell.innerHTML = '';
            for (let i = 0; i < 9; i += 1) {
                let miniCell = document.createElement("div");
                miniCell.id = cell.id + "," + i;
                miniCell.className = "mini-cell";
                miniCell.innerHTML = '';
                cell.appendChild(miniCell);
            }
        }
        
        //cell.className = 'cell box enabled';
    }
    else if (nextCellState == CellState.Fix) {
        cell.className = 'cell disabled';
        updateValue();
    }
    else if (nextCellState == CellState.Set) {
        cell.className = 'cell enabled';
        updateValue();
    }
    else if (nextCellState == CellState.Error) {
        cell.className = 'cell error';
        updateValue();
    }

    if (selectedCell.row == row && selectedCell.col == col) {
        cell.className += ' selected';
    }
    cell.cellState = nextCellState;
}

function Sidebar() {
    let sidebar = document.createElement("div");
    sidebar.id = 'sidebar';

    let d3 = document.createElement("div");
    d3.className = 'checkboxcontainer';
    let d4 = document.createElement("label");
    d4.innerHTML = 'show errors';
    d4.className = 'spring';
    d3.appendChild(d4);
    sidebar.appendChild(d3);

    let checkbox = document.createElement("div");
    checkbox.className = 'custom-checkbox';
    d3.appendChild(checkbox);
    let tick = document.createElement("div");
    tick.className = 'tick';
    let checked = false;
    checkbox.onclick = () => {
        state.toggle_show_errors();
        updateCells();
        if (checked) {
            checked = false;
            checkbox.removeChild(tick);
        } else {
            checked = true;
            checkbox.appendChild(tick);
        }
    };
    
    let d1 = document.createElement("div");
    d1.className = 'rangecontainer';
    let d2 = document.createElement("label");
    d2.innerHTML = 'level';
    d1.appendChild(d2);
    sidebar.appendChild(d1);

    let range = document.createElement("input");
    range.type = 'range';
    range.min = 30;
    range.max = 56;
    range.value = 45;
    range.className = 'slider';
    d1.appendChild(range);

    sidebar.appendChild(
        Button(
            'generate',
            () => {
                console.log(range.value);
                state.generate(range.value);
                updateCells();
            }
        )
    );
    sidebar.appendChild(
        Button(
            'reset',
            () => {
                state.reset();
                Array.from(document.getElementsByClassName('cell')).forEach(cell => cell.cellState = -1);
                updateCells();
            }
        )
    );
    sidebar.appendChild(
        Button(
            'solve',
            () => {
                state.solve();
                updateCells();
            }
        )
    );

    return sidebar;
}

function MainFrame() {
    let mainFrame = document.createElement("div");
    mainFrame.id = 'main-frame';
    mainFrame.appendChild(Sidebar());
    mainFrame.appendChild(Playfield(state, updateCell));
    mainFrame.appendChild(Footer());
    return mainFrame;
}

export default MainFrame;
