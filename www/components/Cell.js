function getValue(state, row, col) {
    return state.get_value(row, col) || "";
}

function Cell(row, col, updateCell, state) {
    let cell = document.createElement("div");
    cell.id = row + ',' + col;
    cell.className = 'cell enabled';
    cell.cellState = -1;
    
    cell.addEventListener("click", event => {
        let value = getValue(state, row, col);
        let newValue = ((value || 0) + 1) % 10;
        state.set_value(newValue, row, col);
        updateCell(row, col);
    });
    
    return cell;
}

export default Cell;