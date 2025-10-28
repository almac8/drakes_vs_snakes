pub struct VertexBuffer {
  id: gl::types::GLuint
}

impl VertexBuffer {
  pub fn new(vertex_data: Vec<f32>) -> Self {
    let mut id: gl::types::GLuint = 0;
  
    unsafe {
      gl::GenBuffers(1, &mut id);
      gl::BindBuffer(gl::ARRAY_BUFFER, id);
    
      gl::BufferData(
        gl::ARRAY_BUFFER,
        (vertex_data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
        vertex_data.as_ptr() as *const gl::types::GLvoid,
        gl::STATIC_DRAW
      );
    
      gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }
    
    Self {
      id
    }
  }

  pub fn id(&self) -> gl::types::GLuint {
    self.id
  }
}

impl Drop for VertexBuffer {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteBuffers(1, &self.id as *const gl::types::GLuint);
    }
  }
}