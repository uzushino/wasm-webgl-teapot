use wasm_bindgen::prelude::*;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

pub fn vertex_shader(context: &WebGlRenderingContext) -> Result<WebGlShader, JsValue> {
    let vert_shader = compile_shader(
        &context,
        WebGlRenderingContext::VERTEX_SHADER,
        r#"
        attribute vec3 aPosition;
        attribute vec3 aNormal;
        attribute vec4 aColor;
        uniform   mat4 uModelMatrix;
        uniform   mat4 uMVPMatrix;
        varying   vec3 vPosition;
        varying   vec3 vNormal;
        varying   vec4 vColor;
        
        void main(void){
            vPosition   = (uModelMatrix * vec4(aPosition, 1.0)).xyz;
            vNormal     = (uModelMatrix * vec4(aNormal, 0.0)).xyz;
            vColor      = aColor;
            gl_Position = uMVPMatrix * vec4(aPosition, 1.0);
        }
        "#,
    )?;

    Ok(vert_shader)
}

pub fn fragment_shader(context: &WebGlRenderingContext) -> Result<WebGlShader, JsValue> {
    let frag_shader = compile_shader(
        &context,
        WebGlRenderingContext::FRAGMENT_SHADER,
        r#"
        precision mediump float;

        uniform vec3        eyePosition;
        uniform samplerCube cubeTexture;
        varying vec3        vPosition;
        varying vec3        vNormal;
        varying vec4        vColor;
        
        void main(void){
            vec3 direction = vPosition - eyePosition;
            vec3 ref       = reflect(direction, vNormal);
            vec4 envColor  = textureCube(cubeTexture, ref);
            vec4 destColor = vColor * envColor;
            gl_FragColor   = destColor;
        }
        "#,
    )?;

    Ok(frag_shader)
}

pub fn create_program(
    context: &WebGlRenderingContext,
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
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
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
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("create shader error"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    let check = context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
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
