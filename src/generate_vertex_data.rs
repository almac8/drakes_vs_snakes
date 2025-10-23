pub fn generate_vertex_data(width: u32, height: u32) -> Vec<f32> {
  vec![
    -(width as f32 / 2.0),  (height as f32 / 2.0), 0.0, 0.0,
     (width as f32 / 2.0),  (height as f32 / 2.0), 1.0, 0.0,
     (width as f32 / 2.0), -(height as f32 / 2.0), 1.0, 1.0,
    -(width as f32 / 2.0), -(height as f32 / 2.0), 0.0, 1.0
  ]
}