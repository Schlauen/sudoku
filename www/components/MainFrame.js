import { PlayfieldState } from "wasm-sudoku";
import Button from "./Button";
import Playfield from "./Playfield";
import Footer from "./Footer";

const state = PlayfieldState.new();
console.log('new state');

function updateCells() {
    let cells = document.getElementsByClassName("cell");
    Array.from(cells).forEach((cell) => updateCell(cell));
}

function updateCell(cell) {
    let row = cell.getAttribute("row");
    let col = cell.getAttribute("col");

    cell.innerHTML = state.get_value(row, col) || "";

    if (state.is_fix(row, col)) {
        cell.className = 'cell disabled';
    } else {
        if (state.get_show_errors() && state.is_error(row, col)) {
            cell.className = 'cell error';
        } else {
            cell.className = 'cell enabled';
        }
    }
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
    range.min = 20;
    range.max = 55;
    range.value = 40;
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
