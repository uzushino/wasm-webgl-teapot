use wasm_bindgen::prelude::*;
use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader};

pub fn vertex_shader(context: &WebGl2RenderingContext) -> Result<WebGlShader, JsValue> {
    let vert_shader = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        r#"
        precision mediump float;

        attribute vec3 aVertexPosition;

        uniform mat4 uPMatrix;
        uniform mat4 uMVMatrix;

        varying highp vec3 vVertexPosition;

        void main(void) {
            gl_Position = uPMatrix * uMVMatrix * vec4(aVertexPosition, 1.0);
            vVertexPosition = aVertexPosition;
        }
        "#,
    )?;

    Ok(vert_shader)
}

pub fn fragment_shader(context: &WebGl2RenderingContext) -> Result<WebGlShader, JsValue> {
    let frag_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r#"
        precision mediump float;

        varying highp vec3 vVertexPosition;
        
        void main (void) {
            gl_FragColor = vec4(1.0, 1.0, 1.0, 0.8);
        }
        "#,
    )?;

    Ok(frag_shader)
}

pub fn create_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("create program error"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    let check = context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false);

    if check {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("create program error")))
    }
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("create shader error"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    let check = context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false);

    if check {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("compile shader error")))
    }
}
