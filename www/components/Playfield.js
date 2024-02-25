import GenericBox from "./GenericBox";
import Cell from "./Cell";

function Playfield(state, updateCell) {
    let playfield = document.createElement("div");
    playfield.id = 'playfield';
    playfield.appendChild(GenericBox(
        0, 0,
        (boxRow, boxCol) => GenericBox(
            boxRow, boxCol,
            (row, col) => Cell(
                row, col,
                updateCell,
                state,
            )
        )
    ));
    return playfield;
}

export default Playfield;