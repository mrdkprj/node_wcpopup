window.addEventListener("contextmenu", e => {
    window.electronAPI.setTitle({x:e.screenX, y:e.screenY});
})

window.addEventListener("keydown", _ => {
    console.log("down")
})

window.addEventListener("click", e => {
    if (e.target.id == "btn"){
        window.electronAPI.toggle();
    }

    if (e.target.id == "append"){
        window.electronAPI.append();
    }
})
