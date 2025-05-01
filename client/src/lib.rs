use serde_json::json;
use serde_json::to_string;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MessageEvent, WebSocket, window};

mod packets;
use crate::packets::{ClientPackets, IncomingPackets};
// console::log_1

pub mod structs {
    pub mod player;
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let doc = window().unwrap().document().unwrap();
    let canvas = doc
        .get_element_by_id("ctx")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;
    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    ctx.set_fill_style_str("red");
    ctx.fill_rect(0.0, 0.0, 90.0, 90.0);

    match init_ws() {
        Ok(_) => println!("connected through ws"),
        Err(e) => println!("err: {:?}", e),
    };

    Ok(())
}

fn init_ws() -> Result<(), JsValue> {
    let ws = WebSocket::new("ws://localhost:8080").unwrap();

    //ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
    let ws_onopen = ws.clone();
    let onopen_callback = Closure::wrap(Box::new(move || {
        /*let ping = json!({
            "type": "Ping"
        });*/

        let ping: ClientPackets = ClientPackets::Ping;
        //let dd = ClientPackets::Move { direction: 3.4 };
        let json = to_string(&ping).unwrap();
        web_sys::console::log_1(&format!("sending: {json}").into());
        if let Err(err) = ws_onopen.send_with_str(&json) {
            web_sys::console::log_1(&format!("error sending message: {:?}", err).into());
        }
    }) as Box<dyn FnMut()>);
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        if let Some(txt) = e.data().as_string() {
            web_sys::console::log_1(&format!("rec: {txt}").into());
        }
    }) as Box<dyn FnMut(_)>);

    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();

    Ok(())
}
