use neon::prelude::*;
use neon::{
    context::{Context, TaskContext},
    result::JsResult,
    types::JsObject,
};

#[derive(Clone)]
pub struct Data {
    pub id: i32,
    pub id1: u32,
    pub data: i32,
}

impl Data {
    pub fn new() -> Self {
        Self {
            id: 0,
            id1: 0,
            data: 0,
        }
    }

    pub fn inc(&mut self) -> &Self {
        self.data += 1;
        self
    }

    pub fn dec(&mut self) -> &Self {
        self.data -= 1;
        self
    }

    pub fn to_object<'a>(&self, cx: &mut TaskContext<'a>) -> JsResult<'a, JsObject> {
        let obj = cx.empty_object();

        let title = cx.number(self.id1);
        obj.set(cx, "id1", title)?;

        let year = cx.number(self.data);
        obj.set(cx, "data", year)?;

        let id = cx.number(self.id);
        obj.set(cx, "id", id)?;

        Ok(obj)
    }
}

pub fn start() -> windows::core::Result<Data> {
    Ok(Data::new())
}
