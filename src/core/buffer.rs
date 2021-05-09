use std::os::raw::c_void;

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
            gl::BindBuffer(target, 0);
        }
        Buffer {
            id: buf,
            target: target,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.target, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.target, 0);
        }
    }

    pub fn vertex_input_attrib(
        &self,
        index: GLuint,
        size: GLint,
        stride: usize,
        offset: usize,
    ) {
        unsafe {
            gl::BindBuffer(self.target, self.id);
            gl::EnableVertexAttribArray(index);
            gl::VertexAttribPointer(
                index,
                size,
                gl::FLOAT,
                gl::FALSE,
                (stride * std::mem::size_of::<GLfloat>()) as GLsizei,
                (offset * std::mem::size_of::<GLfloat>()) as *const c_void,
            );
            gl::BindBuffer(self.target, 0);
        }
    }
}
