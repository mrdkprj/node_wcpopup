#![allow(unused_imports)]
use async_std::sync::Mutex;
use gdkx11::{
    ffi::{gdk_x11_get_default_xdisplay, gdk_x11_lookup_xdisplay, gdk_x11_window_foreign_new_for_display, GdkX11Display, GdkX11Window},
    X11Display, X11DisplayManager, X11Window,
};
use gtk::{
    ffi::{gtk_widget_new, GtkWindow},
    gdk::{ffi::gdk_init, init},
    glib::{self, ffi::g_idle_add, subclass::types::FromObject, translate::FromGlibPtrNone, ObjectExt},
    prelude::{ContainerExt, WidgetExt, WindowExtManual},
    ApplicationWindow, Widget,
};
use gtk::{
    gdk::{ffi::GdkWindow, Window},
    glib::{
        idle_add,
        translate::{FromGlibPtrFull, ToGlibPtr},
        Cast,
    },
};
use neon::{
    handle::Handle,
    prelude::{Context, FunctionContext, ModuleContext},
    result::{JsResult, NeonResult},
    types::{JsArray, JsNumber, JsObject, JsPromise, JsString, JsUndefined, JsValue},
};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use wcpopup::{
    config::{Config, Theme},
    Menu, MenuBuilder,
};
mod types;
use types::*;

static MENU_MAP: Lazy<Mutex<HashMap<i32, Menu>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn build_from_template(mut cx: FunctionContext) -> JsResult<JsNumber> {
    if cx.len() != 2 {
        return cx.throw_error("Invalid number of arguments");
    }

    let parent = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let templates = cx.argument::<JsArray>(1)?.to_vec(&mut cx).unwrap();

    let hwnd = build(&mut cx, parent, templates, Config::default());
    let id = cx.number(hwnd as i32);
    Ok(id)
}

pub fn build_from_template_with_theme(mut cx: FunctionContext) -> JsResult<JsNumber> {
    if cx.len() != 3 {
        return cx.throw_error("Invalid number of arguments");
    }

    let parent = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let templates = cx.argument::<JsArray>(1)?.to_vec(&mut cx).unwrap();
    let theme_str = cx.argument::<JsString>(2)?.value(&mut cx);

    let theme = match theme_str.as_str() {
        "dark" => Theme::Dark,
        "light" => Theme::Light,
        "system" => Theme::System,
        _ => Theme::System,
    };

    let config = Config {
        theme,
        ..Default::default()
    };

    let hwnd = build(&mut cx, parent, templates, config);
    let id = cx.number(hwnd as i32);
    Ok(id)
}

pub fn build_from_template_with_config(mut cx: FunctionContext) -> JsResult<JsNumber> {
    if cx.len() != 3 {
        return cx.throw_error("Invalid number of arguments");
    }

    let parent = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let templates = cx.argument::<JsArray>(1)?.to_vec(&mut cx).unwrap();
    let config_obj = cx.argument::<JsObject>(2)?;

    let config = to_config(&mut cx, config_obj);

    let hwnd = build(&mut cx, parent, templates, config);

    let id = cx.number(hwnd as i32);
    Ok(id)
}

// #[allow(dead_code)]
// unsafe extern "C" fn b(data: *mut std::ffi::c_void) -> i32 {
//     let data: &mut Data = unsafe { &mut *(data as *mut Data) };

//     init();
//     println!("parent:{:?}", data.parent);

//     let mut builder = MenuBuilder::new_from_config(data.parent, data.config.clone());
//     println!("parent2");
//     build_menu(&mut builder, &data.items);
//     let menu = builder.build().unwrap();

//     let mut map = MENU_MAP.try_lock().unwrap();

//     let inner = menu.gtk_menu_handle;
//     (*map).insert(inner as i32, menu);

//     0
// }

