use std::ffi::CString;

use crate::{
  VertexShader,
  FragmentShader,
  Matrix4,
  flatten_matrix
};

pub struct ShaderProgram {
  id: gl::types::GLuint
}

impl ShaderProgram {
  pub fn new(vertex_shader: VertexShader, fragment_shader: FragmentShader) -> Result<Self, String> {
    let id = unsafe { gl::CreateProgram() };
  
    unsafe {
      gl::AttachShader(id, vertex_shader.id());
      gl::AttachShader(id, fragment_shader.id());

      gl::LinkProgram(id);

      gl::DetachShader(id, vertex_shader.id());
      gl::DetachShader(id, fragment_shader.id());
    }
    
    let mut link_success: gl::types::GLint = 1;
    unsafe { gl::GetProgramiv(id, gl::LINK_STATUS, &mut link_success); }
    
    if link_success == 0 {
      let mut error_string = "Failed to link Shader Program: ".to_owned();

      let mut error_message_length: gl::types::GLint = 0;
      unsafe { gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut error_message_length); }
      
      let mut error_message_buffer: Vec<u8> = Vec::with_capacity(error_message_length as usize);
      error_message_buffer.extend([b' '].iter().cycle().take(error_message_length as usize));
      let error_message_c_string = unsafe { CString::from_vec_unchecked(error_message_buffer) };

      unsafe {
        gl::GetProgramInfoLog(
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
  
  pub fn activate(&self) {
    unsafe {
      gl::UseProgram(self.id);
    }
  }

  fn get_uniform_location(&self, uniform_name: String) -> Result<gl::types::GLint, String> {
    let uniform_name = CString::new(uniform_name)
      .map_err(| error | error.to_string())?;

    let location = unsafe { gl::GetUniformLocation(self.id, uniform_name.as_ptr()) };

    Ok(
      location
    )
  }
  
  fn set_uniform_matrix(&self, matrix_location: gl::types::GLint, matrix: &Matrix4) {
    unsafe {
      gl::UniformMatrix4fv(matrix_location, 1, gl::FALSE, flatten_matrix(matrix).as_ptr());
    }
  }

  pub fn set_uniform_uint(&self, uniform_name: String, value: &u32) -> Result<(), String> {
    let uniform_location = self.get_uniform_location(uniform_name)?;
    
    unsafe {
      gl::Uniform1uiv(uniform_location, 1, value);
    }

    Ok(())
  }

  pub fn set_model_matrix(&self, model_matrix: &Matrix4) -> Result<(), String> {
    let model_matrix_location = self.get_uniform_location("model".to_string())?;
    self.set_uniform_matrix(model_matrix_location, model_matrix);

    Ok(())
  }

  pub fn set_view_matrix(&self, view_matrix: &Matrix4) -> Result<(), String> {
    let view_matrix_location = self.get_uniform_location("view".to_string())?;
    self.set_uniform_matrix(view_matrix_location, view_matrix);

    Ok(())
  }

  pub fn set_projection_matrix(&self, projection_matrix: &Matrix4) -> Result<(), String> {
    let projection_matrix_location = self.get_uniform_location("projection".to_string())?;
    self.set_uniform_matrix(projection_matrix_location, projection_matrix);

    Ok(())
  }
}

impl Drop for ShaderProgram {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteProgram(self.id);
    }
  }
}