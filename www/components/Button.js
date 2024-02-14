function Button(name, onClick) {
    let button = document.createElement("div");
    button.className = 'btn';
    button.innerHTML = name;
    button.onclick = onClick;
    return button;
}

export default Button;