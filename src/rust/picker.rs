use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

type CtxRefCount = Rc<web_sys::CanvasRenderingContext2d>;

pub fn initialise(document: &web_sys::Document, ctx: &CtxRefCount) -> Result<(), JsValue> {
   let colours = [
        "#F0F8FF", "#FAEBD7", "#00FFFF", "#7FFFD4", "#F0FFFF", "#F5F5DC", "#FFE4C4",
        "#000000", "#FFEBCD", "#0000FF", "#8A2BE2", "#A52A2A", "#DEB887", "#5F9EA0",
        "#7FFF00", "#D2691E", "#FF7F50", "#6495ED", "#FFF8DC", "#DC143C", "#00FFFF",
        "#00008B", "#008B8B", "#B8860B", "#A9A9A9", "#006400", "#BDB76B", "#8B008B",
        "#556B2F", "#FF8C00", "#9932CC", "#8B0000", "#E9967A", "#8FBC8F", "#483D8B",
        "#2F4F4F", "#00CED1", "#9400D3", "#FF1493", "#696969", "#1E90FF", "#B22222",
        "#FFFAF0", "#228B22", "#FF00FF", "#DCDCDC", "#F8F8FF", "#FFD700", "#DAA520",
    ];

    for colour in &colours[0..] {
        add_colour(&document, ctx,colour)?;
    }

    Ok(())
}

fn add_colour(document: &web_sys::Document, ctx: &CtxRefCount, rgb: &str) -> Result<(), JsValue> {
    let container = document.get_element_by_id("app--picker-container").unwrap();
    let element = document.create_element("div")?.dyn_into::<web_sys::HtmlDivElement>()?;
    let bg_colour = &["background-color: ", rgb, ";"].concat();
    let rgb_ref_count = Rc::new(rgb);

    element.set_attribute("class", "picker")?;
    element.set_attribute("style", bg_colour)?;
    set_mouse_event(&element, ctx, rgb_ref_count)?;
    container.append_child(&element)?;

    Ok(())
}

fn set_mouse_event(
    element: &web_sys::HtmlDivElement,
    ctx: &CtxRefCount,
    rgb: Rc<&str>
) -> Result<(), JsValue> {
    let ctx = ctx.clone();
    let rgb = rgb.clone();
    let val = JsValue::from_str(&rgb);

    let closure = Closure::wrap(
        Box::new(move |_event: web_sys::MouseEvent| {
            ctx.set_stroke_style(&val);
        }) as Box<dyn FnMut(_)>
    );

    element.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
    closure.forget();

    Ok(())
}
