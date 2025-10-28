use std::{
  ffi::CString,
  path::Path
};

pub struct VertexShader {
  id: gl::types::GLuint
}

impl VertexShader {
  pub fn load(shader_path: &Path) -> Result<Self, String> {
    let id = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
    
    let shader_source = std::fs::read_to_string(shader_path).map_err(| error | error.to_string())?;
    let shader_source = CString::new(shader_source).map_err(| error | error.to_string())?;
  
    unsafe {
      gl::ShaderSource(id, 1, &shader_source.as_ptr(), std::ptr::null());
      gl::CompileShader(id);
    }

    let mut compile_success: gl::types::GLint = 1;
    unsafe { gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut compile_success); }

    if compile_success == 0 {
      let mut error_string = "Failed to compile shader: ".to_owned();
      let shader_path_string = shader_path.to_str().unwrap();
      error_string += shader_path_string;

      let mut error_message_length: gl::types::GLint = 0;
      unsafe { gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut error_message_length); }

      let mut error_message_buffer: Vec<u8> = Vec::with_capacity(error_message_length as usize);
      error_message_buffer.extend([b' '].iter().cycle().take(error_message_length as usize));
      let error_message_c_string = unsafe { CString::from_vec_unchecked(error_message_buffer) };

      unsafe {
        gl::GetShaderInfoLog(
          id,
          error_message_length,
          std::ptr::null_mut(),
          error_message_c_string.as_ptr() as *mut gl::types::GLchar
        );
      }

      let error_message_str = error_message_c_string.to_str()
        .map_err(| error | error.to_string())?;

      error_string += error_message_str.get(..(error_message_length - 2) as usize).unwrap();

      return Err(error_string);
    }
    
    Ok(
      Self {
        id
      }
    )
  }

  pub fn id(&self) -> gl::types::GLuint {
    self.id
  }
}

impl Drop for VertexShader {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteShader(self.id);
    }
  }
}