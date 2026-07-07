use js_sys::{Array, JsString, Uint8Array};
use std::borrow::Borrow;
use wasm_bindgen::{JsCast, JsValue};

pub trait MessageSend {
    type Output: MessageReturn;

    fn into_js<A>(value: A, transfer_list: &Array) -> JsValue
    where
        A: Borrow<Self>;
}

pub trait MessageReturn {
    fn from_js(value: &JsValue) -> Self;
}

impl MessageSend for JsValue {
    type Output = JsValue;

    fn into_js<A>(value: A, _transfer_list: &Array) -> JsValue
    where
        A: Borrow<Self>,
    {
        value.borrow().to_owned()
    }
}

impl MessageReturn for JsValue {
    fn from_js(value: &JsValue) -> Self {
        value.clone()
    }
}

impl MessageSend for Vec<u8> {
    type Output = Vec<u8>;

    fn into_js<A>(value: A, transfer_list: &Array) -> JsValue
    where
        A: Borrow<Self>,
    {
        let value = Uint8Array::from(value.borrow().as_slice());
        transfer_list.push(&value.buffer());
        value.into()
    }
}

impl MessageReturn for Vec<u8> {
    fn from_js(value: &JsValue) -> Self {
        let value: &Uint8Array = value.unchecked_ref();
        value.to_vec()
    }
}

impl MessageSend for Option<u8> {
    type Output = Option<u8>;

    fn into_js<A>(value: A, _transfer_list: &Array) -> JsValue
    where
        A: Borrow<Self>,
    {
        JsValue::from(*value.borrow())
    }
}

impl MessageReturn for Option<u8> {
    fn from_js(value: &JsValue) -> Self {
        value.as_f64().map(|f| f as u8)
    }
}

impl MessageSend for bool {
    type Output = bool;

    fn into_js<A>(value: A, _transfer_list: &Array) -> JsValue
    where
        A: Borrow<Self>,
    {
        JsValue::from(*value.borrow())
    }
}

impl MessageReturn for bool {
    fn from_js(value: &JsValue) -> Self {
        value.as_bool().unwrap_or_default()
    }
}

impl MessageSend for String {
    type Output = String;

    fn into_js<A>(value: A, _transfer_list: &Array) -> JsValue
    where
        A: Borrow<Self>,
    {
        JsValue::from(value.borrow())
    }
}

impl MessageReturn for String {
    fn from_js(value: &JsValue) -> Self {
        value.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default()
    }
}

impl MessageSend for Option<String> {
    type Output = Option<String>;

    fn into_js<A>(value: A, _transfer_list: &Array) -> JsValue
    where
        A: Borrow<Self>,
    {
        JsValue::from(value.borrow().to_owned())
    }
}

impl MessageReturn for Option<String> {
    fn from_js(value: &JsValue) -> Self {
        value.dyn_ref::<JsString>().map(|s| s.into())
    }
}

macro_rules! impl_number {
    ($($t:ty),*) => {
        $(impl MessageSend for $t {
            type Output = $t;

            fn into_js<A>(value: A, _transfer_list: &Array) -> JsValue where A: Borrow<Self> {
                JsValue::from(*value.borrow())
            }
        }

        impl MessageReturn for $t {
            fn from_js(value: &JsValue) -> Self {
                let value = value.as_f64().unwrap_or_default();
                value as $t
            }
        })*
    };
}

impl_number!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64);
