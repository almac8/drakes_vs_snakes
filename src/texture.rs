pub struct Texture {
  id: gl::types::GLuint
}

impl Texture {
  pub fn load(file_path: &std::path::Path) -> Result<Self, String> {
    let mut texture_image = image::open(file_path).map_err(| error | error.to_string())?;
    texture_image = texture_image.flipv();
    let texture_image_data = texture_image.as_bytes();
    let mut new_texture_id: gl::types::GLuint = 0;

    unsafe {
      gl::GenTextures(1, &mut new_texture_id);
      gl::BindTexture(gl::TEXTURE_2D, new_texture_id);
      
      gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGBA as gl::types::GLint,
        texture_image.width() as gl::types::GLint,
        texture_image.height() as gl::types::GLint,
        0,
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        texture_image_data.as_ptr() as *const gl::types::GLvoid
      );
      
      gl::GenerateMipmap(gl::TEXTURE_2D);
      gl::BindTexture(gl::TEXTURE_2D, 0);
    }
    
    Ok(
      Self {
        id: new_texture_id
      }
    )
  }

  pub fn id(&self) -> gl::types::GLuint {
    self.id
  }
}

impl Drop for Texture {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteTextures(1, &self.id as *const gl::types::GLuint);
    }
  }
}