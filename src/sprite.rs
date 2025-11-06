use std::path::Path;

use sdl2::{
  pixels::Color,
  ttf::Font
};

use crate::{
  VertexArray,
  Texture,
  Transform,
  generate_vertex_data,
  VertexBuffer,
  ElementBuffer
};

pub struct Sprite {
  vertex_array: VertexArray,
  texture: Texture,
  transform: Transform
}

impl Sprite {
  pub fn print(text: &String, font: &Font, color: &Color) -> Result<Self, String> {
    let texture = Texture::render_text(text, font, color)?;
    let vertex_data = generate_vertex_data(texture.width(), texture.height());

    let element_data = vec![
      0, 1, 2,
      0, 2, 3
    ];

    let vertex_buffer = VertexBuffer::new(vertex_data);
    let element_buffer = ElementBuffer::new(element_data);
    let vertex_array = VertexArray::new(&vertex_buffer, &element_buffer);

    let transform = Transform::new();

    Ok(
      Self {
        vertex_array,
        texture,
        transform
      }
    )
  }

  pub fn load(file_path: &Path) -> Result<Self, String> {
    let texture = Texture::load(file_path)?;
    let vertex_data = generate_vertex_data(texture.width(), texture.height());

    let element_data = vec![
      0, 1, 2,
      0, 2, 3
    ];

    let vertex_buffer = VertexBuffer::new(vertex_data);
    let element_buffer = ElementBuffer::new(element_data);
    let vertex_array = VertexArray::new(&vertex_buffer, &element_buffer);

    let transform = Transform::new();

    Ok(
      Self {
        vertex_array,
        texture,
        transform
      }
    )
  }

  pub fn vertex_array(&self) -> &VertexArray {
    &self.vertex_array
  }
  
  pub fn texture(&self) -> &Texture {
    &self.texture
  }

  pub fn transform(&self) -> &Transform {
    &self.transform
  }

  pub fn mut_transform(&mut self) -> &mut Transform {
    &mut self.transform
  }
}