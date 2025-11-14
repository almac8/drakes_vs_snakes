use crate::{
  sprites,
  Camera,
  ShaderProgram,
  render_sprite
};

pub fn render_main_menu(main_menu_sprites: &sprites::MainMenu, emblem_sprites: &sprites::Emblems, camera: &Camera, text_shader_program: &ShaderProgram, quad_shader_program: &ShaderProgram) -> Result<(), String> {
  render_sprite(main_menu_sprites.new_game(), camera, &text_shader_program)?;
  render_sprite(main_menu_sprites.load_game(), camera, &text_shader_program)?;
  render_sprite(main_menu_sprites.high_scores(), camera, &text_shader_program)?;
  render_sprite(main_menu_sprites.settings(), camera, &text_shader_program)?;
  render_sprite(main_menu_sprites.quit(), camera, &text_shader_program)?;
  
  render_sprite(emblem_sprites.snakes(), camera, &quad_shader_program)?;
  render_sprite(emblem_sprites.drakes(), camera, &quad_shader_program)?;

  Ok(())
}