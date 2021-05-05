use gl;
use std;
use std::default::Default;
use std::marker::PhantomData;
use std::vec::Vec;

pub struct Mesh {
    vao: u32,
    vbo: Vec<u32>,
    verts_num: i32,
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(self.vbo.len() as i32, self.vbo.as_ptr());
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}

#[allow(dead_code)]
impl Mesh {
    pub fn draw_arrays(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.verts_num);
            gl::BindVertexArray(0);
        }
    }

    pub fn draw_elements(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.verts_num,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
            gl::BindVertexArray(0);
        }
    }
}

#[allow(dead_code)]
pub struct Empty;
#[allow(dead_code)]
pub struct Fully;

pub struct MeshBuilder<Indices, Positions> {
    indices: Vec<f32>,
    pos: Vec<f32>,
    norm: Option<Vec<f32>>,
    col: Option<Vec<f32>>,
    uv: Option<Vec<f32>>,
    tan: Option<Vec<f32>>,
    state: (PhantomData<Indices>, PhantomData<Positions>),
}

impl MeshBuilder<Empty, Empty> {
    pub fn new() -> Self {
        MeshBuilder {
            indices: Default::default(),
            pos: Default::default(),
            norm: Default::default(),
            col: Default::default(),
            uv: Default::default(),
            tan: Default::default(),
            state: (PhantomData, PhantomData),
        }
    }
}

impl MeshBuilder<Fully, Fully> {
    pub fn build(self) -> Mesh {
        let mut vao = 0;
        let mut vbo = Vec::new();

        unsafe {
            let mut index = 0;
            gl::GenBuffers(1, &mut index);
            vbo.push(index);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.indices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                std::mem::transmute(self.indices.as_ptr()),
                gl::STATIC_DRAW,
            );

            let mut pos = 0;
            gl::GenBuffers(1, &mut pos);
            vbo.push(pos);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, pos);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.pos.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                std::mem::transmute(self.pos.as_ptr()),
                gl::STATIC_DRAW,
            );

            // TODO: norm, col, uv, tan, ...

            // Vertex Array Object
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // Bind indices
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index);

            // Bind positions
            gl::BindBuffer(gl::ARRAY_BUFFER, pos);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            // TODO: Bind norm, col, uv, tan, ...

            gl::BindVertexArray(0);
        }

        Mesh {
            vao: vao,
            vbo: vbo,
            verts_num: self.indices.len() as i32,
        }
    }
}

impl<Positions> MeshBuilder<Empty, Positions> {
    pub fn indices(mut self, indices: &[f32]) -> MeshBuilder<Fully, Positions> {
        self.indices.copy_from_slice(indices);
        MeshBuilder {
            indices: self.indices,
            pos: self.pos,
            norm: self.norm,
            col: self.col,
            uv: self.uv,
            tan: self.tan,
            state: (PhantomData, self.state.1),
        }
    }
}

impl<Indices> MeshBuilder<Indices, Empty> {
    pub fn positions(mut self, pos: &[f32]) -> MeshBuilder<Indices, Empty> {
        self.pos.copy_from_slice(pos);
        MeshBuilder {
            indices: self.indices,
            pos: self.pos,
            norm: self.norm,
            col: self.col,
            uv: self.uv,
            tan: self.tan,
            state: (self.state.0, PhantomData),
        }
    }
}
