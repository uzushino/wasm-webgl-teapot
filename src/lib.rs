use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ WebGlRenderingContext, KeyboardEvent};

pub mod buffer;
pub mod log;
pub mod teapot;
pub mod cube;
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
        .dyn_into::<WebGlRenderingContext>()?;

    let w = canvas.client_width();
    let h = canvas.client_height();
    context.viewport(0, 0, w, h);

    init(w, h, &context)?;

    context.clear_color(0.0, 0.0, 0.0, 1.0);


    Ok(())
}

fn init(width: i32, height: i32, context: &WebGlRenderingContext) -> Result<(), JsValue> {
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear_depth(1.0);
    context.enable(WebGlRenderingContext::DEPTH_TEST);
    context.depth_func(WebGlRenderingContext::LEQUAL);
    context
        .clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);

    let window = web_sys::window().unwrap();
    let mut scene = scene::Scene::new_with_context(width, height, context)?;
    
    let keydown= Closure::wrap(Box::new(|e: KeyboardEvent| {
        
    }) as Box<dyn FnMut(KeyboardEvent)>);
    window.set_onkeydown(Some(keydown.as_ref().unchecked_ref()));
    
    keydown.forget();
    scene.render()?;

    Ok(())
}
