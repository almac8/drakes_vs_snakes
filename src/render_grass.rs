use crate::{
  GrassSprites,
  Camera,
  ShaderProgram,
  Vector2,
  render_sprite
};

pub fn render_grass(hint_value: usize, grass_sprites: &mut GrassSprites, camera: &Camera, quad_shader_program: &ShaderProgram, location: Vector2) -> Result<(), String> {
  grass_sprites.mut_zero().mut_transform().translate_to(location);
  render_sprite(grass_sprites.zero(), camera, quad_shader_program)?;
  
  match hint_value {
    1 => {
      grass_sprites.mut_one().mut_transform().translate_to(location);
      render_sprite(grass_sprites.one(), camera, quad_shader_program)?
    },

    2 => {
      grass_sprites.mut_two().mut_transform().translate_to(location);
      render_sprite(grass_sprites.two(), camera, quad_shader_program)?
    },

    3 => {
      grass_sprites.mut_three().mut_transform().translate_to(location);
      render_sprite(grass_sprites.three(), camera, quad_shader_program)?
    },

    4 => {
      grass_sprites.mut_four().mut_transform().translate_to(location);
      render_sprite(grass_sprites.four(), camera, quad_shader_program)?
    },

    5 => {
      grass_sprites.mut_five().mut_transform().translate_to(location);
      render_sprite(grass_sprites.five(), camera, quad_shader_program)?
    },

    6 => {
      grass_sprites.mut_six().mut_transform().translate_to(location);
      render_sprite(grass_sprites.six(), camera, quad_shader_program)?
    },

    7 => {
      grass_sprites.mut_seven().mut_transform().translate_to(location);
      render_sprite(grass_sprites.seven(), camera, quad_shader_program)?
    },

    8 => {
      grass_sprites.mut_eight().mut_transform().translate_to(location);
      render_sprite(grass_sprites.eight(), camera, quad_shader_program)?
    },

    _ => {}
  }
  
  Ok(())
}