use wasm_bindgen::prelude::*;
use web_sys::{WebGlBuffer, WebGlRenderingContext, WebGlTexture, WebGlUniformLocation };

use crate::buffer;
use crate::teapot;
use crate::cube;
use crate::shader;

pub struct Scene<'a> {
    context: &'a WebGlRenderingContext,
    width: i32,
    height: i32,
    
    teapot_vertex: Option<WebGlBuffer>,
    teapot_index: Option<WebGlBuffer>,
    teapot_normal: Option<WebGlBuffer>,
    cube_vertex: Option<WebGlBuffer>,
    cube_index: Option<WebGlBuffer>,
    cube_normal: Option<WebGlBuffer>,

    position: i32,
    normal: i32,
    color: i32,

    m: Option<WebGlUniformLocation>,
    mvp: Option<WebGlUniformLocation>,
    eye: Option<WebGlUniformLocation>,
    cube: Option<WebGlUniformLocation>,

    cube_texture: Option<WebGlTexture>,
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
        context.use_program(Some(&program));

        let position = context.get_attrib_location(&program, "aPosition");
        let normal = context.get_attrib_location(&program, "aNormal");
        let color = context.get_attrib_location(&program, "aColor");
        
        let m = context.get_uniform_location(&program, "uModelMatrix"); // Model行列
        let mvp = context.get_uniform_location(&program, "uMVPMatrix"); // ModelViewProjection行列
        let eye = context.get_uniform_location(&program, "eyePosition");
        let cube = context.get_uniform_location(&program, "cubeTexture");

        let cube_texture = Self::create_texture(context).ok();

        // カメラ
        Ok(Scene {
            width,
            height,

            context,

            position,
            normal,
            color,

            teapot_vertex: buffer::vertex_buffer(context, teapot::VERTEX).ok(),
            teapot_index: buffer::index_buffer(context, teapot::INDEX).ok(),
            teapot_normal: buffer::vertex_buffer(context, teapot::NORMAL).ok(),

            cube_vertex: buffer::vertex_buffer(context, cube::VERTEX).ok(),
            cube_index: buffer::index_buffer(context, cube::INDEX).ok(),
            cube_normal: buffer::vertex_buffer(context, cube::NORMAL).ok(),
          
            m,
            mvp,
            eye,
            cube,

            cube_texture: cube_texture,
        })
    }

    pub fn render(&mut self) -> Result<(), JsValue> {
        // 視点座標
        let projection_matrix = 
            nalgebra_glm::perspective((self.width / self.height) as f32, 1.0472, 0.1, 200.0); 
        
        let center = nalgebra_glm::vec3(0.0, 0.0, 0.0);
        let up = nalgebra_glm::vec3(0.0, 1.0, 0.0);
        let eye = nalgebra_glm::vec3(0.0, 0.0, -10.0);
        let view_matrix = nalgebra_glm::look_at(&eye, &center, &up);
        let pv = projection_matrix * view_matrix;

        let mut color = Vec::default();
        for _ in 0..(cube::VERTEX.len() / 3) {
            color.push(1.0);
            color.push(1.0);
            color.push(1.0);
            color.push(1.0);
        }

        // cube
        buffer::render_buffer(
            self.context, 
            self.cube_vertex.as_ref(), 
            self.position, 
            3
        )?;
        buffer::render_buffer(
            self.context, 
            self.cube_normal.as_ref(), 
            self.normal, 
            3
        )?;
        buffer::render_buffer(
            self.context, 
            buffer::vertex_buffer(self.context, color.as_slice()).ok().as_ref(), 
            self.color, 
            4
        )?;
        self.context
            .bind_buffer(
                WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, 
                self.cube_index.as_ref()
            );

        let scale = 
            nalgebra_glm::scale(&nalgebra_glm::identity(), &nalgebra_glm::vec3(100.0, 100.0, 100.0));
        self.context
            .uniform_matrix4fv_with_f32_array(self.m.as_ref(), false, scale.as_slice());
        self.context
            .uniform_matrix4fv_with_f32_array(self.mvp.as_ref(), false, (pv * scale).as_slice());
        self.context
            .uniform3fv_with_f32_array(self.eye.as_ref(), eye.as_slice());

        self.context
            .active_texture(WebGlRenderingContext::TEXTURE0);
        self.context
            .bind_texture(WebGlRenderingContext::TEXTURE_CUBE_MAP, self.cube_texture.as_ref());
        self.context
            .uniform1i(self.cube.as_ref(), 0);
        /*
        self.context
            .uniform1i(self.reflection.as_ref(), 0);
        */
        self.context.draw_elements_with_i32(
            WebGlRenderingContext::TRIANGLES,
            cube::INDEX.len() as i32,
            WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );

        // teapot
        let mut color = Vec::default();
        for _ in 0..(teapot::VERTEX.len() / 3) {
            color.push(1.0);
            color.push(1.0);
            color.push(1.0);
            color.push(1.0);
        }
        buffer::render_buffer(
            self.context, 
            self.teapot_vertex.as_ref(), 
            self.position, 
            3
        )?;
        buffer::render_buffer(
            self.context, 
            self.teapot_normal.as_ref(), 
            self.normal, 
            3
        )?;
        buffer::render_buffer(
            self.context, 
            buffer::vertex_buffer(self.context, color.as_slice()).ok().as_ref(), 
            self.color, 
            4
        )?;
        self.context
            .bind_buffer(
                WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, 
                self.teapot_index.as_ref()
            );

        let translate = 
            nalgebra_glm::translate(&nalgebra_glm::identity(), &nalgebra_glm::vec3(0.0, 0.0, 50.0));
        let rotate = 
            nalgebra_glm::rotate(&translate, 0.785398, &nalgebra_glm::vec3(-50.0, 0.0, 50.0));
        self.context
            .uniform_matrix4fv_with_f32_array(self.m.as_ref(), false, rotate.as_slice());
        self.context
            .uniform_matrix4fv_with_f32_array(self.mvp.as_ref(), false, (pv * rotate).as_slice());
        /*
        self.context
            .uniform1i(self.reflection.as_ref(), 1);
        */
        self.context.draw_elements_with_i32(
            WebGlRenderingContext::TRIANGLES,
            teapot::INDEX.len() as i32,
            WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );

        self.context.flush();

        self.context
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);

        Ok(())
    }

    pub fn create_texture(context: &'a WebGlRenderingContext) -> Result<WebGlTexture, JsValue> {
        let source = std::include_bytes!("check.png");

        let targets = [
            WebGlRenderingContext::TEXTURE_CUBE_MAP_POSITIVE_X,
            WebGlRenderingContext::TEXTURE_CUBE_MAP_POSITIVE_Y,
            WebGlRenderingContext::TEXTURE_CUBE_MAP_POSITIVE_Z,
            WebGlRenderingContext::TEXTURE_CUBE_MAP_NEGATIVE_X,
            WebGlRenderingContext::TEXTURE_CUBE_MAP_NEGATIVE_Y,
            WebGlRenderingContext::TEXTURE_CUBE_MAP_NEGATIVE_Z
        ];

        let tex = context
            .create_texture()
            .ok_or("failed create texture")?;

        context
            .bind_texture(WebGlRenderingContext::TEXTURE_CUBE_MAP, Some(&tex));
       
        for target in targets.iter() {
            let img = image::load_from_memory_with_format(
                source,
                image::ImageFormat::Png
            ).map_err(|e| e.to_string())?;
            
            context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                    *target, 
                    0, 
                    WebGlRenderingContext::RGBA as i32, 
                    256,
                    256,
                    0,
                    WebGlRenderingContext::RGBA,
                    WebGlRenderingContext::UNSIGNED_BYTE,
                    Some(&img.into_rgba8().into_vec())
                )?;
        }

        context
            .generate_mipmap(WebGlRenderingContext::TEXTURE_CUBE_MAP);

        context
            .tex_parameteri(
                WebGlRenderingContext::TEXTURE_CUBE_MAP,
                WebGlRenderingContext::TEXTURE_MIN_FILTER,
                WebGlRenderingContext::LINEAR as i32,
            );
        
        context
            .tex_parameteri(
                WebGlRenderingContext::TEXTURE_CUBE_MAP,
                WebGlRenderingContext::TEXTURE_MAG_FILTER,
                WebGlRenderingContext::LINEAR as i32,
            );
        
        context
            .tex_parameteri(
                WebGlRenderingContext::TEXTURE_CUBE_MAP,
                WebGlRenderingContext::TEXTURE_WRAP_S,
                WebGlRenderingContext::CLAMP_TO_EDGE as i32,
            );
        
        context
            .tex_parameteri(
                WebGlRenderingContext::TEXTURE_CUBE_MAP,
                WebGlRenderingContext::TEXTURE_WRAP_T,
                WebGlRenderingContext::CLAMP_TO_EDGE as i32,
            );

        context
            .bind_texture(WebGlRenderingContext::TEXTURE_CUBE_MAP, None);
       
        Ok(tex)
    }
}
