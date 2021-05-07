use std::path::Path;

use image::GenericImageView;

pub struct Texture {
    id: u32,
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
        println!("Delete Texture: {}", self.id);
    }
}

impl Texture {
    fn new(path: &Path) -> Result<Texture, String> {
        if path.exists() {
            return format!(
                "failed to load {}",
                path.to_str().expect("path must be valid utf-8 string.")
            );
        }

        // Load Image
        let img = image::open(path).expect("failed to load image");
        let data = img.into_bytes();
        let w = img.width() as i32;
        let h = img.height() as i32;

        // Generate Texture
        let mut tex = 0;
        unsafe {
            gl::GenTextures(1, &mut tex);
            gl::BindTexture(gl::TEXTURE_2D, tex);

            gl::TexStorage2D(gl::TEXTURE_2D, 1, gl::RGBA8, w, h);
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                0,
                0,
                w,
                h,
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
