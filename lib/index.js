const menu = require("../build/index.node");

class Menu {
    hwnd = 0

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
        if(!this.hwnd) throw new Error("Menu does not exist");
        return await menu.popup(this.hwnd, x, y)
    }

    popupSync(x, y){
        if(!this.hwnd) throw new Error("Menu does not exist");
        return menu.popupSync(this.hwnd, x, y)
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
