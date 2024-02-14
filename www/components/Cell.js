function Cell(row, col, getValue, setValue, updateCell) {
    let cell = document.createElement("div");
    cell.className = "cell enabled";
    cell.setAttribute('row', row);
    cell.setAttribute('col', col);
    cell.innerHTML = getValue();
    cell.addEventListener("click", event => {
        let value = getValue();
        let newValue = ((value || 0) + 1) % 10;
        setValue(newValue);
        updateCell(cell);
    });
    
    return cell;
}

export default Cell;