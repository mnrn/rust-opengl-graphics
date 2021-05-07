use c_str_macro::c_str;
use cgmath::perspective;
use cgmath::prelude::SquareMatrix;

use crate::core::app::App;
use crate::core::buffer::Buffer;
use crate::core::common;
use crate::core::shader::Shader;
use crate::core::vertex::VertexArray;

#[allow(dead_code)]
type Point3 = cgmath::Point3<f32>;
#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

pub struct TriangleApp {
    vao: VertexArray,
    _vbo: Buffer,
    shader: Shader,
    mvp: Matrix4,
}

#[allow(dead_code)]
impl TriangleApp {
    pub fn new() -> TriangleApp {
        let shader = Shader::new("res/glsl/basic.vs.glsl", "res/glsl/basic.fs.glsl").unwrap();

        let vertices = [
            -0.5f32, -0.5f32, 0.0f32, 0.5f32, -0.5f32, 0.0f32, 0.0f32, 0.5f32, 0.0f32,
        ];
        let vao = VertexArray::new();
        let vbo = Buffer::new(gl::ARRAY_BUFFER, &vertices, gl::STATIC_DRAW);
        vao.init(|| unsafe {
            vbo.vertex_attrib_pointer(0, 3, gl::FLOAT, 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
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
        let proj = perspective(
            cgmath::Deg(60.0f32),
            common::WINDOW_WIDTH as f32 / common::WINDOW_HEIGHT as f32,
            0.1,
            100.0,
        );

        TriangleApp {
            vao: vao,
            _vbo: vbo,
            shader: shader,
            mvp: proj * view * model,
        }
    }
}

impl App for TriangleApp {
    fn render(&self) -> Result<(), String> {
        unsafe {
            gl::Viewport(
                0,
                0,
                common::WINDOW_WIDTH as i32,
                common::WINDOW_HEIGHT as i32,
            );

            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.shader.use_program();
            self.shader.set_mat4(c_str!("MVP"), &self.mvp);
            self.vao.draw_arrays(gl::TRIANGLES, 0, 3);
        }
        Ok(())
    }
}
