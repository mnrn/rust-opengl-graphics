use std::path::Path;

use image::GenericImageView;

pub struct Texture {
    id: u32,
}

impl Drop for Texture {
    // Delete the texture
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
        println!("Delete Texture: {}", self.id);
    }
}


impl Texture {
    fn new(path: &Path) -> Result<Texture, String> {
        // Check the texture path
        if path.exists() {
            return Err(format!(
                "failed to load {}",
                path.to_str().expect("path must be valid utf-8 string.")
            ));
        }

        // Load Image
        let img = image::open(path).expect("failed to load image");
        let (w, h) = img.dimensions();
        let data = img.into_bytes();

        // Generate Texture
        let mut tex = 0;
        unsafe {
            gl::GenTextures(1, &mut tex);
            gl::BindTexture(gl::TEXTURE_2D, tex);

            // Setup
            gl::TexStorage2D(gl::TEXTURE_2D, 1, gl::RGBA8, w as i32, h as i32);
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                0,
                0,
                w as i32,
                h as i32,
                gl::RGBA8,
                gl::UNSIGNED_BYTE,
                std::mem::transmute(data.as_ptr()),
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Ok(Texture { id: tex })
    }
}
