use async_std::sync::Mutex;
use neon::prelude::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use wcpopup::{
    ColorScheme, Config, Corner, Menu, MenuBuilder, MenuItem, MenuItemType, MenuSize,
    SelectedMenuItem, Theme, ThemeColor,
};
use windows::Win32::Foundation::HWND;

static MENU_MAP: Lazy<Mutex<HashMap<i32, Menu>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Debug)]
struct ElectronMenuItem {
    itype: String,
    label: String,
    accelerator: String,
    enabled: bool,
    #[allow(dead_code)]
    visible: bool,
    checked: bool,
    submenu: Vec<ElectronMenuItem>,
    id: String,
    value: String,
    name: String,
}

impl ElectronMenuItem {
    fn from_object(cx: &mut FunctionContext, value: Handle<JsObject>) -> Self {
        Self {
            itype: to_string(cx, &value, "type"),
            label: to_string(cx, &value, "label"),
            accelerator: to_string(cx, &value, "accelerator"),
            enabled: to_bool(cx, &value, "enabled", true),
            visible: to_bool(cx, &value, "visible", true),
            checked: to_bool(cx, &value, "checked", false),
            submenu: value
                .get_opt::<JsArray, _, _>(cx, "submenu")
                .unwrap()
                .unwrap_or_else(|| JsArray::new(cx, 0))
                .to_vec(cx)
                .unwrap()
                .into_iter()
                .map(|value| {
                    let v = value.downcast_or_throw::<JsObject, _>(cx).unwrap();

                    ElectronMenuItem::from_object(cx, v)
                })
                .collect(),
            id: to_string(cx, &value, "id"),
            value: to_string(cx, &value, "value"),
            name: to_string(cx, &value, "name"),
        }
    }
}

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
    let is_dark = cx.argument::<JsBoolean>(2)?.value(&mut cx);

    let mut config = Config::default();
    config.theme = if is_dark { Theme::Dark } else { Theme::Light };

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

fn to_string(cx: &mut FunctionContext, value: &Handle<JsObject>, key: &str) -> String {
    value
        .get_opt::<JsString, _, _>(cx, key)
        .unwrap()
        .unwrap_or_else(|| JsString::new(cx, ""))
        .value(cx)
}

fn to_bool(cx: &mut FunctionContext, value: &Handle<JsObject>, key: &str, def: bool) -> bool {
    value
        .get_opt::<JsBoolean, _, _>(cx, key)
        .unwrap()
        .unwrap_or_else(|| JsBoolean::new(cx, def))
        .value(cx)
}

fn to_i32(cx: &mut FunctionContext, value: &Handle<JsObject>, key: &str) -> i32 {
    value
        .get_opt::<JsNumber, _, _>(cx, key)
        .unwrap()
        .unwrap_or_else(|| JsNumber::new(cx, 0))
        .value(cx) as i32
}

fn to_menu_item(cx: &mut FunctionContext, value: Handle<JsObject>) -> MenuItem {
    let id = to_string(cx, &value, "id");
    let label = to_string(cx, &value, "label");
    let item_value = to_string(cx, &value, "value");
    let accelerator_str = to_string(cx, &value, "accelerator");
    let name = to_string(cx, &value, "name");
    let enabled = to_bool(cx, &value, "enabled", true);
    let checked = to_bool(cx, &value, "checked", false);

    let accelerator = if accelerator_str.is_empty() {
        None
    } else {
        Some(accelerator_str.as_str())
    };
    let disabled = if enabled { None } else { Some(true) };

    let item_type_str = value
        .get_opt::<JsString, _, _>(cx, "type")
        .unwrap()
        .unwrap_or_else(|| JsString::new(cx, ""))
        .value(cx);

    let menu_item_type = match item_type_str.as_str() {
        "normal" => MenuItemType::Text,
        "separator" => MenuItemType::Separator,
        "submenu" => MenuItemType::Submenu,
        "checkbox" => MenuItemType::Checkbox,
        "radio" => MenuItemType::Radio,
        _ => MenuItemType::Text,
    };

    match menu_item_type {
        MenuItemType::Text => {
            MenuItem::new_text_item(&id, &label, &item_value, accelerator, disabled)
        }
        MenuItemType::Separator => MenuItem::new_separator(),
        MenuItemType::Submenu => {
            MenuItem::new_text_item(&id, &label, &item_value, accelerator, disabled)
        }
        MenuItemType::Checkbox => {
            MenuItem::new_check_item(&id, &label, &item_value, accelerator, checked, disabled)
        }
        MenuItemType::Radio => MenuItem::new_radio_item(
            &id,
            &label,
            &item_value,
            &name,
            accelerator,
            checked,
            disabled,
        ),
    }
}

