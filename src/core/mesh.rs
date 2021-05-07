use std::default::Default;
use std::marker::PhantomData;
use std::vec::Vec;

use super::buffer::Buffer;
use super::vertex::VertexArray;

#[allow(dead_code)]
pub struct Mesh {
    vao: VertexArray,
    vbo: Vec<Buffer>,
    verts_num: i32,
}

#[allow(dead_code)]
impl Mesh {
    pub unsafe fn draw_arrays(&self) {
        self.vao.draw_arrays(gl::TRIANGLES, 0, self.verts_num);
    }

    pub unsafe fn draw_elements(&self) {
        self.vao
            .draw_elements(gl::TRIANGLES, self.verts_num, gl::UNSIGNED_INT, 0);
    }
}

#[allow(dead_code)]
pub struct Empty;
#[allow(dead_code)]
pub struct Fully;

#[allow(dead_code)]
pub struct MeshBuilder<Indices, Positions> {
    indices: Vec<i32>,
    pos: Vec<f32>,
    norm: Option<Vec<f32>>,
    col: Option<Vec<f32>>,
    uv: Option<Vec<f32>>,
    tan: Option<Vec<f32>>,
    state: (PhantomData<Indices>, PhantomData<Positions>),
}

#[allow(dead_code)]
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

#[allow(dead_code)]
impl MeshBuilder<Fully, Fully> {
    pub fn build(self) -> Mesh {
        let vao = VertexArray::new();
        let mut vbo = Vec::new();

        unsafe {
            let index = Buffer::new(gl::ELEMENT_ARRAY_BUFFER, &self.indices, gl::STATIC_DRAW);
            vbo.push(index);

            let pos = Buffer::new(gl::ARRAY_BUFFER, &self.pos, gl::STATIC_DRAW);

            // TODO: norm, col, uv, tan, ...

            // Relationship with VAO ans VBO;
            vao.init(|| {
                // Bind positions
                pos.vertex_attrib_pointer(0, 3, gl::FLOAT, 0);
                vbo.push(pos);

                // TODO: Bind norm, col, uv, tan, ...

                // Unbind VBO
                gl::BindBuffer(gl::ARRAY_BUFFER, 0);

                // REMEMBER: do NOT unbind the IBO while a VAO is active as the bound index buffer object IS stored in the VAO; keep the IBO bound.
                //gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            });
        }

        Mesh {
            vao: vao,
            vbo: vbo,
            verts_num: self.indices.len() as i32,
        }
    }
}

#[allow(dead_code)]
impl<Positions> MeshBuilder<Empty, Positions> {
    pub fn indices(mut self, indices: &[i32]) -> MeshBuilder<Fully, Positions> {
        self.indices = Vec::new();
        self.indices.extend(indices.iter().cloned());
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

#[allow(dead_code)]
impl<Indices> MeshBuilder<Indices, Empty> {
    pub fn positions(mut self, pos: &[f32]) -> MeshBuilder<Indices, Fully> {
        self.pos = Vec::new();
        self.pos.extend(pos.iter().cloned());
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
