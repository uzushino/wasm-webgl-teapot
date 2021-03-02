use wasm_bindgen::prelude::*;
use web_sys::{WebGlBuffer, WebGlRenderingContext};

pub fn vertex_buffer(context: &WebGlRenderingContext, vertex: &[f32]) -> Result<WebGlBuffer, JsValue> {
    let buffer = context.create_buffer().ok_or("create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let vert_array = js_sys::Float32Array::view(vertex);

        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    Ok(buffer)
}

pub fn index_buffer(context: &WebGlRenderingContext, indexes: &[u16]) -> Result<WebGlBuffer, JsValue> {
    let buffer = context.create_buffer().ok_or("create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let index_array = js_sys::Uint16Array::view(indexes);

        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            &index_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    Ok(buffer)
}

pub fn render_buffer(context: &WebGlRenderingContext, buffer: Option<&WebGlBuffer>, position: i32, num_vertex: i32) -> Result<(), JsValue> {
    context
        .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, buffer);

    context
        .enable_vertex_attrib_array(position as u32);

    context.vertex_attrib_pointer_with_i32(
        position as u32,
        num_vertex,
        WebGlRenderingContext::FLOAT,
        false,
        0,
        0,
    );
    
    Ok(())
}
