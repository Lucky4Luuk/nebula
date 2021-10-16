use glam::*;

pub trait DistanceField {
    fn position(&self) -> Vec3;
}
