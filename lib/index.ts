import PopupMenu from "../build/index"

export type MenuItemConstructorOptions = {
    id?:string;
    type?: "normal" | "separator" | "submenu" | "checkbox" | "radio";
    label?:string;
    accelerator?:string;
    enabled?:boolean;
    visible?:boolean;
    checked?:boolean;
    submenu?:(MenuItemConstructorOptions[]) | (Menu);
    value?:any;
    name?:string;
    click?:Function;
}

export type MenuItem = {
    id?:string;
    type?: "normal" | "separator" | "submenu" | "checkbox" | "radio";
    label?:string;
    accelerator?:string;
    enabled?:boolean;
    visible?:boolean;
    checked?:boolean;
    submenu?:Menu;
    value?:any;
    name?:string;
    click?:Function;
}

export type Theme = "dark" | "light" | "system";
export type MenuSize = {
    borderSize: number,
    verticalMargin: number,
    horizontalMargin: number,
    itemVerticalPadding: number,
    itemHorizontalPadding: number,
    fontSize?: number,
    fontWeight?: number,
}

export type ColorScheme = {
    color: number,
    border: number,
    disabled: number,
    backgroundColor: number,
    hoverBackgroundColor: number,
}

export type ThemeColor = {
    dark: ColorScheme,
    light: ColorScheme,
}

export type Corner = "Round" | "DoNotRound"

export type Config = {
    theme: Theme,
    size: MenuSize,
    color: ThemeColor,
    corner: Corner,
}

export type MenuType = "main" | "submenu";

const UUID = "MenuItem";

export const getDefaultConfig = () => {
    return PopupMenu.getDefaultConfig();
}

export class Menu {
    hwnd = 0;
    type = "";
    private callbacks:{[key:string]:Function} = {};
    private uuid = 0;

    private ready(){
        if(!this.hwnd) throw new Error("Menu does not exist");
    }

    buildFromTemplate(hwnd:number, template:Array<(MenuItemConstructorOptions) | (MenuItem)>){
        const effectiveTemplate = this.toEffectiveTemplates(template);
        this.hwnd = PopupMenu.buildFromTemplate(hwnd, effectiveTemplate)
    }

    buildFromTemplateWithTheme(hwnd:number, template:Array<(MenuItemConstructorOptions) | (MenuItem)>, theme:Theme){
        const effectiveTemplate = this.toEffectiveTemplates(template);
        this.hwnd = PopupMenu.buildFromTemplateWithTheme(hwnd, effectiveTemplate, theme)
    }

    buildFromTemplateWithConfig(hwnd:number, template:Array<(MenuItemConstructorOptions) | (MenuItem)>, config:Config){
        const effectiveTemplate = this.toEffectiveTemplates(template);
        this.hwnd = PopupMenu.buildFromTemplateWithConfig(hwnd, effectiveTemplate, config);
    }

    private toEffectiveTemplates(items:Array<(MenuItemConstructorOptions) | (MenuItem)>){
        return items.map(item => {
            const newItem = this.toEffectiveTemplate(item);
            if(newItem.type == "submenu" && newItem.submenu){
                this.toEffectiveTemplates(newItem.submenu as MenuItemConstructorOptions[])
            }
            return newItem;
        })
    }

    private toEffectiveTemplate(item:MenuItemConstructorOptions | MenuItem):MenuItemConstructorOptions | MenuItem{

        if(!item.id){
            this.uuid++;
            item.id = UUID + this.uuid;
        }

        if(typeof item.value != "string"){
            item.value = item.value ? String(item.value) : ""
        }

        if(!item.type){
            item.type = item.submenu ? "submenu" : "normal"
        }

        if(item.click){
            this.callbacks[item.id] = item.click
            item.click = undefined;
        }else{
            this.callbacks[item.id] = () => {};
        }

        return item;

    }

    async popup(x:number, y:number){
        this.ready();
        const result = await PopupMenu.popup(this.hwnd, x, y);
        if(Object.keys(result).length && result.id){
            this.callbacks[result.id](result);
        }
    }

    popupSync(x:number, y:number){
        this.ready();
        const result = PopupMenu.popupSync(this.hwnd, x, y);
        if(Object.keys(result).length && result.id){
            this.callbacks[result.id](result);
        }
    }

    items(){
        this.ready();
        return PopupMenu.items(this.hwnd)
    }

    remove(item:MenuItem){
        this.ready();
        return PopupMenu.remove(this.hwnd, item)
    }

    removeFrom(hwnd:number, item:MenuItem){
        this.ready();
        return PopupMenu.remove(hwnd, item)
    }

    removeAt(index:number){
        this.ready();
        return PopupMenu.removeAt(this.hwnd, index)
    }

    removeAtFrom(hwnd:number, index:number){
        this.ready();
        return PopupMenu.removeAt(hwnd, index)
    }

    append(item:MenuItem){
        this.ready();
        PopupMenu.append(this.hwnd, this.toEffectiveTemplate(item) as MenuItem)
    }

    appendTo(hwnd:number, item:MenuItem){
        this.ready();
        PopupMenu.append(hwnd, this.toEffectiveTemplate(item) as MenuItem);
    }

    insert(index:number, item:MenuItem){
        this.ready();
        PopupMenu.insert(this.hwnd, index, this.toEffectiveTemplate(item) as MenuItem);
    }

    insertTo(hwnd:number, index:number, item:MenuItem){
        this.ready();
        PopupMenu.insert(hwnd, index, this.toEffectiveTemplate(item) as MenuItem);
    }

    setTheme(theme:Theme){
        this.ready();
        PopupMenu.setTheme(this.hwnd, theme);
    }

    getMenuItemById(id:string){
        this.ready();
        return PopupMenu.getMenuItemById(this.hwnd, id);
    }
}
