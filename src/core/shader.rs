use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::str;

use gl::types::*;
use nalgebra::{Matrix4, Vector3};

pub struct Shader {
    id: u32,
}

impl Drop for Shader {
    // Deletes the Shader program.
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
        println!("Delete Shader Program: {}", self.id);
    }
}

#[allow(dead_code)]
impl Shader {
    pub fn new(vert_path: &str, frag_path: &str) -> Result<Shader, String> {
        // Vertex
        let mut vert_file =
            File::open(vert_path).unwrap_or_else(|_| panic!("failed to open file: {}", vert_path));
        let mut vert_code = String::new();
        vert_file
            .read_to_string(&mut vert_code)
            .expect("failed to read vertex shader file");

        // Fragment
        let mut frag_file =
            File::open(frag_path).unwrap_or_else(|_| panic!("failed to open file: {}", frag_path));
        let mut frag_code = String::new();
        frag_file
            .read_to_string(&mut frag_code)
            .expect("failed to read fragment shader file");

        let successful: bool;
        let mut shader = Shader { id: 0 };
        unsafe {
            // Compile
            let vert = shader.compile(vert_code.as_str(), gl::VERTEX_SHADER)?;
            let frag = shader.compile(frag_code.as_str(), gl::FRAGMENT_SHADER)?;

            // Link
            let id = gl::CreateProgram();
            gl::AttachShader(id, vert);
            gl::AttachShader(id, frag);
            gl::LinkProgram(id);
            successful = {
                let mut res: GLint = 0;
                gl::GetProgramiv(id, gl::LINK_STATUS, &mut res);
                res != 0
            };

            gl::DeleteShader(vert);
            gl::DeleteShader(frag);

            shader.id = id;
        }

        if successful {
            Ok(shader)
        } else {
            Err(shader.program_log())
        }
    }

    fn compile(&self, source: &str, shader_type: GLuint) -> Result<u32, String> {
        let id = unsafe { gl::CreateShader(shader_type) };
        unsafe {
            let ptr: *const u8 = source.as_bytes().as_ptr();
            let ptr_i8: *const i8 = std::mem::transmute(ptr);
            let len = source.len() as GLint;
            gl::ShaderSource(id, 1, &ptr_i8, &len);
            gl::CompileShader(id);
        }

        let successful = unsafe {
            let mut res: GLint = 0;
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut res);
            res != 0
        };
        if successful {
            Ok(id)
        } else {
            Err(self.shader_log())
        }
    }

    fn shader_log(&self) -> String {
        let mut len = 0;
        unsafe {
            gl::GetShaderiv(self.id, gl::INFO_LOG_LENGTH, &mut len);
        }
        assert!(len > 0);

        let mut buf = Vec::with_capacity(len as usize);
        let buf_ptr = buf.as_mut_ptr() as *mut gl::types::GLchar;
        unsafe {
            gl::GetShaderInfoLog(self.id, len, std::ptr::null_mut(), buf_ptr);
            buf.set_len(len as usize);
        };

        match String::from_utf8(buf) {
            Ok(log) => log,
            Err(vec) => panic!("Could not convert log from buffer: {}", vec),
        }
    }

    fn program_log(&self) -> String {
        let mut len = 0;
        unsafe {
            gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut len);
        }
        assert!(len > 0);

        let mut buf = Vec::with_capacity(len as usize);
        let buf_ptr = buf.as_mut_ptr() as *mut gl::types::GLchar;
        unsafe {
            gl::GetProgramInfoLog(self.id, len, std::ptr::null_mut(), buf_ptr);
            buf.set_len(len as usize);
        };

        match String::from_utf8(buf) {
            Ok(log) => log,
            Err(vec) => panic!("Could not convert log from buffer: {}", vec),
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    fn get_uniform_location_option(&self, name: &str) -> Option<Uniform> {
        let c_name = CString::new(name).unwrap();
        match unsafe { gl::GetUniformLocation(self.id, c_name.as_ptr()) } {
            -1 => None,
            loc => Some(Uniform { id: loc }),
        }
    }

    fn get_uniform_location(&self, name: &str) -> Uniform {
        match self.get_uniform_location_option(name) {
            Some(uni) => uni,
            None => panic!("Could not find uniform \"{}\"", name),
        }
    }

    pub fn set_vec3(&self, name: &str, vec3: &Vector3<f32>) {
        let uni = self.get_uniform_location(name);
        unsafe {
            gl::Uniform3fv(uni.id, 1, vec3.as_ptr());
        }
    }

    pub fn set_mat4(&self, name: &str, mat4: &Matrix4<f32>) {
        let uni = self.get_uniform_location(name);
        unsafe {
            gl::UniformMatrix4fv(uni.id, 1, gl::FALSE, mat4.as_ptr());
        }
    }

    pub fn set_int(&self, name: &str, i: i32) {
        let uni = self.get_uniform_location(name);
        unsafe {
            gl::Uniform1i(uni.id, i);
        }
    }

    pub fn set_texture(&self, name: &str, tex: i32) {
        self.set_int(name, tex);
    }
}

struct Uniform {
    pub id: GLint,
}
