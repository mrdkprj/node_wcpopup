let openContext = false;
window.addEventListener("contextmenu", e => {
    if(navigator.userAgent.includes("Linux")){
        e.preventDefault();
        openContext = true;
    }else{
        window.electronAPI.setTitle({x:e.screenX, y:e.screenY});
    }

})

window.addEventListener("mouseup", (e) => {{

    if(navigator.userAgent.includes("Linux")){
        if (e.button === 2 && openContext) {{
            window.electronAPI.setTitle({x:e.clientX, y:e.clientY});
            openContext = false;
        }}
    }
}});

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

    if (e.target.id == "reload"){
        window.electronAPI.reload();
    }

})
