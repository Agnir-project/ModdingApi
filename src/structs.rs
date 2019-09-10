use std::ops::AddAssign;

pub trait Join {
    type Output;

    fn join(&self) -> Self::Output;
}

pub trait Component {}

#[derive(Debug)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl AddAssign for Vector3f {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}
