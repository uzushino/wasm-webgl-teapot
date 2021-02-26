use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

pub mod buffer;
pub mod log;
pub mod mesh;
pub mod scene;
pub mod shader;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let w = canvas.client_width();
    let h = canvas.client_height();
    context.viewport(0, 0, w, h);

    init(w, h, &context)?;

    context.clear_color(0.0, 0.0, 0.0, 1.0);

    Ok(())
}

fn init(width: i32, height: i32, context: &WebGl2RenderingContext) -> Result<(), JsValue> {
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.enable(WebGl2RenderingContext::DEPTH_TEST);
    context.depth_func(WebGl2RenderingContext::LEQUAL);
    context
        .clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);

    let mut scene = scene::Scene::new_with_context(width, height, context)?;
    scene.render()?;

    Ok(())
}
