use image::{
    GenericImageView,
    DynamicImage,
};

pub struct Texture2D {
    id: u32,
}

impl Drop for Texture2D {
    // Delete the texture
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
        println!("Delete Texture: {}", self.id);
    }
}

#[allow(dead_code)]
impl Texture2D {
    pub fn new(path: &str) -> Result<Texture2D, String> {

        // Load Image
        let img: DynamicImage = image::open(path).expect("failed to load image");
        println!("Load Image: {}", path);
        let (w, h) = img.dimensions();
        let format = match img {
            DynamicImage::ImageLuma8(_) => gl::RED,
            DynamicImage::ImageLumaA8(_) => gl::RG,
            DynamicImage::ImageRgb8(_) => gl::RGB,
            DynamicImage::ImageRgba8(_) => gl::RGBA,
            DynamicImage::ImageBgr8(_) => gl::RGB,
            DynamicImage::ImageBgra8(_) => gl::RGBA,
            _ => unreachable!()
        };
        let data = img.into_bytes();

        // Generate Texture
        let mut tex = 0;
        unsafe {
            gl::GenTextures(1, &mut tex);
            gl::BindTexture(gl::TEXTURE_2D, tex);

            // Set the texture wrapping parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            // Set texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);

            // Generate mipmap
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as i32,
                w as i32,
                h as i32,
                0,
                format,
                gl::UNSIGNED_BYTE,
                std::mem::transmute(data.as_ptr())
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Ok(Texture2D { id: tex })
    }

    pub fn binding<F>(&self, cb: F) where F: FnOnce() {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id); 
        }

        cb();

        unsafe { 
            gl::BindTexture(gl::TEXTURE_2D, 0); 
        }
    }
}
