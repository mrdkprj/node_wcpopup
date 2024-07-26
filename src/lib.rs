use async_std::sync::Mutex;
use neon::prelude::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use wcpopup::{Config, Menu, MenuBuilder, Theme};
use windows::Win32::Foundation::HWND;
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

fn build(
    cx: &mut FunctionContext,
    parent: f64,
    templates: Vec<Handle<JsValue>>,
    config: Config,
) -> isize {
    let items: Vec<ElectronMenuItem> = templates
        .into_iter()
        .map(|value| {
            let v = value.downcast_or_throw::<JsObject, _>(cx).unwrap();
            ElectronMenuItem::from_object(cx, v)
        })
        .collect();

    let hwnd = HWND(parent as isize);
    let mut builder = MenuBuilder::new_from_config(hwnd, config);

    build_menu(&mut builder, &items);

    let menu = builder.build().unwrap();
    let mut map = MENU_MAP.try_lock().unwrap();
    let inner = menu.hwnd.0;
    (*map).insert(inner as i32, menu);

    inner
}

fn build_menu(builder: &mut MenuBuilder, items: &Vec<ElectronMenuItem>) {
    for item in items {
        let disabled = if item.enabled { None } else { Some(false) };
        match item.itype.as_str() {
            "normal" => {
                if item.accelerator.is_empty() {
                    builder.text(&item.id, &item.label, disabled);
                } else {
                    builder.text_with_accelerator(
                        &item.id,
                        &item.label,
                        disabled,
                        &item.accelerator,
                    );
                }
            }
            "separator" => {
                builder.separator();
            }
            "submenu" => {
                let mut parent = builder.submenu(&item.label, disabled);
                build_menu(&mut parent, &item.submenu);
                parent.build().unwrap();
            }
            "checkbox" => {
                if item.accelerator.is_empty() {
                    builder.check(&item.id, &item.label, &item.value, item.checked, disabled);
                } else {
                    builder.check_with_accelerator(
                        &item.id,
                        &item.label,
                        &item.value,
                        item.checked,
                        disabled,
                        &item.accelerator,
                    );
                }
            }
            "radio" => {
                if item.accelerator.is_empty() {
                    builder.radio(
                        &item.id,
                        &item.label,
                        &item.value,
                        &item.name,
                        item.checked,
                        disabled,
                    );
                } else {
                    builder.radio_with_accelerator(
                        &item.id,
                        &item.label,
                        &item.value,
                        &item.name,
                        item.checked,
                        disabled,
                        &item.accelerator,
                    );
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

    let id = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let x = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let y = cx.argument::<JsNumber>(2)?.value(&mut cx);

    let (deferred, promise) = cx.promise();
    let channel = cx.channel();

    async_std::task::spawn(async move {
        let map = MENU_MAP.lock().await;
        let menu = map.get(&(id as i32)).unwrap();
        let x = menu.popup_at_async(x as i32, y as i32).await;

        deferred.settle_with(&channel, |mut cx| match x {
            Some(data) => from_selected_item(&mut cx, &data),
            None => Ok(cx.empty_object()),
        });
    });

    Ok(promise)
}

pub fn popup_sync(mut cx: FunctionContext) -> JsResult<JsObject> {
    if cx.len() != 3 {
        return cx.throw_error("Invalid number of arguments");
    }

    let id = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let x = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let y = cx.argument::<JsNumber>(2)?.value(&mut cx);

    let map = MENU_MAP.try_lock().unwrap();
    let menu = map.get(&(id as i32)).unwrap();
    let x = menu.popup_at(x as i32, y as i32);

    let result = match x {
        Some(data) => from_selected_item(&mut cx, &data),
        None => Ok(cx.empty_object()),
    };

    result
}

pub fn items(mut cx: FunctionContext) -> JsResult<JsArray> {
    let id = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let map = MENU_MAP.try_lock().unwrap();
    let menu = map.get(&(id as i32)).unwrap();
    let items = extract_item(&menu.items(), &mut cx)?;

    Ok(items)
}

pub fn remove(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let id = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let index = cx.argument::<JsNumber>(1)?.value(&mut cx);

    let mut map = MENU_MAP.try_lock().unwrap();
    let menu = map.get_mut(&(id as i32)).unwrap();
    menu.remove(index as u32);

    Ok(cx.undefined())
}

pub fn append(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let id = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let jsitem = cx.argument::<JsObject>(1)?;

    let mut map = MENU_MAP.try_lock().unwrap();
    let menu = map.get_mut(&(id as i32)).unwrap();
    let item = to_menu_item(&mut cx, jsitem);
    menu.append(item);

    Ok(cx.undefined())
}

pub fn insert(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let id = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let index = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let jsitem = cx.argument::<JsObject>(2)?;

    let mut map = MENU_MAP.try_lock().unwrap();
    let menu = map.get_mut(&(id as i32)).unwrap();
    let item = to_menu_item(&mut cx, jsitem);
    menu.insert(item, index as u32);

    Ok(cx.undefined())
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
    cx.export_function(
        "buildFromTemplateWithConfig",
        build_from_template_with_config,
    )?;
    cx.export_function("popup", popup)?;
    cx.export_function("popupSync", popup_sync)?;
    cx.export_function("items", items)?;
    cx.export_function("remove", remove)?;
    cx.export_function("append", append)?;
    cx.export_function("insert", insert)?;
    cx.export_function("getDefaultConfig", get_default_config)?;
    cx.export_function("setTheme", set_theme)?;
    Ok(())
}
