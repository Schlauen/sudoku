function Checkbox(name, state, updateCells) {
    let checkboxContainer = document.createElement("div");
    checkboxContainer.className = 'menu-element checkbox-container';
    let label = document.createElement("label");
    label.innerHTML = name;
    checkboxContainer.appendChild(label);
    let checkbox = document.createElement("div");
    checkbox.className = 'custom-checkbox';
    checkboxContainer.appendChild(checkbox);
    let tick = document.createElement("div");
    tick.className = 'tick';
    checkboxContainer.checked = false;
    checkbox.onclick = () => {
        state.toggle_show_errors();
        updateCells();
        if (checkboxContainer.checked) {
            checkboxContainer.checked = false;
            checkbox.removeChild(tick);
        } else {
            checkboxContainer.checked = true;
            checkbox.appendChild(tick);
        }
    };
    return checkboxContainer;
}

export default Checkbox;