fn extract_item<'a, C: Context<'a>>(vec: &Vec<MenuItem>, cx: &mut C) -> JsResult<'a, JsArray> {
    let items = JsArray::new(cx, vec.len());
    for (index, item) in vec.iter().enumerate() {
        if item.menu_item_type == MenuItemType::Submenu {
            let obj = from_menu_item(cx, item)?;
            items.set(cx, index as u32, obj)?;
            let submenus = item.submenu.as_ref().unwrap().items();
            extract_item(&submenus, cx)?;
        } else {
            let obj = from_menu_item(cx, item)?;
            items.set(cx, index as u32, obj)?;
        }
    }

    Ok(items)
}

fn from_menu_item<'a, C: Context<'a>>(cx: &mut C, item: &MenuItem) -> JsResult<'a, JsObject> {
    let obj = cx.empty_object();

    let id = cx.string(item.id.clone());
    obj.set(cx, "id", id)?;

    let label = cx.string(item.label.clone());
    obj.set(cx, "label", label)?;

    let value = cx.string(item.value.clone());
    obj.set(cx, "value", value)?;

    let name = cx.string(item.name.clone());
    obj.set(cx, "name", name)?;

    let checked = cx.boolean(item.checked());
    obj.set(cx, "checked", checked)?;

    let enabled = cx.boolean(!item.disabled());
    obj.set(cx, "enabled", enabled)?;

    Ok(obj)
}

fn from_selected_item<'a, C: Context<'a>>(
    cx: &mut C,
    item: &SelectedMenuItem,
) -> JsResult<'a, JsObject> {
    let obj = cx.empty_object();

    let id = cx.string(item.id.clone());
    obj.set(cx, "id", id)?;

    let label = cx.string(item.label.clone());
    obj.set(cx, "label", label)?;

    let value = cx.string(item.value.clone());
    obj.set(cx, "value", value)?;

    let name = cx.string(item.name.clone());
    obj.set(cx, "name", name)?;

    let checked = cx.boolean(item.checked);
    obj.set(cx, "checked", checked)?;

    Ok(obj)
}