fn build(cx: &mut FunctionContext, parent: f64, templates: Vec<Handle<JsValue>>, config: Config) -> isize {
    let items: Vec<ElectronMenuItem> = templates
        .into_iter()
        .map(|value| {
            let v = value.downcast_or_throw::<JsObject, _>(cx).unwrap();
            ElectronMenuItem::from_object(cx, v)
        })
        .collect();

    #[cfg(target_os = "linux")]
    {
        println!("linux:{:?}", parent);
        // let data = Data {
        //     parent: parent as isize,
        //     config: config.clone(),
        //     items: items.clone(),
        // };
        // let state_ptr: *mut std::ffi::c_void = &mut data as *mut _ as *mut std::ffi::c_void;
        // let x = Box::into_raw(Box::new(data));
        // idle_add(move || {
        println!("parent:{:?}", parent);
        // let d = gtk::gdk::Display::default().unwrap();
        // gdkx11::X11Display::ff
        // let display: *mut gdkx11::ffi::GdkX11Display = d.to_glib_none().0;
        // let xwin: *mut GdkWindow = gdkx11::X11Window::foreign_new_for_display(display, data.parent as u64).to_glib_full().0;
        // let _window = unsafe { gtk::gdk::Window::from_glib_full(xwin) };
        // let _window: ApplicationWindow = unsafe { ApplicationWindow::from_glib_none(data.parent as *mut GtkApplicationWindow) };
        // println!("ApplicationWindow");

        if gtk::init().is_ok() {
            println!("init");
            // let gdk_window: X11Window = unsafe { X11Window::from_glib_full(parent as usize as *mut GdkX11Window) };
            // let gdk_window: Window = unsafe { Window::from_glib_full(parent as usize as *mut GdkWindow) };

            // println!("gdk_window:{:?}", gdk_window);
            // let window: *mut GdkX11Window = unsafe { X11Window::from_glib_none(parent as isize) };
            // let x = unsafe { gdk_x11_window_get_xid(window) };
            // let x1 = Box::into_raw(Box::new(x));
            let x = unsafe { gdk_x11_get_default_xdisplay() };

            let gdkd = unsafe { gdk_x11_lookup_xdisplay(x) };
            let w = unsafe { gdk_x11_window_foreign_new_for_display(gdkd, parent as u64) };
            let gdk_window: Window = unsafe { Window::from_glib_full(w) };
            println!("window:{:?}", gdk_window);
            let window = gtk::Window::new(gtk::WindowType::Toplevel);
            window.connect_realize(glib::clone!(@weak gdk_window as wd => move |w| w.set_window(wd)));
            window.set_has_window(true);
            window.realize();
            window.reset_style();

            // let xwin: X11Window = gdk_window.dynamic_cast().unwrap();

            // println!("{:?}", xwin.xid());

            // let mut app: Option<Widget> = None;
            // println!("list_toplevels");
            // for x in gtk::Window::list_toplevels() {
            //     if let Some(win) = x.window() {
            //         println!("win:{:?}", win);

            //         let xwin: X11Window = win.dynamic_cast().unwrap();

            //         println!("{:?}", xwin.xid());
            //         // if win == gdk_window {
            //         //     println!("oik");
            //         // }
            //         app = Some(x);
            //     }
            // }

            // let w: gtk::Window = app.unwrap().dynamic_cast().unwrap();

            // println!("{:?}", w);
            // println!("{:?}", config.color.dark.background_color);
            // let r = wcpopup::config::rgba_from_hex(config.color.dark.background_color);
            // println!("{:?}", r);
            // let h = wcpopup::config::hex_from_rgba(r.r, r.g, r.b, r.a);
            // println!("{:?}", h);

            println!("parent2:{:?}", config.theme);
            let mut builder = MenuBuilder::new_for_window_from_config(&window, config);

            build_menu(&mut builder, &items);
            println!("parent3");
            let menu = builder.build().unwrap();
            println!("{:?}", menu);
            let mut map = MENU_MAP.try_lock().unwrap();

            let inner = menu.gtk_menu_handle;
            (*map).insert(inner as i32, menu);
            return inner;
        }
        // gtk::glib::ControlFlow::Continue
        // });
        // unsafe { g_idle_add(Some(b), x as *mut std::ffi::c_void) };
        0
    }

    #[cfg(target_os = "windows")]
    {
        let mut builder = MenuBuilder::new_from_config(parent as isize, config);

        build_menu(&mut builder, &items);
        let menu = builder.build().unwrap();

        let mut map = MENU_MAP.try_lock().unwrap();

        let inner = menu.window_handle;
        (*map).insert(inner as i32, menu);

        inner
    }
}

fn build_menu(builder: &mut MenuBuilder, items: &Vec<ElectronMenuItem>) {
    for item in items {
        let disabled = if item.enabled {
            None
        } else {
            Some(false)
        };
        match item.itype.as_str() {
            "normal" => {
                if item.accelerator.is_empty() {
                    builder.text(&item.id, &item.label, disabled);
                } else {
                    builder.text_with_accelerator(&item.id, &item.label, disabled, &item.accelerator);
                }
            }
            "separator" => {
                builder.separator();
            }
            "submenu" => {
                let mut parent = builder.submenu(&item.id, &item.label, disabled);
                build_menu(&mut parent, &item.submenu);
                let submenu = parent.build().unwrap();
                let mut map = MENU_MAP.try_lock().unwrap();
                #[cfg(target_os = "linux")]
                (*map).insert(submenu.gtk_menu_handle as i32, submenu);
                #[cfg(target_os = "windows")]
                (*map).insert(submenu.window_handle as i32, submenu);
                std::mem::drop(map);
            }
            "checkbox" => {
                if item.accelerator.is_empty() {
                    builder.check(&item.id, &item.label, item.checked, disabled);
                } else {
                    builder.check_with_accelerator(&item.id, &item.label, item.checked, disabled, &item.accelerator);
                }
            }
            "radio" => {
                if item.accelerator.is_empty() {
                    builder.radio(&item.id, &item.label, &item.name, item.checked, disabled);
                } else {
                    builder.radio_with_accelerator(&item.id, &item.label, &item.name, item.checked, disabled, &item.accelerator);
                }
            }
            _ => {}
        }
    }
}

