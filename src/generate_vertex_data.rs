pub fn generate_vertex_data(width: u32, height: u32) -> Vec<f32> {
  vec![
    -(width as f32 / 2.0),  (height as f32 / 2.0), 0.0, 0.0,
     (width as f32 / 2.0),  (height as f32 / 2.0), 1.0, 0.0,
     (width as f32 / 2.0), -(height as f32 / 2.0), 1.0, 1.0,
    -(width as f32 / 2.0), -(height as f32 / 2.0), 0.0, 1.0
  ]
}

pub fn generate_animation_vertex_data(width: u32, height: u32, num_frames: u32) -> Vec<f32> {
  vec![
    -(width as f32 / 2.0),  (height as f32 / 2.0), 0.0,                     0.0,
     (width as f32 / 2.0),  (height as f32 / 2.0), 1.0 / num_frames as f32, 0.0,
     (width as f32 / 2.0), -(height as f32 / 2.0), 1.0 / num_frames as f32, 1.0,
    -(width as f32 / 2.0), -(height as f32 / 2.0), 0.0,                     1.0
  ]
}