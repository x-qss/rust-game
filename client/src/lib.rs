use js_sys::Function;
use js_sys::Math;
use serde_json::json;
use serde_json::to_string;
use structs::camera::Camera;
use structs::player::Player;
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{
    AddEventListenerOptions, CanvasRenderingContext2d, Event, EventTarget, HtmlCanvasElement,
    MessageEvent, WebSocket, window,
};

use std::{thread, time};

use std::cell::RefCell;
use std::rc::Rc;

mod packets;
use crate::packets::{ClientPackets, IncomingPackets};
// console::log_1

pub mod structs {
    pub mod camera;
    pub mod player;
}

pub mod render;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = getPristineWebSocket)]
    fn get_pristine_websocket() -> Function;
}

/*
struct Ws(WebSocket);
impl Ws {
    fn open(&self, url: String) -> Result<Self, JsValue> {

        let ws: JsValue = get_pristine_websocket().call1(&JsValue::NULL, &JsValue::from_str("url"))?;
        let ws = ws.dyn_into::<WebSocket>()?;

        let onopen_callback = Closure::wrap(Box::new(move || {
            self.on_open();
        }) as Box<dyn FnMut()>);
        onopen_callback.forget();

        Ok(Ws(ws))
    }

    fn on_open(&self) {
        let ping = ClientPackets::Ping;
            let json = to_string(&ping).unwrap();

            self.send(&json);
    }

    //fn on_message

    fn send(&self, msg: &str) -> Result<(), ()> {
        if let Err(err) = self.0.send_with_str(&msg) {
            web_sys::console::log_1(&format!("error sending: {:?}", err).into());
        }

        Ok(())
    }
}



et ws = ws_constructor.call1(&JsValue::NULL, &JsValue::from_str("ws://localhost:8080"))?;
    let ws = ws.dyn_into::<WebSocket>()?;

    let ws_onopen = ws.clone();
    let onopen_callback = Closure::wrap(Box::new(move || {
        let ping: ClientPackets = ClientPackets::Ping;
        let json = to_string(&ping).unwrap();
        web_sys::console::log_1(&format!("sending: {json}").into());
        if let Err(err) = ws_onopen.send_with_str(&json) {
            web_sys::console::log_1(&format!("error sending message: {:?}", err).into());
        }
    }) as Box<dyn FnMut()>);
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();
*/

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
    let win = web_sys::window().expect("should have a Window");
    let win_clone = win.clone();
    let win_clone_2 = win.clone();

    match init_ws() {
        Ok(_) => println!("connected through ws"),
        Err(e) => println!("err: {:?}", e),
    }

    let mut cam: Camera = Camera { x: 0.0, y: 0.0 };
    let mut myplayer: Player = Player {
        uuid: 0,
        name: String::from("sdaasd"),
        x: 0.0,
        y: 0.0,
        vx: 0.0,
        vy: 0.0,
        lx: 0.0,
        ly: 0.0,
        scale: 30,
    };

    let key_down_callback = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
        let key = e.key();
        let key_code = e.key_code();
        let key_str = key.as_str();

        match key_code {
            _ => {
                web_sys::console::log_1(&format!("key: {key_str}, keycode: {key_code}").into());
            }
        }
    }) as Box<dyn FnMut(_)>);
    win_clone_2
        .add_event_listener_with_callback("keydown", key_down_callback.as_ref().unchecked_ref())
        .unwrap();

    key_down_callback.forget();

    /*move || {
        win_clone_2.remove_event_listener_with_callback(
            "keydown",
            key_down_callback.as_ref().unchecked_ref(),
        )
        .unwrap();
    };*/

    let animation_loop: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let animation_loop_2: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = animation_loop.clone();

    let animation_loop_clone = animation_loop.clone();

    let f = Rc::new(RefCell::new(0.0));
    let f_clone = f.clone();

    *animation_loop_clone.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let now = win_clone.performance().unwrap().now();
        let delta = (now - *f_clone.borrow()) / 1000.0;
        *f_clone.borrow_mut() = now;

        ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
        render::render(&ctx, delta, &mut myplayer, &canvas);

        win_clone
            .request_animation_frame(
                animation_loop
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .unchecked_ref(),
            )
            .unwrap();
    }) as Box<dyn FnMut()>));

    win.request_animation_frame(
        animation_loop_2
            .borrow()
            .as_ref()
            .unwrap()
            .as_ref()
            .unchecked_ref(),
    )?;

    //loop {
    //    render::render(&ctx, &myplayer);
    //}

    Ok(())
}

fn init_ws() -> Result<(), JsValue> {
    let ws_constructor = get_pristine_websocket();
    let ws = ws_constructor.call1(&JsValue::NULL, &JsValue::from_str("ws://localhost:8080"))?;
    let ws = ws.dyn_into::<WebSocket>()?;

    let ws_onopen = ws.clone();
    let onopen_callback = Closure::wrap(Box::new(move || {
        let ping: ClientPackets = ClientPackets::Ping;
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
