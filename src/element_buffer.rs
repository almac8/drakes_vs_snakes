pub struct ElementBuffer {
  id: gl::types::GLuint
}

impl ElementBuffer {
  pub fn new(element_data: Vec<u32>) -> Self {
    let mut id: gl::types::GLuint = 0;
  
    unsafe {
      gl::GenBuffers(1, &mut id);
      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id);
    
      gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        (element_data.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
        element_data.as_ptr() as *const gl::types::GLvoid,
        gl::STATIC_DRAW
      );
    
      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    }

    Self {
      id
    }
  }

  pub fn id(&self) -> gl::types::GLuint {
    self.id
  }
}

impl Drop for ElementBuffer {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteBuffers(1, &self.id as *const gl::types::GLuint);
    }
  }
}