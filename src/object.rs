use nalgebra::Vec1 as Vector1;
use nalgebra::Vec2 as Vector2;
use gfx_device_gl::{Resources, Output, CommandBuffer};
use ncollide::point::PointQuery;
use ncollide::shape::Cuboid2;

pub type Vec1 = Vector1<f64>;
pub type Vec2 = Vector2<f64>;

#[derive(Copy, Clone)]
pub struct Transform {
    pos: Vec2,
    scale: Vec2,
    rot: f64
}

#[allow(dead_code)]
impl Transform {
    fn new() -> Transform {
        Transform { pos: Vec2::new(0.0, 0.0), scale: Vec2::new(1.0, 1.0), rot: 0.0 }
    }
    pub fn mov(&mut self, v: Vec2) {
        self.pos = self.pos + v;
    }
    pub fn mov_to(&mut self, v: Vec2) {
        self.pos = v;
    }
    pub fn rot(&mut self, r: f64) {
        self.rot += r;
    }
    pub fn rot_to(&mut self, r: f64) {
        self.rot = r;
    }
    // pub fn fwd(&mut self, d: f64) {
    //     self.pos.x += d * (-self.rot.sin());
    //     self.pos.y += d * self.rot.cos();
    // }
}


pub struct Component {
    trans: Transform,
    sprite: Option<Texture<Resources>>
}

pub trait Object {
	fn update(&mut self, dt: f64);
    fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer<Resources>, Output>, view: math::Matrix2d);
}

pub trait MovableObject {
	fn mov(&mut self, pos: Vec2);
	fn mov_to(&mut self, pos: Vec2);
	fn rot(&mut self, r: f64);
	fn rot_to(&mut self, r: f64);
}

pub trait Target {
	fn destroy(&mut self);
}