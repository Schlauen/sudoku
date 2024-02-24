import { PlayfieldState } from "wasm-sudoku";
import Button from "./Button";
import Playfield from "./Playfield";
import Footer from "./Footer";

const state = PlayfieldState.new();
var selectedCell = {
    row: 0,
    col: 0
}
var spacePressed = false;
console.log('new state');

document.addEventListener('keydown', (e) => {
    if (e.code === "Space") spacePressed = true;
});

document.addEventListener('keyup', (e) => { 
    if (e.code === "Space") spacePressed = false;

    if (e.code === "ArrowUp" || e.code === "KeyW") selectedCell.row = Math.max(0, selectedCell.row - 1);
    else if (e.code === "ArrowDown" || e.code === "KeyS") selectedCell.row = Math.min(8, selectedCell.row + 1);
    else if (e.code === "ArrowLeft" || e.code === "KeyA") selectedCell.col = Math.max(0, selectedCell.col - 1);
    else if (e.code === "ArrowRight" || e.code === "KeyD") selectedCell.col = Math.min(8, selectedCell.col + 1);
    else if (spacePressed && digitPessed(e, 1)) console.log("Hallo Ursin.");
    else if (digitPessed(e, 1)) state.set_value(1, selectedCell.row, selectedCell.col);
    else if (digitPessed(e, 2)) state.set_value(2, selectedCell.row, selectedCell.col);
    else if (digitPessed(e, 3)) state.set_value(3, selectedCell.row, selectedCell.col);
    else if (digitPessed(e, 4)) state.set_value(4, selectedCell.row, selectedCell.col);
    else if (digitPessed(e, 5)) state.set_value(5, selectedCell.row, selectedCell.col);
    else if (digitPessed(e, 6)) state.set_value(6, selectedCell.row, selectedCell.col);
    else if (digitPessed(e, 7)) state.set_value(7, selectedCell.row, selectedCell.col);
    else if (digitPessed(e, 8)) state.set_value(8, selectedCell.row, selectedCell.col);
    else if (digitPessed(e, 9)) state.set_value(9, selectedCell.row, selectedCell.col);
    else if (e.code === "Delete" || digitPessed(e, 0)) state.reset_value(selectedCell.row, selectedCell.col);
  
    updateCells();
});

function digitPessed(event, digit) {
    return event.code === "Digit" + digit || event.code === "Numpad" + digit;
}

function updateCells() {
    let cells = document.getElementsByClassName("cell");
    Array.from(cells).forEach((cell) => updateCell(cell));
}

function updateCell(cell) {
    let row = cell.getAttribute("row");
    let col = cell.getAttribute("col");

    cell.innerHTML = state.get_value(row, col) || "";

    let className = 'cell';
    if (selectedCell.row == row && selectedCell.col == col) {
        className += ' selected';
    }

    if (state.is_fix(row, col)) {
        className += ' disabled';
    } else {
        if (state.get_show_errors() && state.is_error(row, col)) {
            className += ' error';
        } else {
            className += ' enabled';
        }
    }
    cell.className = className;
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
