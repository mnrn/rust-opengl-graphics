use crate::core::app::App;
use crate::core::buffer::Buffer;
use crate::core::framework::Context;
use crate::core::shader::Shader;
use crate::core::texture::Texture2D;
use crate::core::texture::TextureBuilder;
use crate::core::vertex::VertexArray;

#[allow(dead_code)]
type Point3 = cgmath::Point3<f32>;
#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

pub struct TextureMappingApp {
    shader: Shader,
    vao: VertexArray,
    tex: Texture2D,
}

#[allow(dead_code)]
impl App for TextureMappingApp {
    fn new(ctx: &Context) -> TextureMappingApp {
        ctx.set_viewport();
        ctx.set_clear_color(1.0, 1.0, 1.0, 1.0);

        let shader = Shader::new("res/glsl/texture.vs.glsl", "res/glsl/texture.fs.glsl").unwrap();

        let vertices: [f32; 20] = [
            // positions      // texture coords
            0.5, 0.5, 0.0, 1.0, 1.0, // top right
            0.5, -0.5, 0.0, 1.0, 0.0, // bottom right
            -0.5, -0.5, 0.0, 0.0, 0.0, // bottom left
            -0.5, 0.5, 0.0, 0.0, 1.0, // top left
        ];
        let indices: [u32; 6] = [
            0, 1, 3, // first triangle
            1, 2, 3, // second triangle
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
            .build2d("res/textures/seamless/vintage_water.png")
            .unwrap();

        TextureMappingApp {
            vao: vao,
            shader: shader,
            tex: tex,
        }
    }

    fn render(&self, ctx: &Context) -> Result<(), String> {
        ctx.clear_buffer(gl::COLOR_BUFFER_BIT);

        self.shader.use_program();
        self.tex.binding(|| {
            self.vao
                .draw_elements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0);
        });

        Ok(())
    }
}
