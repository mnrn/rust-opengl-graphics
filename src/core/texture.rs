use std::marker::PhantomData;
use std::path::Path;

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
    pub fn new<P>(path: P, min_filter: u32, mag_filter: u32,wrap_s: u32, wrap_t: u32, generate_mipmap: bool) 
        -> Result<Texture2D, String> where P: AsRef<Path> {

        // Load Image
        let img: DynamicImage = image::open(path).expect("failed to load image");
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

            // Set texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mag_filter as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, min_filter as i32);

            // Set the texture wrapping parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap_s as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap_t as i32);

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
            if generate_mipmap {
                gl::GenerateMipmap(gl::TEXTURE_2D);
            }

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

#[allow(dead_code)]
pub struct Empty;
#[allow(dead_code)]
pub struct Fully;

#[allow(dead_code)]
pub struct TextureBuilder<MinFilter, MagFilter, WrapS, WrapT> {
    min_filter: u32,
    mag_filter: u32,
    wrap_s: u32,
    wrap_t: u32,
    generate_mipmap: bool,
    state: (PhantomData<MinFilter>, PhantomData<MagFilter>, PhantomData<WrapS>, PhantomData<WrapT>),
}

#[allow(dead_code)]
impl TextureBuilder<Empty, Empty, Empty, Empty> {
    pub fn new() -> TextureBuilder<Empty, Empty, Empty, Empty> {
        TextureBuilder {
            min_filter: gl::LINEAR,
            mag_filter: gl::LINEAR,
            wrap_s: gl::CLAMP_TO_EDGE,
            wrap_t: gl::CLAMP_TO_EDGE,
            generate_mipmap: false,
            state: (PhantomData, PhantomData, PhantomData, PhantomData),
        }
    }
}

#[allow(dead_code)]
impl<WrapS, WrapT> TextureBuilder<Empty, Empty, WrapS, WrapT> {
    pub fn filter(self, filter: u32) -> TextureBuilder<Fully, Fully, WrapS, WrapT> {
        TextureBuilder {
            min_filter: filter,
            mag_filter: filter,
            wrap_s: self.wrap_s,
            wrap_t: self.wrap_t,
            generate_mipmap: self.generate_mipmap,
            state: (PhantomData, PhantomData, self.state.2, self.state.3),
        }
    }
}

#[allow(dead_code)]
impl<MinFilter, MagFilter> TextureBuilder<MinFilter, MagFilter, Empty, Empty> {
    pub fn wrap(self, wrap: u32) -> TextureBuilder<MinFilter, MagFilter, Fully, Fully> {
        TextureBuilder {
            min_filter: self.min_filter,
            mag_filter: self.mag_filter,
            wrap_s: wrap,
            wrap_t: wrap,
            generate_mipmap: self.generate_mipmap,
            state: (self.state.0, self.state.1, PhantomData, PhantomData),
        }
    }
}

#[allow(dead_code)]
impl TextureBuilder<Fully, Fully, Fully, Fully> {
    pub fn generate_mipmap(self) -> Self {
        TextureBuilder {
            min_filter: self.min_filter,
            mag_filter: self.mag_filter,
            wrap_s: self.wrap_s,
            wrap_t: self.wrap_t,
            generate_mipmap: true,
            state: (self.state.0, self.state.1, self.state.2, self.state.3),
        }
    }

    pub fn build2d<P>(&self, path: P) -> Result<Texture2D, String> where P: AsRef<Path> {
        Texture2D::new(path, self.min_filter, self.mag_filter, self.wrap_s, self.wrap_t, self.generate_mipmap)
    }
}