pub fn popup(mut cx: FunctionContext) -> JsResult<JsPromise> {
    if cx.len() != 3 {
        return cx.throw_error("Invalid number of arguments");
    }

    let hwnd = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let x = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let y = cx.argument::<JsNumber>(2)?.value(&mut cx);

    let (deferred, promise) = cx.promise();
    let channel = cx.channel();

    #[cfg(target_os = "windows")]
    async_std::task::spawn(async move {
        let map = MENU_MAP.lock().await;
        let menu = map.get(&(hwnd as i32)).unwrap();
        let x = menu.popup_at_async(x as i32, y as i32).await;

        deferred.settle_with(&channel, |mut cx| match x {
            Some(data) => from_menu_item(&mut cx, &data),
            None => Ok(cx.empty_object()),
        });
    });
    #[cfg(target_os = "linux")]
    gtk::glib::spawn_future_local(async move {
        println!("enter");
        let map = MENU_MAP.lock().await;
        let menu = map.get(&(hwnd as i32)).unwrap();
        let x = menu.popup_at_async(x as i32, y as i32).await;
        println!("returned");
        deferred.settle_with(&channel, |mut cx| match x {
            Some(data) => from_menu_item(&mut cx, &data),
            None => Ok(cx.empty_object()),
        });
    });

    Ok(promise)
}

pub fn items(mut cx: FunctionContext) -> JsResult<JsArray> {
    let hwnd = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let map = MENU_MAP.try_lock().unwrap();
    let menu = map.get(&(hwnd as i32)).unwrap();
    let items = extract_item(&menu.items(), &mut cx)?;

    Ok(items)
}

pub fn remove(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let hwnd = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let jsitem = cx.argument::<JsObject>(1)?;

    let item = to_menu_item(&mut cx, jsitem);

    let mut map = MENU_MAP.try_lock().unwrap();
    let menu = map.get_mut(&(hwnd as i32)).unwrap();
    menu.remove(&item);

    Ok(cx.undefined())
}

pub fn remove_at(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let hwnd = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let index = cx.argument::<JsNumber>(1)?.value(&mut cx);

    let mut map = MENU_MAP.try_lock().unwrap();
    let menu = map.get_mut(&(hwnd as i32)).unwrap();
    menu.remove_at(index as u32);

    Ok(cx.undefined())
}

pub fn append(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let hwnd = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let jsitem = cx.argument::<JsObject>(1)?;

    let mut map = MENU_MAP.try_lock().unwrap();
    let menu = map.get_mut(&(hwnd as i32)).unwrap();
    let item = to_menu_item(&mut cx, jsitem);
    menu.append(item);

    Ok(cx.undefined())
}

pub fn insert(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let hwnd = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let index = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let jsitem = cx.argument::<JsObject>(2)?;

    let mut map = MENU_MAP.try_lock().unwrap();
    let menu = map.get_mut(&(hwnd as i32)).unwrap();
    let item = to_menu_item(&mut cx, jsitem);
    menu.insert(item, index as u32);

    Ok(cx.undefined())
}

pub fn get_menu_item_by_id(mut cx: FunctionContext) -> JsResult<JsObject> {
    let hwnd = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let id = cx.argument::<JsString>(1)?.value(&mut cx);

    let map = MENU_MAP.try_lock().unwrap();
    let menu = map.get(&(hwnd as i32)).unwrap();
    if let Some(item) = menu.get_menu_item_by_id(id.as_str()) {
        from_menu_item(&mut cx, &item)
    } else {
        Ok(cx.empty_object())
    }
}

pub fn get_default_config(mut cx: FunctionContext) -> JsResult<JsObject> {
    let configjs = from_config(&mut cx, &Config::default())?;
    Ok(configjs)
}

pub fn set_theme(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let id = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let theme_str = cx.argument::<JsString>(1)?.value(&mut cx);

    let theme = match theme_str.as_str() {
        "dark" => Theme::Dark,
        "light" => Theme::Light,
        "system" => Theme::System,
        _ => Theme::System,
    };

    let mut map = MENU_MAP.try_lock().unwrap();
    let menu = map.get_mut(&(id as i32)).unwrap();
    menu.set_theme(theme);

    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("buildFromTemplate", build_from_template)?;
    cx.export_function("buildFromTemplateWithTheme", build_from_template_with_theme)?;
    cx.export_function("buildFromTemplateWithConfig", build_from_template_with_config)?;
    cx.export_function("popup", popup)?;
    cx.export_function("items", items)?;
    cx.export_function("removeAt", remove_at)?;
    cx.export_function("remove", remove)?;
    cx.export_function("append", append)?;
    cx.export_function("insert", insert)?;
    cx.export_function("getDefaultConfig", get_default_config)?;
    cx.export_function("setTheme", set_theme)?;
    cx.export_function("getMenuItemById", get_menu_item_by_id)?;
    Ok(())
}
