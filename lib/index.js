const menu = require("../build/index.node");

class Menu {
    hwnd = 0

    _ready(){
        if(!this.hwnd) throw new Error("Menu does not exist");
    }

    getDefaultConfig(){
        return menu.getDefaultConfig();
    }

    buildFromTemplate(hwnd, rawTemplate){
        const template = this.extract(rawTemplate);
        this.hwnd = menu.buildFromTemplate(hwnd, template)
    }

    buildFromTemplateWithTheme(hwnd, rawTemplate, theme){
        const template = this.extract(rawTemplate);
        this.hwnd = menu.buildFromTemplateWithTheme(hwnd, template, theme == "Dark")
    }

    buildFromTemplateWithConfig(hwnd, rawTemplate, config){
        const template = this.extract(rawTemplate);
        this.hwnd = menu.buildFromTemplateWithConfig(hwnd, template, config)
    }

    extract(rawTemplate){
        return rawTemplate.map(item => {

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
        this._ready();
        return await menu.popup(this.hwnd, x, y)
    }

    popupSync(x, y){
        this._ready();
        return menu.popupSync(this.hwnd, x, y)
    }

    items(){
        this._ready();
        return menu.items(this.hwnd)
    }

    remove(index){
        this._ready();
        menu.remove(this.hwnd, index)
    }

    append(item){
        this._ready();
        menu.append(this.hwnd, item)
    }

    insert(index, item){
        this._ready();
        menu.insert(this.hwnd, index, item);
    }

    setTheme(theme){
        this._ready();
        menu.setTheme(this.hwnd, theme);
    }
}

module.exports = { Menu };
