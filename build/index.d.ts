import { Config, Menu, MenuItem, MenuItemConstructorOptions, Theme } from "../lib";

declare namespace PopupMenu {

    type PopupMenuItem = {
        id: string;
        type: "normal" | "separator" | "submenu" | "checkbox" | "radio";
        label: string;
        accelerator: string;
        enabled: boolean;
        checked: boolean;
        submenu: Menu;
        value: any;
        name: string;
        readonly uuid:number;
    };

    function getDefaultConfig(): Config;
    function buildFromTemplate(hwnd:number, template:MenuItemConstructorOptions[]): number;
    function buildFromTemplateWithTheme(hwnd:number, template:MenuItemConstructorOptions[], theme:Theme): number;
    function buildFromTemplateWithConfig(hwnd:number, template:MenuItemConstructorOptions[], config:Config): number;
    function popup(hwnd:number, x:number, y:number): Promise<PopupMenuItem>;
    function popupSync(hwnd:number, x:number, y:number): PopupMenuItem;
    function items(hwnd:number): PopupMenuItem[];
    function remove(hwnd:number, item:MenuItem): PopupMenuItem;
    function removeAt(hwnd:number, index:number): PopupMenuItem;
    function append(hwnd:number, item:MenuItem): void;
    function insert(hwnd:number, index:number, item:MenuItem): void;
    function setTheme(hwnd:number, theme:Theme): void;
    function getMenuItemById(hwnd:number, id:string): PopupMenuItem | void;
}

export default PopupMenu;