const { contextBridge, ipcRenderer } = require('electron')

contextBridge.exposeInMainWorld('electronAPI', {
  setTitle: (title) => ipcRenderer.send('set-title', title),
  toggle: () => ipcRenderer.send('toggle'),
  append: () => ipcRenderer.send('append'),
  reload: () => ipcRenderer.send('reload'),
})