use amethyst::ecs::{Component, DenseVecStorage};

pub enum Shape {
    Sphere{ radius: f32, rotation: f64 },
    Rectangle{ height: f32, width: f32, rotation: f64 }
}

impl Component for Shape {
    type Storage = DenseVecStorage<Self>;
}
