use nalgebra_glm::TMat4;
use wasm_bindgen::prelude::*;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext};

use crate::buffer;
use crate::log;
use crate::mesh;
use crate::shader;

pub struct Scene<'a> {
    context: &'a WebGlRenderingContext,

    perspective: TMat4<f32>,
    mv: TMat4<f32>,

    program: WebGlProgram,
    vertex_buffer: Option<WebGlBuffer>,
    index_buffer: Option<WebGlBuffer>,

    position_loc: i32,
    normal_loc: i32,
}

impl<'a> Scene<'a> {
    pub fn new_with_context(
        width: i32,
        height: i32,
        context: &'a WebGlRenderingContext,
    ) -> Result<Self, JsValue> {
        let vert_shader = shader::vertex_shader(context)?;
        let frag_shader = shader::fragment_shader(context)?;
        let program = shader::create_program(context, &vert_shader, &frag_shader)?;

        let position_loc = context.get_attrib_location(&program, "aVertexPosition");
        let normal_loc = context.get_attrib_location(&program, "aVertexNormal");

        let fovy = 1.0472;
        let aspect = (width / height) as f32;
        let near = 0.1;
        let far = 100.0;

        Ok(Scene {
            context,
            program,
            position_loc,
            normal_loc,

            vertex_buffer: buffer::vertex_buffer(context).ok(),
            index_buffer: buffer::index_buffer(context).ok(),

            perspective: nalgebra_glm::perspective(aspect, fovy, near, far),
            mv: nalgebra_glm::identity(),
        })
    }

    pub fn render(&mut self) -> Result<(), JsValue> {
        self.translate(0.0, 0.0, -10.0);

        let program = self.program.as_ref();

        self.context.use_program(Some(program));
        self.context
            .enable_vertex_attrib_array(self.position_loc as u32);

        let vertex_buffer = self.vertex_buffer.as_ref();
        self.context
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, vertex_buffer);
        self.context.vertex_attrib_pointer_with_i32(
            self.position_loc as u32,
            3,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );

        let p = self
            .context
            .get_uniform_location(program, "uPMatrix")
            .ok_or("failed location")?;
        self.context
            .uniform_matrix4fv_with_f32_array(Some(&p), false, self.perspective.as_slice());

        let mv = self
            .context
            .get_uniform_location(program, "uMVMatrix")
            .ok_or("failed location")?;
        self.context
            .uniform_matrix4fv_with_f32_array(Some(&mv), false, self.mv.as_slice());

        let index_buffer = self.index_buffer.as_ref();
        self.context
            .bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, index_buffer);
        self.context.draw_elements_with_i32(
            WebGlRenderingContext::TRIANGLES,
            mesh::INDEX.len() as i32,
            WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );

        self.context
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);

        Ok(())
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        let m = self.mv;
        let v = nalgebra_glm::vec3(x, y, z);
        let m = nalgebra_glm::translate(&m, &v);

        let _ = std::mem::replace(&mut self.mv, m);
    }
}
