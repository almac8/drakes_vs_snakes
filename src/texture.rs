use sdl2::{
  pixels::Color,
  ttf::Font
};

use crate::generate_texture;

pub struct Texture {
  id: gl::types::GLuint,
  width: u32,
  height: u32
}

impl Texture {
  pub fn load(file_path: &std::path::Path) -> Result<Self, String> {
    let mut texture_image = image::open(file_path).map_err(| error | error.to_string())?;
    texture_image = texture_image.flipv();
    let width = texture_image.width();
    let height = texture_image.height();
    let texture_image_data = texture_image.as_bytes();
    let id = generate_texture(width as gl::types::GLint, height as gl::types::GLint, texture_image_data);
    
    Ok(
      Self {
        id,
        width,
        height
      }
    )
  }

  pub fn render_text(text: &String, font: &Font, color: &Color) -> Result<Self, String> {
    let mut image_surface = font
      .render(&text)
      .blended(*color)
      .map_err(| error | error.to_string())?;
    
    image_surface = image_surface.convert_format(sdl2::pixels::PixelFormatEnum::ABGR8888)?;
    let width = image_surface.width();
    let height = image_surface.height();
  
    let image_data = match image_surface.without_lock() {
      Some(data) => data,
      None => return Err("Text rendering error".to_string())
    };

    let id = generate_texture(width as gl::types::GLint, height as gl::types::GLint, image_data);
    
    Ok(
      Self {
        id,
        width,
        height
      }
    )
  }

  pub fn id(&self) -> gl::types::GLuint {
    self.id
  }

  pub fn width(&self) -> u32 {
    self.width
  }

  pub fn height(&self) -> u32 {
    self.height
  }
}

impl Drop for Texture {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteTextures(1, &self.id as *const gl::types::GLuint);
    }
  }
}