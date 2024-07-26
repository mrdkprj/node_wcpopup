declare namespace PopupMenu {
    type SelectedMenuItem = {
        id:string;
        label:string;
        value:string;
        name:string;
        checked:boolean;
    }

    type MenuItem = {
        id?:string;
        type?: "normal" | "separator" | "submenu" | "checkbox" | "radio";
        label?:string;
        accelerator?:string;
        enabled?:boolean;
        visible?:boolean;
        checked?:boolean;
        submenu?:MenuItem[];
        value?:any;
        name?:string;
        onClick?:Function;
    }

    type Theme = "dark" | "light" | "system";
    type MenuSize = {
        borderSize: number,
        verticalMargin: number,
        horizontalMargin: number,
        itemVerticalPadding: number,
        itemHorizontalPadding: number,
        fontSize?: number,
        fontWeight?: number,
    }

    type ColorScheme = {
        color: number,
        border: number,
        disabled: number,
        backgroundColor: number,
        hoverBackgroundColor: number,
    }

    type ThemeColor = {
        dark: ColorScheme,
        light: ColorScheme,
    }

    type Corner = "Round" | "DoNotRound"

    type Config = {
        theme: Theme,
        size: MenuSize,
        color: ThemeColor,
        corner: Corner,
    }

    class Menu {
        constructor();
        getDefaultConfig():Config;
        buildFromTemplate(hwnd:number, template:MenuItem[]): void;
        buildFromTemplateWithTheme(hwnd:number, template:MenuItem[], theme:Theme): void;
        buildFromTemplateWithConfig(hwnd:number, template:MenuItem[], config:Config): void;
        popup(x:number, y:number): Promise<void>;
        popupSync(x:number, y:number): void;
        items():MenuItem[];
        remove(index:number):MenuItem;
        append(item:MenuItem):void;
        insert(index:number, item:MenuItem):void;
        setTheme(theme:Theme):void;
    }
}

export = PopupMenu;