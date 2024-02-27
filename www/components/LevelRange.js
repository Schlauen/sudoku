function LevelRange() {
    let levelRange = document.createElement("div");
    levelRange.className = 'menu-element range-container';
    let label = document.createElement("label");
    label.innerHTML = 'level:';
    levelRange.appendChild(label);

    let range = document.createElement("input");
    range.type = 'range';
    range.min = 30;
    range.max = 58;
    range.value = 45;
    range.className = 'slider';
    range.id = 'level-value';
    levelRange.appendChild(range);
    return levelRange;
}

export default LevelRange;