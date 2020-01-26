mod utilities;
mod canvas;
mod picker;

use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn render(height: u32, width: u32) -> Result<(), JsValue> {
    // For better error messages in the browser console
    utilities::set_panic_hook();

    let document = web_sys::window().unwrap().document().unwrap();

    let canvas_tuple = canvas::initialise(&document, height, width)?;

    let canvas = canvas_tuple.0;
    let context = canvas_tuple.1;

    let ctx = Rc::new(context);
    let pressed = Rc::new(Cell::new(false));

    picker::initialise(&document, &ctx)?;

    canvas::initialise_mouse_events(&canvas, &ctx, &pressed)?;

    Ok(())
}
