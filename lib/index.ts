import PopupMenu from "../build/index";

export type MenuItemType = "normal" | "separator" | "submenu" | "checkbox" | "radio";

export type MenuItemConstructorOptions = {
    id?: string;
    type?: MenuItemType;
    label?: string;
    accelerator?: string;
    enabled?: boolean;
    checked?: boolean;
    submenu?: MenuItemConstructorOptions[] | Menu;
    value?: any;
    name?: string;
    click?: Function;
};

export type MenuItem = {
    id?: string;
    type?: MenuItemType;
    label?: string;
    accelerator?: string;
    enabled?: boolean;
    checked?: boolean;
    submenu?: Menu;
    value?: any;
    name?: string;
    click?: Function;
};

export type Theme = "dark" | "light" | "system";
export type MenuSize = {
    borderSize: number;
    verticalMargin: number;
    horizontalMargin: number;
    itemVerticalPadding: number;
    itemHorizontalPadding: number;
    fontSize?: number;
    fontWeight?: number;
};

export type ColorScheme = {
    color: number;
    border: number;
    disabled: number;
    backgroundColor: number;
    hoverBackgroundColor: number;
};

export type ThemeColor = {
    dark: ColorScheme;
    light: ColorScheme;
};

export type Corner = "Round" | "DoNotRound";

export type FontWeight = "Thin" | "Light" | "Normal" | "Medium" | "Bold";
export type MenuFont = {
    fontFamily: string;
    darkFontSize: number;
    darkFontWeight: FontWeight;
    lightFontSize: number;
    lightFontWeight: FontWeight;
};

export type Config = {
    theme: Theme;
    size: MenuSize;
    color: ThemeColor;
    corner: Corner;
    font: MenuFont;
};

export type MenuType = "main" | "submenu";

const UUID = "MenuItem";

export const getDefaultConfig = () => {
    return PopupMenu.getDefaultConfig();
};

export class Menu {
    hwnd = 0;
    type = "";
    private callbacks: { [key: string]: Function } = {};
    private uuid = 0;

    private ready() {
        if (!this.hwnd) throw new Error("Menu does not exist");
    }

    buildFromTemplate(hwnd: number, template: MenuItemConstructorOptions[]) {
        const effectiveTemplate = this.toEffectiveTemplates(template);
        this.hwnd = PopupMenu.buildFromTemplate(hwnd, effectiveTemplate);
    }

    buildFromTemplateWithTheme(hwnd: number, template: MenuItemConstructorOptions[], theme: Theme) {
        const effectiveTemplate = this.toEffectiveTemplates(template);
        this.hwnd = PopupMenu.buildFromTemplateWithTheme(hwnd, effectiveTemplate, theme);
    }

    buildFromTemplateWithConfig(hwnd: number, template: MenuItemConstructorOptions[], config: Config) {
        const effectiveTemplate = this.toEffectiveTemplates(template);
        this.hwnd = PopupMenu.buildFromTemplateWithConfig(hwnd, effectiveTemplate, config);
    }

    private toEffectiveTemplates(items: MenuItemConstructorOptions[]): MenuItemConstructorOptions[] {
        return items.map((item) => {
            const newItem = this.toEffectiveTemplate(item);
            if (newItem.type == "submenu" && newItem.submenu) {
                this.toEffectiveTemplates(newItem.submenu as MenuItemConstructorOptions[]);
            }
            return newItem;
        });
    }

    private toEffectiveTemplate(item: MenuItemConstructorOptions | MenuItem): MenuItemConstructorOptions | MenuItem {
        item.id = this.getId(item.id);

        item.value = this.getValue(item.value);

        item.type = this.getType(item.type, item.submenu);

        if (item.click) {
            this.callbacks[item.id] = item.click;
        } else {
            this.callbacks[item.id] = () => {};
        }

        return item;
    }

    private getId(id: string | undefined): string {
        if (!id) {
            this.uuid++;
            return UUID + this.uuid;
        }

        return id;
    }

    private getType(type: MenuItemType | undefined, submenu: MenuItemConstructorOptions[] | Menu | undefined): MenuItemType {
        if (!type) {
            return submenu ? "submenu" : "normal";
        }

        return type;
    }

    private getValue(value: any): string {
        if (typeof value != "string") {
            return value ? String(value) : "";
        }

        return value;
    }

    private toMenuItem(item: PopupMenu.PopupMenuItem): MenuItem {
        const submenu = new Menu();
        if (item.submenu && Object.keys(item.submenu).length) {
            submenu.hwnd = item.submenu.hwnd;
            submenu.type = item.submenu.type;
        }
        return {
            ...item,
            click: this.callbacks[item.id],
            submenu,
        };
    }

    async popup(x: number, y: number) {
        this.ready();
        const result = await PopupMenu.popup(this.hwnd, x, y);
        if (Object.keys(result).length) {
            this.callbacks[result.id](result);
        }
    }

    popupSync(x: number, y: number) {
        this.ready();
        const result = PopupMenu.popupSync(this.hwnd, x, y);
        if (Object.keys(result).length) {
            this.callbacks[result.id](result);
        }
    }

    items(): MenuItem[] {
        this.ready();
        return PopupMenu.items(this.hwnd).map((item) => this.toMenuItem(item));
    }

    remove(item: MenuItem) {
        this.ready();
        this.toMenuItem(PopupMenu.remove(this.hwnd, item));
    }

    removeFrom(hwnd: number, item: MenuItem) {
        this.ready();
        this.toMenuItem(PopupMenu.remove(hwnd, item));
    }

    removeAt(index: number) {
        this.ready();
        this.toMenuItem(PopupMenu.removeAt(this.hwnd, index));
    }

    removeAtFrom(hwnd: number, index: number) {
        this.ready();
        this.toMenuItem(PopupMenu.removeAt(hwnd, index));
    }

    append(item: MenuItem) {
        this.ready();
        PopupMenu.append(this.hwnd, this.toEffectiveTemplate(item) as MenuItem);
    }

    appendTo(hwnd: number, item: MenuItem) {
        this.ready();
        PopupMenu.append(hwnd, this.toEffectiveTemplate(item) as MenuItem);
    }

    insert(index: number, item: MenuItem) {
        this.ready();
        PopupMenu.insert(this.hwnd, index, this.toEffectiveTemplate(item) as MenuItem);
    }

    insertTo(hwnd: number, index: number, item: MenuItem) {
        this.ready();
        PopupMenu.insert(hwnd, index, this.toEffectiveTemplate(item) as MenuItem);
    }

    setTheme(theme: Theme) {
        this.ready();
        PopupMenu.setTheme(this.hwnd, theme);
    }

    getMenuItemById(id: string): MenuItem | void {
        this.ready();
        const item = PopupMenu.getMenuItemById(this.hwnd, id);
        if (item) {
            return this.toMenuItem(item);
        }
        return item;
    }
}
