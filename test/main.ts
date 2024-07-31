import { app, BrowserWindow, ipcMain, nativeTheme } from "electron";
import os from "os";
import path from "path";
import { getDefaultConfig, Menu, MenuItem, MenuItemConstructorOptions } from "../lib/index";

let menu: Menu;
let dark = true;
const createWindow = () => {
    nativeTheme.themeSource = "dark";
    const win = new BrowserWindow({
        title: "main",
        width: 800,
        height: 601,
        webPreferences: {
            preload: path.join(__dirname, "preload.js"),
        },
    });

    win.loadFile("index.html");

    const win2 = new BrowserWindow({
        title: "sub",
        parent: win,
        width: 800,
        height: 601,
        webPreferences: {
            preload: path.join(__dirname, "preload.js"),
        },
    });

    win2.loadFile("index2.html");

    menu = new Menu();
    const hbuf = win2.getNativeWindowHandle();
    let hwnd = 0;
    if (os.endianness() == "LE") {
        hwnd = hbuf.readInt32LE();
    } else {
        hwnd = hbuf.readInt32BE();
    }
    let config = getDefaultConfig();
    config.theme = "dark";
    config.size.itemVerticalPadding = 15;
    menu.buildFromTemplateWithConfig(hwnd, getTemp(), config);
};

const handleSetTitle = async (_event: any, pos: any) => {
    await menu.popup(pos.x, pos.y);
};

const toggle = () => {
    dark = !dark;
    nativeTheme.themeSource = dark ? "dark" : "light";
    menu.setTheme(nativeTheme.themeSource);
};

let apflg = true;
const append = () => {
    apflg = !apflg;
    if (apflg) {
        menu.append({
            accelerator: "F1",
            label: "Test fro main",
            click: callback,
        });
    } else {
        const submenu = menu.getMenuItemById("theme");
        if (submenu && submenu.submenu) {
            menu.appendTo(submenu.submenu.hwnd, {
                accelerator: "F2",
                label: "Test for sub",
                click: callback,
            });
        }
    }
};

const reload = () => {
    const submenu = menu.getMenuItemById("theme");
    if (submenu) {
        let items = submenu.submenu?.items();
        console.log(items);
    }
};

app.whenReady().then(async () => {
    createWindow();
    ipcMain.on("set-title", handleSetTitle);
    ipcMain.on("toggle", toggle);
    ipcMain.on("append", append);
    ipcMain.on("reload", reload);
});

app.on("window-all-closed", () => {
    if (process.platform !== "darwin") app.quit();
});

const callback = (a: MenuItem) => {
    console.log(a);
};

const getTemp = () => {
    const template: MenuItemConstructorOptions[] = [
        {
            id: "",
            label: t("playbackSpeed"),
            submenu: playbackSpeedMenu(),
        },
        {
            id: "",
            label: t("seekSpeed"),
            submenu: seekSpeedMenu(),
        },
        {
            id: "",
            label: t("fitToWindow"),
            type: "checkbox",
            checked: false,
            click: callback,
        },
        { type: "separator" },
        {
            id: "",
            //label: t("playlist"),
            label: t("playbackSpeed"),
            //accelerator: "CmdOrCtrl+P",

            click: callback,
        },
        {
            id: "",
            label: t("fullscreen"),
            accelerator: "F11",
            click: callback,
        },
        {
            id: "",
            label: t("pip"),
            click: callback,
        },
        { id: "", type: "separator" },
        {
            id: "",
            label: t("capture"),
            //accelerator: "CmdOrCtrl+S",
            click: callback,
        },
        { id: "", type: "separator" },
        {
            id: "theme",
            label: t("theme"),
            submenu: themeMenu(),
        },
    ];

    return template;
};

const themeMenu = () => {
    const template: MenuItemConstructorOptions[] = [
        {
            id: "themelight",
            label: t("light"),
            type: "checkbox",
            checked: false,
            click: callback,
            value: "light",
        },
        {
            id: "themedark",
            label: t("dark"),
            type: "checkbox",
            checked: true,
            click: callback,
            value: "dark",
        },
    ];

    return template;
};

const playbackSpeedMenu = () => {
    const template: MenuItemConstructorOptions[] = [
        {
            id: "playbackrate0",
            label: "0.25",
            type: "checkbox",
            click: callback,
            value: 0.25,
        },
        {
            id: "playbackrate1",
            label: "0.5",
            type: "checkbox",
            click: callback,
            value: 0.5,
        },
        {
            id: "playbackrate2",
            label: "0.75",
            type: "checkbox",
            click: callback,
            value: 0.75,
        },
        {
            id: "playbackrate3",
            label: `1 - ${t("default")}`,
            type: "checkbox",
            click: callback,
            checked: true,
            value: 1,
        },
        {
            id: "playbackrate4",
            label: "1.25",
            type: "checkbox",
            click: callback,
            value: 1.25,
        },
        {
            id: "playbackrate5",
            label: "1.5",
            type: "checkbox",
            click: callback,
            value: 1.5,
        },
        {
            id: "playbackrate6",
            label: "1.75",
            type: "checkbox",
            click: callback,
            value: 1.75,
        },
        {
            id: "playbackrate7",
            label: "2",
            type: "checkbox",
            click: callback,
            value: 2,
        },
    ];

    return template;
};

const seekSpeedMenu = () => {
    const template: MenuItemConstructorOptions[] = [
        {
            id: "seekspeed0",
            label: `0.03${t("second")}`,
            type: "checkbox",
            value: 0.03,
        },
        {
            id: "seekspeed1",
            label: `0.05${t("second")}`,
            type: "checkbox",
            value: 0.05,
        },
        {
            id: "seekspeed2",
            label: `0.1${t("second")}`,
            type: "checkbox",
            value: 0.1,
        },
        {
            id: "seekspeed3",
            label: `0.5${t("second")}`,
            type: "checkbox",
            value: 0.5,
        },
        {
            id: "seekspeed4",
            label: `1${t("second")}`,
            type: "checkbox",
            value: 1,
        },
        {
            id: "seekspeed5",
            label: `3${t("second")}`,
            type: "checkbox",
            value: 3,
        },
        {
            id: "seekspeed6",
            label: `5${t("second")} - ${t("default")}`,
            type: `checkbox`,
            checked: true,
            value: 5,
        },
        {
            id: "seekspeed7",
            label: `10${t("second")}`,
            type: "checkbox",
            value: 10,
        },
        {
            id: "seekspeed8",
            label: `20${t("second")}`,
            type: "checkbox",
            value: 20,
        },
    ];

    return template;
};

const t = (a: string) => a;
