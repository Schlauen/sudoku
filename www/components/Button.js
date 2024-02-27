function Button(name, onClick) {
    let button = document.createElement("div");
    button.className = 'menu-element btn';
    button.innerHTML = name;
    button.onclick = onClick;
    return button;
}

export default Button;