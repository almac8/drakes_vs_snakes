use crate::{
  ShadowSprites,
  Camera,
  ShaderProgram,
  render_sprite,
  Vector2
};

pub fn render_shadow(shadow_bits: &[bool; 4], shadow_sprites: &mut ShadowSprites, camera: &Camera, quad_shader_program: &ShaderProgram, location: Vector2) -> Result<(), String> {
  match shadow_bits {
    [false, false, false, false] => {
      shadow_sprites.mut_zero().mut_transform().translate_to(location);
      render_sprite(shadow_sprites.zero(), camera, quad_shader_program)?;
    },
    
    [false, false, false, true] => {
      shadow_sprites.mut_one().mut_transform().translate_to(location);
      shadow_sprites.mut_one().mut_transform().rotate_to(180.0);
      render_sprite(shadow_sprites.one(), camera, quad_shader_program)?;
    },
    
    [false, false, true, false] => {
      shadow_sprites.mut_one().mut_transform().translate_to(location);
      shadow_sprites.mut_one().mut_transform().rotate_to(90.0);
      render_sprite(shadow_sprites.one(), camera, quad_shader_program)?;
    },
    
    [false, false, true, true] => {
      shadow_sprites.mut_three().mut_transform().translate_to(location);
      shadow_sprites.mut_three().mut_transform().rotate_to(90.0);
      render_sprite(shadow_sprites.three(), camera, quad_shader_program)?;
    },
    
    [false, true, false, false] => {
      shadow_sprites.mut_one().mut_transform().translate_to(location);
      shadow_sprites.mut_one().mut_transform().rotate_to(270.0);
      render_sprite(shadow_sprites.one(), camera, quad_shader_program)?;
    },
    
    [false, true, false, true] => {
      shadow_sprites.mut_three().mut_transform().translate_to(location);
      shadow_sprites.mut_three().mut_transform().rotate_to(180.0);
      render_sprite(shadow_sprites.three(), camera, quad_shader_program)?;
    },
    
    [false, true, true, false] => {
      shadow_sprites.mut_two().mut_transform().translate_to(location);
      shadow_sprites.mut_two().mut_transform().rotate_to(90.0);
      render_sprite(shadow_sprites.two(), camera, quad_shader_program)?;
    },
    
    [false, true, true, true] => {
      shadow_sprites.mut_two().mut_transform().translate_to(location);
      shadow_sprites.mut_two().mut_transform().rotate_to(180.0);
      render_sprite(shadow_sprites.four(), camera, quad_shader_program)?;
    },
    
    [true, false, false, false] => {
      shadow_sprites.mut_one().mut_transform().translate_to(location);
      shadow_sprites.mut_one().mut_transform().rotate_to(0.0);
      render_sprite(shadow_sprites.one(), camera, quad_shader_program)?;
    },
    
    [true, false, false, true] => {
      shadow_sprites.mut_two().mut_transform().translate_to(location);
      shadow_sprites.mut_two().mut_transform().rotate_to(0.0);
      render_sprite(shadow_sprites.two(), camera, quad_shader_program)?;
    },
    
    [true, false, true, false] => {
      shadow_sprites.mut_three().mut_transform().translate_to(location);
      shadow_sprites.mut_three().mut_transform().rotate_to(0.0);
      render_sprite(shadow_sprites.three(), camera, quad_shader_program)?;
    },
    
    [true, false, true, true] => {
      shadow_sprites.mut_four().mut_transform().translate_to(location);
      shadow_sprites.mut_four().mut_transform().rotate_to(90.0);
      render_sprite(shadow_sprites.four(), camera, quad_shader_program)?;
    },
    
    [true, true, false, false] => {
      shadow_sprites.mut_three().mut_transform().translate_to(location);
      shadow_sprites.mut_three().mut_transform().rotate_to(270.0);
      render_sprite(shadow_sprites.three(), camera, quad_shader_program)?;
    },
    
    [true, true, false, true] => {
      shadow_sprites.mut_four().mut_transform().translate_to(location);
      shadow_sprites.mut_four().mut_transform().rotate_to(270.0);
      render_sprite(shadow_sprites.four(), camera, quad_shader_program)?;
    },
    
    [true, true, true, false] => {
      shadow_sprites.mut_four().mut_transform().translate_to(location);
      shadow_sprites.mut_four().mut_transform().rotate_to(0.0);
      render_sprite(shadow_sprites.four(), camera, quad_shader_program)?;
    },
    
    [true, true, true, true] => {
      shadow_sprites.mut_five().mut_transform().translate_to(location);
      shadow_sprites.mut_five().mut_transform().rotate_to(0.0);
      render_sprite(shadow_sprites.five(), camera, quad_shader_program)?;
    }
  }
  
  Ok(())
}