use gl;

pub struct VertexArray {
    pub id: u32,
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}

#[allow(dead_code)]
impl VertexArray {
    pub fn new() -> Self {
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        VertexArray {
            id: vao
        }
    }
}
