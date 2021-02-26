use wasm_bindgen::prelude::*;
use web_sys::{WebGlBuffer, WebGl2RenderingContext};

use crate::mesh;

pub fn vertex_buffer(context: &WebGl2RenderingContext, vertex: &[f32]) -> Result<WebGlBuffer, JsValue> {
    let buffer = context.create_buffer().ok_or("create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let vert_array = js_sys::Float32Array::view(vertex);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    Ok(buffer)
}

pub fn index_buffer(context: &WebGl2RenderingContext, indexes: &[u16]) -> Result<WebGlBuffer, JsValue> {
    let buffer = context.create_buffer().ok_or("create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let index_array = js_sys::Uint16Array::view(indexes);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            &index_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    Ok(buffer)
}
