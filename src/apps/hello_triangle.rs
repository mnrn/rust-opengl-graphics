use nalgebra::{Matrix4, Perspective3, Point3, Vector3};

use crate::core::app::App;
use crate::core::buffer::Buffer;
use crate::core::framework::Context;
use crate::core::shader::Shader;
use crate::core::vertex::VertexArray;

pub struct HelloTriangleApp {
    vao: VertexArray,
    shader: Shader,
    mvp: Matrix4<f32>,
}

#[allow(dead_code)]
impl App for HelloTriangleApp {
    fn new(ctx: &Context) -> HelloTriangleApp {
        ctx.set_viewport();
        ctx.set_clear_color(1.0, 1.0, 1.0, 1.0);

        let shader = Shader::new("res/glsl/basic.vs.glsl", "res/glsl/basic.fs.glsl").unwrap();

        let vertices = [
            -1.0f32, -1.0f32, 0.0f32, 1.0f32, -1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32,
        ];
        let vao = VertexArray::new();
        let vbo = Buffer::new(gl::ARRAY_BUFFER, &vertices, gl::STATIC_DRAW);
        vao.binding(|| {
            vbo.vertex_input_attrib(0, 3, 0, 0);
        });

        let model = Matrix4::identity();
        let eye = Point3::new(0.0f32, 0.0, -2.5);
        let target = Point3::new(0.0, 0.0, 0.0);
        let view = Matrix4::look_at_rh(&eye, &target, &Vector3::y());
        let proj = Perspective3::new(ctx.aspect(), 2.1, 0.1, 100.0);

        HelloTriangleApp {
            vao: vao,
            shader: shader,
            mvp: proj.as_matrix() * view * model,
        }
    }

    fn render(&self, ctx: &Context) -> Result<(), String> {
        ctx.clear_buffer(gl::COLOR_BUFFER_BIT);

        self.shader.use_program();
        self.shader.set_mat4("MVP", &self.mvp);
        self.vao.draw_arrays(gl::TRIANGLES, 0, 3);

        Ok(())
    }
}
