mod utilities;

use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn render(height: u32, width: u32) -> Result<(), JsValue> {
    // For better error messages in the browser console
    utilities::set_panic_hook();

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas_tuple = initialise_canvas(&document, height, width)?;
    let canvas = canvas_tuple.0;
    let context = canvas_tuple.1;

    let ctx = Rc::new(context);
    let pressed = Rc::new(Cell::new(false));

    set_mouse_down_event(&canvas,&ctx, &pressed)?;
    set_mouse_move_event(&canvas, &ctx, &pressed)?;
    set_mouse_up_event(&canvas, &ctx, &pressed)?;

    Ok(())
}

type Canvas = (web_sys::HtmlCanvasElement, web_sys::CanvasRenderingContext2d);
type CtxRefCount = Rc<web_sys::CanvasRenderingContext2d>;
type PressedRefCount = Rc<Cell<bool>>;

fn initialise_canvas(document: &web_sys::Document, height: u32, width: u32) -> Result<Canvas, JsValue> {
    let canvas = document.create_element("canvas")?.dyn_into::<web_sys::HtmlCanvasElement>()?;
    canvas.set_height(height);
    canvas.set_width(width);

    let canvas_container = document.get_element_by_id("app--canvas-container").unwrap();
    canvas_container.append_child(&canvas)?;

    let ctx = canvas.get_context("2d")?.unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    Ok((canvas, ctx))
}

// TODO add touch / multi-touch support if I don't get lazy again
fn set_mouse_down_event(
    canvas: &web_sys::HtmlCanvasElement,
    ctx: &CtxRefCount,
    pressed: &PressedRefCount
) -> Result<(), JsValue> {
    let ctx = ctx.clone();
    let pressed = pressed.clone();

    let closure = Closure::wrap(
        Box::new(move |event: web_sys::MouseEvent| {
            ctx.begin_path();
            ctx.move_to(event.offset_x() as f64, event.offset_y() as f64);
            pressed.set(true);
        }) as Box<dyn FnMut(_)>
    );

    canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
    closure.forget();

    Ok(())
}

fn set_mouse_move_event(
    canvas: &web_sys::HtmlCanvasElement,
    ctx: &CtxRefCount,
    pressed: &PressedRefCount
) -> Result<(), JsValue> {
    let ctx = ctx.clone();
    let pressed = pressed.clone();

    let closure = Closure::wrap(
        Box::new(move |event: web_sys::MouseEvent| {
            let offset_x = event.offset_x() as f64;
            let offset_y = event.offset_y() as f64;
            if !pressed.get() { return (); }
            ctx.line_to(offset_x, offset_y);
            ctx.stroke();
            ctx.begin_path();
            ctx.move_to(offset_x, offset_y);
        }) as Box<dyn FnMut(_)>
    );

    canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
    closure.forget();

    Ok(())
}

fn set_mouse_up_event(
    canvas: &web_sys::HtmlCanvasElement,
    ctx: &CtxRefCount,
    pressed: &PressedRefCount
) -> Result<(), JsValue> {
    let ctx = ctx.clone();
    let pressed = pressed.clone();

    let closure = Closure::wrap(
        Box::new(move |event: web_sys::MouseEvent| {
            pressed.set(false);
            ctx.line_to(event.offset_x() as f64, event.offset_y() as f64);
            ctx.stroke();
        }) as Box<dyn FnMut(_)>
    );

    canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
    closure.forget();

    Ok(())
}
