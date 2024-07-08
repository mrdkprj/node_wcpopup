window.addEventListener("contextmenu", e => {
    window.electronAPI.setTitle({x:e.screenX, y:e.screenY});
})