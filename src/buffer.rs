use wasm_bindgen::prelude::*;
use web_sys::{WebGlBuffer, WebGlRenderingContext};

use crate::mesh;

pub fn vertex_buffer(context: &WebGlRenderingContext) -> Result<WebGlBuffer, JsValue> {
    let buffer = context.create_buffer().ok_or("create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let vert_array = js_sys::Float32Array::view(mesh::VERTEX);

        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    Ok(buffer)
}

pub fn index_buffer(context: &WebGlRenderingContext) -> Result<WebGlBuffer, JsValue> {
    let buffer = context.create_buffer().ok_or("create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let index_array = js_sys::Uint16Array::view(mesh::INDEX);

        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            &index_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    Ok(buffer)
}
