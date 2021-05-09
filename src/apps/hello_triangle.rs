use cgmath::perspective;
use cgmath::prelude::SquareMatrix;

use crate::core::app::App;
use crate::core::buffer::Buffer;
use crate::core::framework::Context;
use crate::core::shader::Shader;
use crate::core::vertex::VertexArray;

#[allow(dead_code)]
type Point3 = cgmath::Point3<f32>;
#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

pub struct HelloTriangleApp {
    vao: VertexArray,
    shader: Shader,
    mvp: Matrix4,
}

#[allow(dead_code)]
impl App for HelloTriangleApp {
    fn new(ctx: &Context) -> HelloTriangleApp {
        ctx.set_viewport();
        ctx.set_clear_color(1.0, 1.0, 1.0, 1.0);

        let shader = Shader::new("res/glsl/basic.vs.glsl", "res/glsl/basic.fs.glsl").unwrap();

        let vertices = [
            -0.5f32, -0.5f32, 0.0f32, 0.5f32, -0.5f32, 0.0f32, 0.0f32, 0.5f32, 0.0f32,
        ];
        let vao = VertexArray::new();
        let vbo = Buffer::new(gl::ARRAY_BUFFER, &vertices, gl::STATIC_DRAW);
        vao.binding(|| unsafe {
            vbo.vertex_input_attrib(0, 3, 0, 0);
        });

        let model = Matrix4::identity();
        let view = Matrix4::look_at_rh(
            Point3 {
                x: 0.0,
                y: 0.0,
                z: -2.5,
            },
            Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        );
        let proj = perspective(cgmath::Deg(60.0f32), ctx.aspect(), 0.1, 100.0);

        HelloTriangleApp {
            vao: vao,
            shader: shader,
            mvp: proj * view * model,
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
