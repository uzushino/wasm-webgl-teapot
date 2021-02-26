use js_sys::JsString;
use nalgebra_glm::TMat4;
use wasm_bindgen::prelude::*;
use web_sys::{WebGlBuffer, WebGlProgram, WebGl2RenderingContext, WebGlUniformLocation, WebGlTexture};

use crate::buffer::{self, vertex_buffer};
use crate::log;
use crate::mesh;
use crate::shader;

pub struct Scene<'a> {
    context: &'a WebGl2RenderingContext,

    perspective: TMat4<f32>,
    mv: TMat4<f32>,

    program: WebGlProgram,
    
    //vertex_buffer: Option<WebGlBuffer>,
    //index_buffer: Option<WebGlBuffer>,

    position_loc: i32,
    normal_loc: i32,
    color_loc: i32,
}

impl<'a> Scene<'a> {
    pub fn new_with_context(
        width: i32,
        height: i32,
        context: &'a WebGl2RenderingContext,
    ) -> Result<Self, JsValue> {
        let vert_shader = shader::vertex_shader(context)?;
        let frag_shader = shader::fragment_shader(context)?;
        let program = shader::create_program(context, &vert_shader, &frag_shader)?;

        let position_loc = context.get_attrib_location(&program, "aVertexPosition");
        let normal_loc = context.get_attrib_location(&program, "aVertexNormal");
        let color_loc = context.get_attrib_location(&program, "aColor");

        // カメラ
        let fovy = 1.0472;
        let aspect = (width / height) as f32;
        let near = 0.1;
        let far = 100.0;

        Ok(Scene {
            context,
            program,
            position_loc,
            normal_loc,
            color_loc,

            // vertex_buffer: buffer::vertex_buffer(context).ok(),
            // index_buffer: buffer::index_buffer(context).ok(),

            perspective: nalgebra_glm::perspective(aspect, fovy, near, far),
            mv: nalgebra_glm::identity(),
        })
    }

    pub fn render(&mut self) -> Result<(), JsValue> {
        // 視点座標
        self.translate(0.0, 0.0, -10.0);

        let program = self.program.as_ref();

        self.context.use_program(Some(program));

        // teapot
        let teapot_vb = buffer::vertex_buffer(self.context, mesh::VERTEX).ok();
        let teapot_vb = teapot_vb.as_ref();
        self.context
            .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, teapot_vb);
        
        self.context
            .enable_vertex_attrib_array(self.position_loc as u32);

        self.context.vertex_attrib_pointer_with_i32(
            self.position_loc as u32,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );

        let teapot_ib = buffer::index_buffer(self.context, mesh::INDEX).ok();
        let teapot_ib = teapot_ib.as_ref();
        self.context
            .bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, teapot_ib);
        self.context.draw_elements_with_i32(
            WebGl2RenderingContext::TRIANGLES,
            mesh::INDEX.len() as i32,
            WebGl2RenderingContext::UNSIGNED_SHORT,
            0,
        );

        self.context
            .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);

        Ok(())
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        let m = self.mv;
        let v = nalgebra_glm::vec3(x, y, z);
        let m = nalgebra_glm::translate(&m, &v);

        let _ = std::mem::replace(&mut self.mv, m);
    }

    pub fn uniform_locations(&mut self, program: &WebGlProgram) -> Result<Vec<WebGlUniformLocation>, JsValue> {
        let v1 = self
            .context
            .get_uniform_location(program, "mMatrix")
            .ok_or("failed location")?;
        let v2 = self
            .context
            .get_uniform_location(program, "mvpMatrix")
            .ok_or("failed location")?;
        let v3 = self
            .context
            .get_uniform_location(program, "eyePosition")
            .ok_or("failed location")?;
        let v4 = self
            .context
            .get_uniform_location(program, "cubeTexture")
            .ok_or("failed location")?;
        let v5 = self
            .context
            .get_uniform_location(program, "reflection")
            .ok_or("failed location")?;
        Ok(vec![v1, v2, v3, v4, v5])
    }

    pub fn create_texture(&mut self) -> Result<WebGlTexture, JsValue> {
        let sources = [
            std::include_bytes!("cube_PX.png"),
            /*
            std::include_bytes!("cube_PY.png"),
            std::include_bytes!("cube_PZ.png"),
            std::include_bytes!("cube_NX.png"),
            std::include_bytes!("cube_NY.png"),
            std::include_bytes!("cube_NZ.png"),
            */
        ];

        let targets = [
            WebGl2RenderingContext::TEXTURE_CUBE_MAP_POSITIVE_X,
            WebGl2RenderingContext::TEXTURE_CUBE_MAP_POSITIVE_Y,
            WebGl2RenderingContext::TEXTURE_CUBE_MAP_POSITIVE_Z,
            WebGl2RenderingContext::TEXTURE_CUBE_MAP_NEGATIVE_X,
            WebGl2RenderingContext::TEXTURE_CUBE_MAP_NEGATIVE_Y,
            WebGl2RenderingContext::TEXTURE_CUBE_MAP_NEGATIVE_Z
        ];

        let tex = self
            .context
            .create_texture()
            .ok_or("failed create texture")?;

        self
            .context
            .bind_texture(WebGl2RenderingContext::TEXTURE_CUBE_MAP, Some(&tex));
       
        for (i, target) in targets.iter().enumerate() {
            let pic = sources[i];
            let img = image::load_from_memory_with_format(
                pic,
                image::ImageFormat::Png
            ).map_err(|e| e.to_string())?;

            self
                .context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                    *target, 
                    0, 
                    WebGl2RenderingContext::RGBA as i32, 
                    186,
                    213,
                    0,
                    WebGl2RenderingContext::RGBA,
                    WebGl2RenderingContext::UNSIGNED_BYTE,
                    Some(&img.into_rgba8().into_vec()))?;
        }

        self
            .context
            .generate_mipmap(WebGl2RenderingContext::TEXTURE_CUBE_MAP);

        self
            .context
            .tex_parameteri(
                WebGl2RenderingContext::TEXTURE_CUBE_MAP,
                WebGl2RenderingContext::TEXTURE_MIN_FILTER,
                WebGl2RenderingContext::LINEAR as i32,
            );
        self
            .context
            .tex_parameteri(
                WebGl2RenderingContext::TEXTURE_CUBE_MAP,
                WebGl2RenderingContext::TEXTURE_MAG_FILTER,
                WebGl2RenderingContext::LINEAR as i32,
            );
        self
            .context
            .tex_parameteri(
                WebGl2RenderingContext::TEXTURE_CUBE_MAP,
                WebGl2RenderingContext::TEXTURE_WRAP_S,
                WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
            );
        self
            .context
            .tex_parameteri(
                WebGl2RenderingContext::TEXTURE_CUBE_MAP,
                WebGl2RenderingContext::TEXTURE_WRAP_T,
                WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
            );

        self
            .context
            .bind_texture(WebGl2RenderingContext::TEXTURE_CUBE_MAP, None);
       
        Ok(tex)
    }
}
