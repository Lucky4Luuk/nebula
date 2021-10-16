use glam::*;

pub trait Camera {
    fn position(&self) -> Vec3;
    fn get_view_matrix(&self) -> Mat4;
    fn get_proj_matrix(&self) -> Mat4;
}
