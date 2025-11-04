use crate::{
  Sprite,
  Camera,
  ShaderProgram
};

pub fn render_sprite(sprite: &Sprite, camera: &Camera, shader_program: &ShaderProgram) -> Result<(), String> {
  shader_program.activate();
  shader_program.set_model_matrix(&sprite.transform.matrix())?;
  shader_program.set_view_matrix(&camera.view_matrix())?;
  shader_program.set_projection_matrix(camera.projection_matrix())?;

  unsafe {
    gl::BindVertexArray(sprite.vertex_array.id());
    gl::BindTexture(gl::TEXTURE_2D, sprite.texture.id());
    gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
    gl::BindVertexArray(0);
  }

  Ok(())
}