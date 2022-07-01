use nalgebra_glm as glm;

use crate::core::app::App;
use crate::core::buffer::Buffer;
use crate::core::framework::Context;
use crate::core::shader::Shader;
use crate::core::texture::Texture2D;
use crate::core::texture::TextureBuilder;
use crate::core::vertex::VertexArray;

pub struct TextureMappingApp {
    shader: Shader,
    vao: VertexArray,
    tex: Texture2D,
    mvp: glm::Mat4
}

#[allow(dead_code)]
impl App for TextureMappingApp {
    fn new(ctx: &Context) -> TextureMappingApp {
        ctx.set_viewport();
        ctx.set_clear_color(1.0, 1.0, 1.0, 1.0);

        let shader = Shader::new("res/glsl/texture.vs.glsl", "res/glsl/texture.fs.glsl").unwrap();

        let vertices: [f32; 20] = [
            -1.0,  1.0, 0.0, 0.0, 0.0,
             1.0,  1.0, 0.0, 1.0, 0.0,
            -1.0, -1.0, 0.0, 0.0, 1.0,
             1.0, -1.0, 0.0, 1.0, 1.0,
        ];
        let indices: [u32; 6] = [
            0, 1, 2, // first triangle
            3, 2, 1, // second triangle
        ];

        let vao = VertexArray::new();
        let vbo = Buffer::new(gl::ARRAY_BUFFER, &vertices, gl::STATIC_DRAW);
        let ibo = Buffer::new(gl::ELEMENT_ARRAY_BUFFER, &indices, gl::STATIC_DRAW);

        vao.binding(|| {
            vbo.vertex_input_attrib(0, 3, 5, 0);
            vbo.vertex_input_attrib(1, 2, 5, 3);
            ibo.bind();
        });

        let tex = TextureBuilder::new()
            .filter(gl::LINEAR)
            .wrap(gl::REPEAT)
            .build2d("res/textures/sailboat-on-lake.png")
            .unwrap();

        let model = glm::Mat4::identity();
        let eye = glm::vec3(0.0, 0.0, -2.5);
        let target = glm::vec3(0.0, 0.0, 0.0);
        let view = glm::look_at_rh(&eye, &target, &glm::Vec3::y());
        let fov = glm::radians(&glm::vec3(60.0, 60.0, 60.0));
        let proj = glm::perspective(ctx.aspect(), fov.y, 0.1, 100.0);

        TextureMappingApp {
            vao: vao,
            shader: shader,
            tex: tex,
            mvp: proj * view * model
        }
    }

    fn render(&self, ctx: &Context) -> Result<(), String> {
        ctx.clear_buffer(gl::COLOR_BUFFER_BIT);

        self.shader.use_program();
        self.shader.set_mat4("MVP", &self.mvp);
        self.tex.binding(|| {
            self.vao
                .draw_elements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0);
        });

        Ok(())
    }
}
