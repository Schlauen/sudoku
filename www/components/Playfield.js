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
                () => state.get_value(row, col) || "",
                newValue => state.set_value(newValue, row, col),
                updateCell
            )
        )
    ));
    return playfield;
}

export default Playfield;