use crate::{
  Matrix4,
  Vector4
};

pub fn calculate_projection_matrix(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Matrix4 {
  let rml = right - left;
  let rpl = right + left;
  let tmb = top - bottom;
  let tpb = top + bottom;
  let fmn = far - near;
  let fpn = far + near;

  Matrix4 {
    x: Vector4::new(2.0 / rml, 0.0, 0.0, -(rpl / rml)),
    y: Vector4::new(0.0, 2.0 / tmb, 0.0, -(tpb / tmb)),
    z: Vector4::new(0.0, 0.0, -2.0 / fmn, -(fpn / fmn)),
    w: Vector4::new(0.0, 0.0, 0.0, 1.0)
  }
}