fn to_config(cx: &mut FunctionContext, value: Handle<JsObject>) -> Config {
    let theme = if to_string(cx, &value, "theme") == "Dark" {
        Theme::Dark
    } else {
        Theme::Light
    };

    let size_obj = value.get::<JsObject, _, _>(cx, "size").unwrap();
    let font_size = to_i32(cx, &size_obj, "fontSize");
    let font_weight = to_i32(cx, &size_obj, "fontWeight");

    let size = MenuSize {
        border_size: to_i32(cx, &size_obj, "borderSize"),
        vertical_margin: to_i32(cx, &size_obj, "verticalMargin"),
        horizontal_margin: to_i32(cx, &size_obj, "horizontalMargin"),
        item_vertical_padding: to_i32(cx, &size_obj, "itemVerticalPadding"),
        item_horizontal_padding: to_i32(cx, &size_obj, "itemHorizontalPadding"),
        font_size: if font_size > 0 { Some(font_size) } else { None },
        font_weight: if font_weight > 0 {
            Some(font_weight)
        } else {
            None
        },
    };

    let color_obj = value.get::<JsObject, _, _>(cx, "color").unwrap();
    let dark_color_scheme_obj = color_obj.get::<JsObject, _, _>(cx, "dark").unwrap();
    let light_color_scheme_obj = color_obj.get::<JsObject, _, _>(cx, "light").unwrap();

    let dark = ColorScheme {
        color: to_i32(cx, &dark_color_scheme_obj, "color") as u32,
        border: to_i32(cx, &dark_color_scheme_obj, "border") as u32,
        disabled: to_i32(cx, &dark_color_scheme_obj, "disabled") as u32,
        background_color: to_i32(cx, &dark_color_scheme_obj, "backgroundColor") as u32,
        hover_background_color: to_i32(cx, &dark_color_scheme_obj, "hoverBackgroundColor") as u32,
    };

    let light = ColorScheme {
        color: to_i32(cx, &light_color_scheme_obj, "color") as u32,
        border: to_i32(cx, &light_color_scheme_obj, "border") as u32,
        disabled: to_i32(cx, &light_color_scheme_obj, "disabled") as u32,
        background_color: to_i32(cx, &light_color_scheme_obj, "backgroundColor") as u32,
        hover_background_color: to_i32(cx, &light_color_scheme_obj, "hoverBackgroundColor") as u32,
    };

    let color = ThemeColor { dark, light };

    let corner = if to_string(cx, &value, "corner") == "Round" {
        Corner::Round
    } else {
        Corner::DoNotRound
    };

    Config {
        theme,
        size,
        color,
        corner,
    }
}

fn from_config<'a, C: Context<'a>>(cx: &mut C, config: &Config) -> JsResult<'a, JsObject> {
    let configjs = cx.empty_object();

    let theme = cx.string(if config.theme == Theme::Dark {
        "Dark"
    } else {
        "Light"
    });
    configjs.set(cx, "theme", theme)?;

    let size = cx.empty_object();
    let a = cx.number(config.size.border_size);
    size.set(cx, "borderSize", a)?;
    let a = cx.number(config.size.vertical_margin);
    size.set(cx, "verticalMargin", a)?;
    let a = cx.number(config.size.horizontal_margin);
    size.set(cx, "horizontalMargin", a)?;
    let a = cx.number(config.size.item_vertical_padding);
    size.set(cx, "itemVerticalPadding", a)?;
    let a = cx.number(config.size.item_horizontal_padding);
    size.set(cx, "itemHorizontalPadding", a)?;
    let a = cx.number(config.size.font_size.unwrap_or_else(|| 0));
    size.set(cx, "fontSize", a)?;
    let a = cx.number(config.size.font_weight.unwrap_or_else(|| 0));
    size.set(cx, "fontWeight", a)?;

    configjs.set(cx, "size", size)?;

    let color = cx.empty_object();
    let dark = cx.empty_object();
    let a = cx.number(config.color.dark.color);
    dark.set(cx, "color", a)?;
    let a = cx.number(config.color.dark.border);
    dark.set(cx, "border", a)?;
    let a = cx.number(config.color.dark.disabled);
    dark.set(cx, "disabled", a)?;
    let a = cx.number(config.color.dark.background_color);
    dark.set(cx, "backgroundColor", a)?;
    let a = cx.number(config.color.dark.hover_background_color);
    dark.set(cx, "hoverBackgroundColor", a)?;
    color.set(cx, "dark", dark)?;

    let light = cx.empty_object();
    let a = cx.number(config.color.light.color);
    light.set(cx, "color", a)?;
    let a = cx.number(config.color.light.border);
    light.set(cx, "border", a)?;
    let a = cx.number(config.color.light.disabled);
    light.set(cx, "disabled", a)?;
    let a = cx.number(config.color.light.background_color);
    light.set(cx, "backgroundColor", a)?;
    let a = cx.number(config.color.light.hover_background_color);
    light.set(cx, "hoverBackgroundColor", a)?;
    color.set(cx, "light", light)?;

    configjs.set(cx, "color", color)?;

    let corner = cx.string(if config.corner == Corner::Round {
        "Round"
    } else {
        "DoNotRound"
    });
    configjs.set(cx, "corner", corner)?;

    Ok(configjs)
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
    Ok(())
}
