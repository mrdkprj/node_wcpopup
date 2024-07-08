const menu = require("../build/index.node");

class Menu {
    hwnd = 0

    getDefaultConfig(){
        return menu.getDefaultConfig();
    }

    buildFromTemplate(hwnd, rawTemplatem){
        const template = this.extract(rawTemplatem);
        this.hwnd = menu.buildFromTemplate(hwnd, template)
    }

    buildFromTemplateWithTheme(hwnd, rawTemplatem, theme){
        const template = this.extract(rawTemplatem);
        this.hwnd = menu.buildFromTemplateWithTheme(hwnd, template, theme == "Dark")
    }

    buildFromTemplateWithConfig(hwnd, rawTemplatem, config){
        const template = this.extract(rawTemplatem);
        this.hwnd = menu.buildFromTemplateWithConfig(hwnd, template, config)
    }

    extract(rawTemplatem){
        return rawTemplatem.map(item => {
            if(typeof item.value != "string"){
                item.value = item.value ? String(item.value) : ""
            }
            if(!item.type){
                item.type = item.submenu ? "submenu" : "normal"
            }

            if(item.type == "submenu"){
                this.extract(item.submenu)
            }
            return item;
        })
    }

    async popup(x, y){
        if(!this.hwnd) throw new Error("Menu does not exist");
        return await menu.popup(this.hwnd, x, y)
    }

    items(){
        if(!this.hwnd) throw new Error("Menu does not exist");
        return menu.items(this.hwnd)
    }

    remove(index){
        if(!this.hwnd) throw new Error("Menu does not exist");
        menu.remove(this.hwnd, index)
    }

    append(item){
        if(!this.hwnd) throw new Error("Menu does not exist");
        menu.append(this.hwnd, item)
    }

    insert(index, item){
        if(!this.hwnd) throw new Error("Menu does not exist");
        menu.insert(this.hwnd, index, item);
    }

}

module.exports = { Menu };
