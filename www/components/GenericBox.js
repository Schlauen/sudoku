function GenericBox(parentRow, parentCol, childFactory) {
    let box = document.createElement("div");
    box.className = 'box';
    box.setAttribute('row', parentRow);
    box.setAttribute('col', parentCol);
    for (let i = 0; i < 9; i += 1) {
        let row = 3 * parentRow + Math.floor(i / 3);
        let col = 3 * parentCol + (i % 3);
        box.appendChild(childFactory(row, col));
    }
    return box;
}

export default GenericBox;