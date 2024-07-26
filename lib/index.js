const menu = require("../build/index.node");

const UUID = "MenuItem";

class Menu {
    _hwnd = 0;
    _callbacks = {};
    _uuid = 0;

    _ready(){
        if(!this._hwnd) throw new Error("Menu does not exist");
    }

    getDefaultConfig(){
        return menu.getDefaultConfig();
    }

    buildFromTemplate(hwnd, rawTemplate){
        const template = this._extract(rawTemplate);
        this._hwnd = menu.buildFromTemplate(hwnd, template)
    }

    buildFromTemplateWithTheme(hwnd, rawTemplate, theme){
        const template = this._extract(rawTemplate);
        this._hwnd = menu.buildFromTemplateWithTheme(hwnd, template, theme == "Dark")
    }

    buildFromTemplateWithConfig(hwnd, rawTemplate, config){
        const template = this._extract(rawTemplate);
        this._hwnd = menu.buildFromTemplateWithConfig(hwnd, template, config)
    }

    _extract(rawTemplate){
        return rawTemplate.map(item => {

            if(!item.id){
                this._uuid++;
                item.id = UUID + this._uuid;
            }

            if(typeof item.value != "string"){
                item.value = item.value ? String(item.value) : ""
            }

            if(!item.type){
                item.type = item.submenu ? "submenu" : "normal"
            }

            if(item.type == "submenu"){
                this._extract(item.submenu)
            }

            if(item.click){
                callbacks[item.id] = item.click
                item.click = null;
            }

            return item;
        })
    }

    async popup(x, y){
        this._ready();
        const result = await menu.popup(this._hwnd, x, y);
        if(Object.keys(result).length){
            this._callbacks[result.id](result);
        }
    }

    popupSync(x, y){
        this._ready();
        const result = menu.popupSync(this._hwnd, x, y);
        if(Object.keys(result).length){
            this._callbacks[result.id](result);
        }
    }

    items(){
        this._ready();
        return menu.items(this._hwnd)
    }

    remove(index){
        this._ready();
        menu.remove(this._hwnd, index)
    }

    append(item){
        this._ready();
        menu.append(this._hwnd, item)
    }

    insert(index, item){
        this._ready();
        menu.insert(this._hwnd, index, item);
    }

    setTheme(theme){
        this._ready();
        menu.setTheme(this._hwnd, theme);
    }
}

module.exports = { Menu };
