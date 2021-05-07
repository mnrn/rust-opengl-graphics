use gl::types::*;

pub struct Buffer {
    id: u32,
    target: GLenum,
}

impl Drop for Buffer {
    // Delete the buffer
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
        println!("Delete Buffer: {}", self.id);
    }
}

#[allow(dead_code)]
impl Buffer {
    pub fn new<T>(target: GLenum, data: &[T], usage: GLenum) -> Self {
        let mut buf = 0;
        unsafe {
            gl::GenBuffers(1, &mut buf);
            gl::BindBuffer(target, buf);
            gl::BufferData(
                target,
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                std::mem::transmute(data.as_ptr()),
                usage,
            );
        }
        Buffer {
            id: buf,
            target: target,
        }
    }

    pub unsafe fn vertex_attrib_pointer(
        &self,
        index: GLuint,
        size: GLint,
        _type: GLenum,
        stride: GLsizei,
    ) {
        gl::BindBuffer(self.target, self.id);
        gl::EnableVertexAttribArray(index);
        gl::VertexAttribPointer(index, size, _type, gl::FALSE, stride, std::ptr::null());
    }
}
