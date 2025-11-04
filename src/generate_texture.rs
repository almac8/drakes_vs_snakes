pub fn generate_texture(width: gl::types::GLint, height: gl::types::GLint, pixel_data: &[u8]) -> gl::types::GLuint {
  let mut handle: gl::types::GLuint = 0;
  
  unsafe {
    gl::GenTextures(1, &mut handle);
    gl::BindTexture(gl::TEXTURE_2D, handle);
    
    gl::TexImage2D(
      gl::TEXTURE_2D,
      0,
      gl::RGBA as gl::types::GLint,
      width,
      height,
      0,
      gl::RGBA,
      gl::UNSIGNED_BYTE,
      pixel_data.as_ptr() as *const gl::types::GLvoid
    );
    
    gl::GenerateMipmap(gl::TEXTURE_2D);
    gl::BindTexture(gl::TEXTURE_2D, 0);
  }

  handle
}