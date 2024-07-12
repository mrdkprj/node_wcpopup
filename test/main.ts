import { app, BrowserWindow,ipcMain } from "electron";
import os from "os"
import path from "path";
import { Menu, MenuItem } from "../lib/index"

let menu:Menu;

const createWindow = () => {
  const win = new BrowserWindow({
    width: 800,
    height: 600,
    webPreferences: {
        preload: path.join(__dirname, 'preload.js')
    }
  })

  win.loadFile('index.html')
  menu = new Menu();
  const hbuf = win.getNativeWindowHandle();
  let hwnd;
    if (os.endianness() == "LE") {
        hwnd = hbuf.readInt32LE()
    }
    else {
        hwnd = hbuf.readInt32BE()
    }
    let config = menu.getDefaultConfig();
    config.theme = "Dark"
    config.size.itemVerticalPadding = 15;
    menu.buildFromTemplateWithConfig(hwnd, getTemp(), config)

    menu.append({accelerator:"F1", label:"Test"});

}

const handleSetTitle = async (_event:any, pos:any) => {
    const x = await menu.popup(pos.x, pos.y);
    console.log(x)
}

app.whenReady().then(async () => {
  createWindow()
  ipcMain.on('set-title', handleSetTitle)
})

app.on('window-all-closed', () => {
    if (process.platform !== 'darwin') app.quit()
})

const getTemp = () => {
  const template:MenuItem[] = [
    {
      id:"",
        label: t("playbackSpeed"),
        submenu: playbackSpeedMenu()
    },
    {
      id:"",
        label: t("seekSpeed"),
        submenu: seekSpeedMenu()
    },
    {
      id:"",
        label: t("fitToWindow"),
        type: "checkbox",
        checked: false,
    },
    { type: "separator" },
    {
      id:"",
        //label: t("playlist"),
        label: t("playbackSpeed"),
        //accelerator: "CmdOrCtrl+P",
    },
    {
      id:"",
        label: t("fullscreen"),
        accelerator:"F11",
    },
    {
      id:"",
        label: t("pip"),
    },
    {      id:"", type: "separator" },
    {
      id:"",
        label: t("capture"),
        //accelerator: "CmdOrCtrl+S",
    },
    {      id:"", type: "separator" },
    {
      id:"",
        label: t("theme"),
        submenu: themeMenu()
    },
  ]

  return template;
}

const themeMenu = () => {

  const template:MenuItem[] = [
    {
        id: "themeLight",
        label: t("light"),
        type:"checkbox",
        checked: false,
        value:"light"
    },
    {
        id: "themeDark",
        label: t("dark"),
        type:"checkbox",
        checked: true,
        value:"dark"
    },
  ]

  return template;
}

const playbackSpeedMenu = () => {

  const type = "PlaybackSpeed"
  const template:MenuItem[] = [
    {
        id: "playbackrate0",
        label:"0.25",
        type:"checkbox",
        value:0.25
    },
    {
        id: "playbackrate1",
        label:"0.5",
        type:"checkbox",
        value:0.5
    },
    {
        id: "playbackrate2",
        label:"0.75",
        type:"checkbox",
        value:0.75
    },
    {
        id: "playbackrate3",
        label:`1 - ${t("default")}`,
        type:"checkbox",
        checked:true,
        value:1
    },
    {
        id: "playbackrate4",
        label:"1.25",
        type:"checkbox",
        value:1.25
    },
    {
        id: "playbackrate5",
        label:"1.5",
        type:"checkbox",
        value:1.5
    },
    {
        id: "playbackrate6",
        label:"1.75",
        type:"checkbox",
        value:1.75
    },
    {
        id: "playbackrate7",
        label:"2",
        type:"checkbox",
        value:2
    },
  ]

  return template;
}

const seekSpeedMenu = () => {


  const template:MenuItem[] = [
    {
        id: "seekspeed0",
        label:`0.03${t("second")}`,
        type:"checkbox",
        value:0.03
    },
    {
        id: "seekspeed1",
        label:`0.05${t("second")}`,
        type:"checkbox",
        value:0.05
    },
    {
        id: "seekspeed2",
        label:`0.1${t("second")}`,
        type:"checkbox",
        value:0.1
    },
    {
        id: "seekspeed3",
        label:`0.5${t("second")}`,
        type:"checkbox",
        value:0.5
    },
    {
        id: "seekspeed4",
        label:`1${t("second")}`,
        type:"checkbox",
        value:1
    },
    {
        id: "seekspeed5",
        label:`3${t("second")}`,
        type:"checkbox",
        value:3
    },
    {
        id: "seekspeed6",
        label:`5${t("second")} - ${t("default")}`,
        type:`checkbox`,
        checked:true,
        value:5
    },
    {
        id: "seekspeed7",
        label:`10${t("second")}`,
        type:"checkbox",
        value:10
    },
    {
        id: "seekspeed8",
        label:`20${t("second")}`,
        type:"checkbox",
        value:20
    },
  ]

  return template;
}

const t = (a:string) => a;